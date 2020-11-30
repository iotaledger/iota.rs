use std::num::NonZeroU64;

use iota::{
  message::prelude::{Address, Message, MessageId, UTXOInput},
  BIP32Path, Seed,
};
use neon::prelude::*;

pub(crate) enum Api {
  // High level APIs
  SendTransfer {
    seed: Seed,
    path: Option<BIP32Path>,
    index: Option<usize>,
    outputs: Vec<(Address, NonZeroU64)>,
  },
  GetUnspentAddress {
    seed: Seed,
    path: Option<BIP32Path>,
    index: Option<usize>,
  },
  FindMessages {
    indexation_keys: Vec<String>,
    message_ids: Vec<MessageId>,
  },
  GetBalance {
    seed: Seed,
    path: Option<BIP32Path>,
    index: Option<usize>,
  },
  // Node APIs
  GetInfo,
  GetTips,
  PostMessage(Message),
  GetMessagesByIndexation(String),
  GetMessage(MessageId),
  GetMessageMetadata(MessageId),
  GetRawMessage(MessageId),
  GetMessageChildren(MessageId),
  GetOutput(UTXOInput),
  FindOutputs {
    outputs: Vec<UTXOInput>,
    addresses: Vec<Address>,
  },
  GetAddressBalance(Address),
  GetAddressOutputs(Address),
  GetMilestone(u64),
  Retry(MessageId),
  Reattach(MessageId),
  Promote(MessageId),
}

pub(crate) struct ClientTask {
  pub client_id: String,
  pub api: Api,
}

impl Task for ClientTask {
  type Output = String;
  type Error = crate::Error;
  type JsEvent = JsString;

  fn perform(&self) -> Result<Self::Output, Self::Error> {
    crate::block_on(crate::convert_async_panics(|| async move {
      let client = crate::get_client(self.client_id.clone());
      let client = client.read().unwrap();
      let res = match &self.api {
        Api::SendTransfer {
          seed,
          path,
          index,
          outputs,
        } => {
          let mut sender = client.send(seed);
          if let Some(path) = path {
            sender = sender.path(path);
          }
          if let Some(index) = index {
            sender = sender.index(*index);
          }
          for output in outputs {
            sender = sender.output(output.0.clone(), output.1);
          }
          let message_id = sender.post().await?;
          serde_json::to_string(&message_id).unwrap()
        }
        Api::GetUnspentAddress { seed, path, index } => {
          let mut getter = client.get_unspent_address(seed);
          if let Some(path) = path {
            getter = getter.path(path);
          }
          if let Some(index) = index {
            getter = getter.index(*index);
          }
          let (address, index) = getter.get().await?;
          serde_json::to_string(&(address, index)).unwrap()
        }
        Api::FindMessages {
          indexation_keys,
          message_ids,
        } => {
          let messages = client
            .find_messages(&indexation_keys[..], &message_ids[..])
            .await?;
          serde_json::to_string(&messages).unwrap()
        }
        Api::GetBalance { seed, path, index } => {
          let mut getter = client.get_balance(seed);
          if let Some(path) = path {
            getter = getter.path(path);
          }
          if let Some(index) = index {
            getter = getter.index(*index);
          }
          let balance = getter.get().await?;
          serde_json::to_string(&balance).unwrap()
        }
        Api::GetInfo => serde_json::to_string(&client.get_info().await?).unwrap(),
        Api::GetTips => {
          let tips = client.get_tips().await?;
          let tips = vec![tips.0, tips.1];
          serde_json::to_string(&tips).unwrap()
        }
        Api::PostMessage(message) => {
          let message_id = client.post_message(message).await?;
          serde_json::to_string(&message_id).unwrap()
        }
        Api::GetMessagesByIndexation(index) => {
          let messages = client.get_message().index(index.as_str()).await?;
          serde_json::to_string(&messages).unwrap()
        }
        Api::GetMessage(id) => {
          let message = client.get_message().data(&id).await?;
          serde_json::to_string(&message).unwrap()
        }
        Api::GetMessageMetadata(id) => {
          let metadata = client.get_message().metadata(&id).await?;
          serde_json::to_string(&metadata).unwrap()
        }
        Api::GetRawMessage(id) => client.get_message().raw(&id).await?,
        Api::GetMessageChildren(id) => {
          let messages = client.get_message().children(&id).await?;
          serde_json::to_string(&messages).unwrap()
        }
        Api::GetOutput(id) => {
          let output = client.get_output(id).await?;
          serde_json::to_string(&output).unwrap()
        }
        Api::FindOutputs { outputs, addresses } => {
          let outputs = client.find_outputs(outputs, addresses).await?;
          serde_json::to_string(&outputs).unwrap()
        }
        Api::GetAddressBalance(address) => {
          let balance = client.get_address().balance(address).await?;
          serde_json::to_string(&balance).unwrap()
        }
        Api::GetAddressOutputs(address) => {
          let output_ids = client.get_address().outputs(address).await?;
          serde_json::to_string(&output_ids).unwrap()
        }
        Api::GetMilestone(index) => {
          let milestone = client.get_milestone(*index).await?;
          serde_json::to_string(&milestone).unwrap()
        }
        Api::Retry(message_id) => {
          let message = client.retry(message_id).await?;
          serde_json::to_string(&message).unwrap()
        }
        Api::Reattach(message_id) => {
          let message = client.reattach(message_id).await?;
          serde_json::to_string(&message).unwrap()
        }
        Api::Promote(message_id) => {
          let message = client.promote(message_id).await?;
          serde_json::to_string(&message).unwrap()
        }
      };
      Ok(res)
    }))
  }

  fn complete(
    self,
    mut cx: TaskContext,
    result: Result<Self::Output, Self::Error>,
  ) -> JsResult<Self::JsEvent> {
    match result {
      Ok(s) => Ok(cx.string(s)),
      Err(e) => cx.throw_error(format!("ClientTask error: {:?}", e)),
    }
  }
}
