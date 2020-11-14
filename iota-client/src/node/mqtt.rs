use crate::client::{Client, TopicEvent, TopicHandlerMap};
use crate::Result;
use regex::Regex;
use rumqttc::{
  Client as MqttClient, Connection as MqttConnection, Event, Incoming, MqttOptions, QoS,
};

use std::convert::TryFrom;
use std::sync::{Arc, Mutex};

macro_rules! lazy_static {
  ($init:expr => $type:ty) => {{
    static mut VALUE: Option<$type> = None;
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| unsafe { VALUE = Some($init) });
    unsafe { VALUE.as_ref() }.expect("failed to get lazy static value")
  }};
}

/// A topic.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Topic(String);

impl TryFrom<&str> for Topic {
  type Error = crate::Error;

  fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
    Self::new(value)
  }
}

impl Topic {
  /// Creates a new topic and checks if it's valid.
  pub fn new<S: Into<String>>(name: S) -> Result<Self> {
    let valid_topics = lazy_static!(
      ["milestones/latest", "milestones/solid", "messages", "messages/referenced"].to_vec() => Vec<&str>
    );
    let regexes = lazy_static!(
      [
        Regex::new(r"messages/([A-Fa-f0-9]{64})/metadata").unwrap(),
        Regex::new(r"outputs/([A-Fa-f0-9]{64})(\d{4})").unwrap(),
        Regex::new("addresses/([A-Fa-f0-9]{64})/outputs").unwrap(),
        Regex::new(r"messages/indexation/(\.)").unwrap()
      ].to_vec() => Vec<Regex>
    );
    let name = name.into();
    if valid_topics.iter().any(|valid| valid == &name)
      || regexes.iter().any(|re| re.is_match(&name))
    {
      let topic = Self(name);
      Ok(topic)
    } else {
      Err(crate::Error::InvalidMqttTopic(name))
    }
  }
}

pub(crate) fn get_mqtt_client(client: &mut Client) -> Result<MqttClient> {
  match client.mqtt_client.clone() {
    Some(c) => Ok(c),
    None => {
      for node in client.pool.read().unwrap().iter() {
        let host = node.host_str().unwrap();
        let mqttoptions = MqttOptions::new(host, host, 1883);
        let (_, mut connection) = MqttClient::new(mqttoptions.clone(), 10);
        // poll the event loop until we find a ConnAck event,
        // which means that the mqtt client is ready to be used on this host
        // if the event loop returns an error, we check the next node
        // note that we need to do this on a separate thread to prevent tokio runtime panics
        let got_ack = std::thread::spawn(move || {
          let mut got_ack = false;
          for event in connection.iter() {
            match event {
              Ok(event) => {
                if let Event::Incoming(incoming) = event {
                  if let Incoming::ConnAck(_) = incoming {
                    got_ack = true;
                    break;
                  }
                }
              }
              Err(_) => break,
            }
          }
          got_ack
        })
        .join()
        .unwrap();

        // if we found a valid mqtt connection, loop it on a separate thread
        if got_ack {
          let (mqtt_client, connection) = MqttClient::new(mqttoptions, 10);
          client.mqtt_client = Some(mqtt_client);
          poll_mqtt(client.mqtt_topic_handlers.clone(), connection);
        }
      }
      client
        .mqtt_client
        .clone()
        .ok_or_else(|| crate::Error::NodePoolEmpty)
    }
  }
}

fn poll_mqtt(mqtt_topic_handlers: Arc<Mutex<TopicHandlerMap>>, connection: MqttConnection) {
  let mut connection = connection;
  std::thread::spawn(move || {
    for event in connection.iter() {
      if let Ok(Event::Incoming(Incoming::Publish(p))) = event {
        let event_topic = p.topic;
        let mqtt_topic_handlers_guard = mqtt_topic_handlers.lock().unwrap();
        if let Some(handlers) = mqtt_topic_handlers_guard.get(&Topic(event_topic.clone())) {
          let event_payload = String::from_utf8_lossy(&*p.payload).to_string();
          let event = TopicEvent {
            topic: event_topic,
            payload: event_payload,
          };
          for handler in handlers {
            handler(&event)
          }
        }
      }
    }
  });
}

/// MQTT subscriber.
pub struct MqttManager<'a> {
  client: &'a mut Client,
  topics: Vec<Topic>,
}

impl<'a> MqttManager<'a> {
  /// Initializes a new instance of the mqtt subscriber.
  pub fn new(client: &'a mut Client) -> Self {
    Self {
      client,
      topics: vec![],
    }
  }

  /// Add a new topic to the list.
  pub fn topic(mut self, topic: Topic) -> Self {
    self.topics.push(topic.into());
    self
  }

  /// Add a collection of topics to the list.
  pub fn topics(mut self, topics: Vec<Topic>) -> Self {
    self.topics.extend(topics.into_iter());
    self
  }

  /// Subscribe to the given topics with the callback.
  pub fn subscribe<C: Fn(&crate::client::TopicEvent) + Send + Sync + 'static>(
    mut self,
    callback: C,
  ) -> Result<()> {
    let mut client = get_mqtt_client(&mut self.client)?;
    let cb = Arc::new(
      Box::new(callback) as Box<dyn Fn(&crate::client::TopicEvent) + Send + Sync + 'static>
    );
    let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
    let mut mqtt_topic_handlers = mqtt_topic_handlers.lock().unwrap();
    for topic in self.topics {
      client.subscribe(&topic.0, QoS::AtLeastOnce)?;
      match mqtt_topic_handlers.get_mut(&topic) {
        Some(handlers) => handlers.push(cb.clone()),
        None => {
          mqtt_topic_handlers.insert(topic, vec![cb.clone()]);
        }
      }
    }
    Ok(())
  }

  /// Unsubscribe from the given topics.
  /// If no topics were provided, the function will unsubscribe from every subscribed topic.
  pub fn unsubscribe(mut self) -> Result<()> {
    let mut client = get_mqtt_client(&mut self.client)?;
    let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
    let mut mqtt_topic_handlers = mqtt_topic_handlers.lock().unwrap();

    let topics = if self.topics.is_empty() {
      mqtt_topic_handlers.keys().cloned().collect()
    } else {
      self.topics
    };
    for topic in topics {
      client.unsubscribe(&topic.0)?;
      mqtt_topic_handlers.remove(&topic);
    }
    Ok(())
  }
}
