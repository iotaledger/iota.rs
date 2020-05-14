use wasm_bindgen::prelude::*;
use iota::crypto::Kerl;
use iota::signing::{IotaSeed, Seed};
use iota::ternary::{T1B1Buf, TryteBuf};
use iota::bundle::TransactionField;
use iota_conversion::Trinary;
use serde::Serialize;

/* #[wasm_bindgen]
extern {
  #[wasm_bindgen(js_namespace = console)]
  pub fn log(s: &str);
  #[wasm_bindgen(js_namespace = console)]
  pub fn error(s: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

macro_rules! console_error {
  ($($t:tt)*) => (error(&format_args!($($t)*).to_string()))
} */

#[wasm_bindgen]
#[derive(Serialize)]
pub struct NewAddress {
    pub index: u64,
    address: String,
}

#[wasm_bindgen]
impl NewAddress {
    #[wasm_bindgen(getter)]
    pub fn address(&self) -> String {
        self.address.clone()
    }
}

#[wasm_bindgen]
pub struct Client {
    client: iota::Client,
}

fn response_to_js_value<T: Serialize>(response: T) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&response)
        .map_err(js_error)
}

fn js_error<T: std::fmt::Debug>(e: T) -> JsValue {
    JsValue::from(format!("{:?}", e))
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(uri: &str) -> Result<Client, JsValue> {
        console_error_panic_hook::set_once();
        let client = Client {
            client: iota::Client::new(uri)
                .map_err(|e| e.to_string())?
        };
        Ok(client)
    }

    #[wasm_bindgen(js_name = "getNodeInfo")]
    pub async fn get_node_info(self) -> Result<JsValue, JsValue> {
        let node_info = self.client.get_node_info()
            .await
            .map_err(|e| JsValue::from(e.to_string()))?;
        let res = response_to_js_value(node_info)?;
        Ok(res)
    }

    #[wasm_bindgen(js_name = "getNewAddress")]
    pub async fn get_new_address(self, seed: String, index: Option<u64>, security: Option<u8>) -> Result<JsValue, JsValue> {
        let encoded_seed = IotaSeed::<Kerl>::from_buf(
            TryteBuf::try_from_str(&seed)
            .map_err(js_error)?
            .as_trits()
            .encode::<T1B1Buf>(),
        )
            .map_err(js_error)?;

        let mut builder = self.client.get_new_address();
        builder = builder.seed(&encoded_seed);

        if let Some(index) = index {
            builder = builder.index(index);
        }
        if let Some(security) = security {
            builder = builder.security(security);
        }

        let (index, address) = builder
            .generate()
            .await
            .map_err(js_error)?;

        let new_address = NewAddress {
            index,
            address: address
                .to_inner()
                .as_i8_slice()
                .trytes()
                .map_err(js_error)?
        };
        let res = response_to_js_value(new_address)?;

        Ok(res)
    }

    /* #[wasm_bindgen(js_name = "addNeighbors")]
    pub async fn add_neighbors(self, uris: JsValue) -> Result<JsValue, JsValue> {
        let uris: Vec<String> = uris.into_serde().map_err(js_error)?;
        let mut builder = self.client.add_neighbors()
            .uris(&uris)
            .map_err(js_error)?;

        let added_neighbords = builder
            .send()
            .await
            .map_err(js_error)?;
        let res = response_to_js_value(added_neighbords)?;

        Ok(res)
    } */
}
