use crate::client::{Client, TopicEvent, TopicHandlerMap};
use crate::Result;
use paho_mqtt::{
  Client as MqttClient, ConnectOptionsBuilder, CreateOptionsBuilder, DisconnectOptionsBuilder,
  MQTT_VERSION_3_1_1,
};
use regex::Regex;

use std::convert::TryFrom;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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

pub(crate) fn get_mqtt_client(client: &mut Client) -> Result<&MqttClient> {
  match client.mqtt_client {
    Some(ref c) => Ok(c),
    None => {
      for node in client.pool.read().unwrap().iter() {
        // node.set_path("mqtt");
        let uri = &format!(
          "{}://{}:{}/mqtt",
          if node.scheme() == "https" {
            "wss"
          } else {
            "ws"
          },
          node.host_str().unwrap(),
          node.port_or_known_default().unwrap()
        );
        let mqtt_options = CreateOptionsBuilder::new()
          .server_uri(uri)
          .client_id(format!("iota.rs-mqtt-client-{}", uri))
          .finalize();
        let mut mqtt_client = MqttClient::new(mqtt_options)?;

        let conn_opts = ConnectOptionsBuilder::new()
          .keep_alive_interval(Duration::from_secs(20))
          .mqtt_version(MQTT_VERSION_3_1_1)
          .clean_session(true)
          .connect_timeout(client.broker_options.timeout)
          .finalize();

        if let Ok(_) = mqtt_client.connect(conn_opts) {
          poll_mqtt(client.mqtt_topic_handlers.clone(), &mut mqtt_client);
          client.mqtt_client = Some(mqtt_client);
          break;
        }
      }
      client
        .mqtt_client
        .as_ref()
        .ok_or_else(|| crate::Error::MqttConnectionNotFound)
    }
  }
}

fn poll_mqtt(mqtt_topic_handlers: Arc<Mutex<TopicHandlerMap>>, client: &mut MqttClient) {
  let receiver = client.start_consuming();
  std::thread::spawn(move || {
    while let Ok(message) = receiver.recv() {
      if let Some(message) = message {
        let topic = message.topic().to_string();
        let mqtt_topic_handlers_guard = mqtt_topic_handlers.lock().unwrap();
        if let Some(handlers) = mqtt_topic_handlers_guard.get(&Topic(topic.clone())) {
          let event = TopicEvent {
            topic,
            payload: message.payload_str().to_string(),
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
}

impl<'a> MqttManager<'a> {
  /// Initializes a new instance of the mqtt subscriber.
  pub fn new(client: &'a mut Client) -> Self {
    Self { client }
  }

  /// Add a new topic to the list.
  pub fn topic(self, topic: Topic) -> MqttTopicManager<'a> {
    MqttTopicManager::new(self.client).topic(topic)
  }

  /// Add a collection of topics to the list.
  pub fn topics(self, topics: Vec<Topic>) -> MqttTopicManager<'a> {
    MqttTopicManager::new(self.client).topics(topics)
  }

  /// Unsubscribes from all subscriptions.
  pub fn unsubscribe(self) -> crate::Result<()> {
    MqttTopicManager::new(self.client).unsubscribe()
  }

  /// Disconnects the broker.
  /// This will clear the stored topic handlers and close the MQTT connection.
  pub fn disconnect(mut self) -> Result<()> {
    let timeout = self.client.broker_options.timeout;
    let client = get_mqtt_client(&mut self.client)?;

    let disconnect_options = DisconnectOptionsBuilder::new().timeout(timeout).finalize();
    client.disconnect(disconnect_options)?;
    self.client.mqtt_client = None;

    {
      let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
      let mut mqtt_topic_handlers = mqtt_topic_handlers.lock().unwrap();
      mqtt_topic_handlers.clear()
    }

    Ok(())
  }
}

/// The MQTT topic manager.
/// Subscribes and unsubscribes from topics.
pub struct MqttTopicManager<'a> {
  client: &'a mut Client,
  topics: Vec<Topic>,
}

impl<'a> MqttTopicManager<'a> {
  /// Initializes a new instance of the mqtt topic manager.
  fn new(client: &'a mut Client) -> Self {
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
    let client = get_mqtt_client(&mut self.client)?;
    let cb = Arc::new(
      Box::new(callback) as Box<dyn Fn(&crate::client::TopicEvent) + Send + Sync + 'static>
    );
    client.subscribe_many(
      &self
        .topics
        .iter()
        .map(|t| t.0.clone())
        .collect::<Vec<String>>(),
      &vec![1; self.topics.len()],
    )?;
    {
      let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
      let mut mqtt_topic_handlers = mqtt_topic_handlers.lock().unwrap();
      for topic in self.topics {
        match mqtt_topic_handlers.get_mut(&topic) {
          Some(handlers) => handlers.push(cb.clone()),
          None => {
            mqtt_topic_handlers.insert(topic, vec![cb.clone()]);
          }
        }
      }
    }
    Ok(())
  }

  /// Unsubscribe from the given topics.
  /// If no topics were provided, the function will unsubscribe from every subscribed topic.
  pub fn unsubscribe(mut self) -> Result<()> {
    let topics = {
      let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
      let mqtt_topic_handlers = mqtt_topic_handlers.lock().unwrap();
      if self.topics.is_empty() {
        mqtt_topic_handlers.keys().cloned().collect()
      } else {
        self.topics
      }
    };

    let client = get_mqtt_client(&mut self.client)?;
    client.unsubscribe_many(&topics.iter().map(|t| t.0.clone()).collect::<Vec<String>>())?;

    let empty_topic_handlers = {
      let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
      let mut mqtt_topic_handlers = mqtt_topic_handlers.lock().unwrap();
      for topic in topics {
        mqtt_topic_handlers.remove(&topic);
      }
      mqtt_topic_handlers.is_empty()
    };

    if self.client.broker_options.automatic_disconnect && empty_topic_handlers {
      MqttManager::new(self.client).disconnect()?;
    }

    Ok(())
  }
}
