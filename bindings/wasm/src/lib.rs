use wasm_bindgen::prelude::*;
use iota::crypto::Kerl;
use iota::signing::{IotaSeed, Seed};
use iota::ternary::{T1B1Buf, TryteBuf};
use iota::bundle::{Address, TransactionField, Tag};
use iota::client::response::{Transfer, TransactionDef};
use iota_conversion::Trinary;
use iota_bundle_preview::Hash;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
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
}

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
#[derive(Deserialize)]
pub struct NewTransfer {
    value: u64
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct SentTransaction {
    #[serde(rename = "isTail")]
    is_tail: bool
}

fn response_to_js_value<T: Serialize>(response: T) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&response)
        .map_err(js_error)
}

fn js_error<T: std::fmt::Debug>(e: T) -> JsValue {
    JsValue::from(format!("{:?}", e))
}

fn create_hash_from_string(bytes: String) -> Result<Hash, JsValue> {
    let hash = Hash::from_inner_unchecked(
        TryteBuf::try_from_str(&bytes)
            .map_err(js_error)?
            .as_trits()
            .encode(),
    );
    Ok(hash)
}

fn create_hash(bytes: JsValue) -> Result<Hash, JsValue> {
    let bytes: String = bytes.into_serde().map_err(js_error)?;
    create_hash_from_string(bytes)
}

fn create_hash_array(bytes_vec: JsValue) -> Result<Vec<Hash>, JsValue> {
    let hashes_vecs: Vec<String> = bytes_vec.into_serde().map_err(js_error)?;
    let mut hashes = Vec::new();
    for hash in hashes_vecs {
        hashes.push(create_hash_from_string(hash)?);
    }
    Ok(hashes)
}

fn create_address(address: String) -> Result<Address, JsValue> {
    let address = Address::from_inner_unchecked(
        TryteBuf::try_from_str(&address)
        .map_err(js_error)?
        .as_trits()
        .encode(),
    );
    Ok(address)
}

fn create_addresses(addresses: JsValue) -> Result<Vec<Address>, JsValue> {
    let addresses_vec: Vec<String> = addresses.into_serde().map_err(js_error)?;
    let mut addresses = Vec::new();
    for address in addresses_vec {
        let address = create_address(address)?;
        addresses.push(address);
    }
    Ok(addresses)
}

#[wasm_bindgen(js_name = "addNode")]
pub fn add_node(uri: &str) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    iota::Client::add_node(uri)
        .map_err(js_error)?;
    Ok(())
}

#[wasm_bindgen(js_name = "addNeighbors")]
pub async fn add_neighbors(uris: JsValue) -> Result<JsValue, JsValue> {
    let uris: Vec<String> = uris.into_serde().map_err(js_error)?;
    let added_neighbors = iota::Client::add_neighbors(uris)
        .await
        .map_err(js_error)?;

    let res = response_to_js_value(added_neighbors)?;

    Ok(res)
}

#[wasm_bindgen(js_name = "attachToTangle")]
pub async fn attach_to_tangle(
    trunk_transaction_hash_bytes: JsValue,
    branch_transaction_hash_bytes: JsValue,
    min_weight_magnitude: Option<u8>,
    transactions_trytes: JsValue,
) -> Result<JsValue, JsValue> {
    let mut builder = iota::Client::attach_to_tangle();

    if trunk_transaction_hash_bytes.is_truthy() {
        let hash = create_hash(trunk_transaction_hash_bytes)?;
        builder = builder.trunk_transaction(&hash);
    }

    if branch_transaction_hash_bytes.is_truthy() {
        let hash = create_hash(branch_transaction_hash_bytes)?;
        builder = builder.branch_transaction(&hash);
    }

    if transactions_trytes.is_truthy() {
        // let transactions_trytes: Vec<Transaction> = transactions_trytes.into_serde().map_err(js_error)?;
    }

    if let Some(min_weight_magnitude) = min_weight_magnitude {
        builder = builder.min_weight_magnitude(min_weight_magnitude);
    }

    let attach_response = builder
        .send()
        .await
        .map_err(js_error)?;

    let response = response_to_js_value(&attach_response)?;

    Ok(response)
}

#[wasm_bindgen(js_name = "broadcastBundle")]
pub async fn broadcast_bundle(tail_transaction_hash_bytes: JsValue) -> Result<JsValue, JsValue> {
    let tail_transaction_hash = create_hash(tail_transaction_hash_bytes)?;

    let broadcast_response = iota::Client::broadcast_bundle(&tail_transaction_hash)
        .await
        .map_err(js_error)?;

    let response: Vec<TransactionDef> = broadcast_response.iter().map(|tx| TransactionDef::from(tx)).collect();
    let response = response_to_js_value(&response)?;

    Ok(response)
}

#[wasm_bindgen(js_name = "checkConsistency")]
pub async fn check_consistency(tails: JsValue) -> Result<JsValue, JsValue> {
    let tails = create_hash_array(tails)?;

    let consistency_response = iota::Client::check_consistency(&tails)
        .await
        .map_err(js_error)?;

    let response = response_to_js_value(consistency_response)?;

    Ok(response)
}

#[wasm_bindgen(js_name = "findTransactions")]
pub async fn find_transactions(
    bundle_hashes_bytes: JsValue,
    tags: JsValue,
    aprovees_hashes_bytes: JsValue,
    addresses: JsValue,
) -> Result<JsValue, JsValue> {
    let mut builder = iota::Client::find_transactions();

    if bundle_hashes_bytes.is_truthy() {
        let bundle_hashes = create_hash_array(bundle_hashes_bytes)?;
        builder = builder.bundles(&bundle_hashes);
    }

    if aprovees_hashes_bytes.is_truthy() {
        let approvees_hashes = create_hash_array(aprovees_hashes_bytes)?;
        builder = builder.approvees(&approvees_hashes);
    }

    if tags.is_truthy() {
        let tags_vec: Vec<String> = tags.into_serde().map_err(js_error)?;
        let mut formatted_tags = Vec::new();
        for tag in tags_vec {
            let formatted_tag = TryteBuf::try_from_str(&tag)
                .map_err(js_error)?
                .as_trits()
                .encode::<T1B1Buf>();
            formatted_tags.push(Tag::from_inner_unchecked(formatted_tag));
        }
        builder = builder.tags(&formatted_tags[..]);
    }

    if addresses.is_truthy() {
        let addresses = create_addresses(addresses)?;
        builder = builder.addresses(&addresses);
    }

    let find_response = builder
        .send()
        .await
        .map_err(js_error)?;

    let response = response_to_js_value(find_response)?;

    Ok(response)
}

#[wasm_bindgen(js_name = "getBalances")]
pub async fn get_balances(addresses: JsValue, threshold: Option<u8>, tips_hashes_bytes: JsValue) -> Result<JsValue, JsValue> {
    let mut builder = iota::Client::get_balances();

    if addresses.is_truthy() {
        let addresses = create_addresses(addresses)?;
        builder = builder.addresses(&addresses);
    }

    if let Some(threshold) = threshold {
        builder = builder.threshold(threshold);
    }

    if tips_hashes_bytes.is_truthy() {
        let tips_hashes = create_hash_array(tips_hashes_bytes)?;
        builder = builder.tips(&tips_hashes);
    }

    let balance_response = builder
        .send()
        .await
        .map_err(js_error)?;

    let response = response_to_js_value(balance_response)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "getBundle")]
pub async fn get_bundle(hash_bytes: JsValue) -> Result<JsValue, JsValue> {
    let hash = create_hash(hash_bytes)?;
    let bundle = iota::Client::get_bundle(&hash)
        .await
        .map_err(js_error)?;

    let response: Vec<TransactionDef> = bundle.iter().map(|tx| TransactionDef::from(tx)).collect();
    let response = response_to_js_value(response)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "getInclusionStates")]
pub async fn get_inclusion_states(transaction_hashes_bytes: JsValue, tips_hashes_bytes: JsValue) -> Result<JsValue, JsValue> {
    let mut builder = iota::Client::get_inclusion_states();

    if transaction_hashes_bytes.is_truthy() {
        let transaction_hashes = create_hash_array(transaction_hashes_bytes)?;
        builder = builder.transactions(&transaction_hashes);
    }

    if tips_hashes_bytes.is_truthy() {
        let tips_hashes = create_hash_array(tips_hashes_bytes)?;
        builder = builder.tips(&tips_hashes);
    }

    let inclusion_states = builder
        .send()
        .await
        .map_err(js_error)?;

    let response = response_to_js_value(inclusion_states)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "getInputs")]
pub async fn get_inputs(seed: String, index: Option<u64>, security: Option<u8>, threshold: Option<u64>) -> Result<JsValue, JsValue> {
    let encoded_seed = IotaSeed::<Kerl>::from_buf(
        TryteBuf::try_from_str(&seed)
        .map_err(js_error)?
        .as_trits()
        .encode::<T1B1Buf>(),
    )
        .map_err(js_error)?;
    let mut builder = iota::Client::get_inputs(&encoded_seed);

    if let Some(index) = index {
        builder = builder.index(index);
    }

    if let Some(security) = security {
        builder = builder.security(security);
    }

    if let Some(threshold) = threshold {
        builder = builder.threshold(threshold);
    }

    let inputs = builder
        .generate()
        .await
        .map_err(js_error)?;

    let response = response_to_js_value(inputs)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "getLatestInclusion")]
pub async fn get_latest_inclusion(transaction_hashes: JsValue) -> Result<JsValue, JsValue> {
    let transaction_hashes = create_hash_array(transaction_hashes)?;
    let latest_inclusion = iota::Client::get_latest_inclusion(&transaction_hashes)
        .await
        .map_err(js_error)?;

    let response = response_to_js_value(latest_inclusion)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "getLatestSolidSubtangleMilestone")]
pub async fn get_latest_solid_subtangle_milestone() -> Result<JsValue, JsValue> {
    let hash = iota::Client::get_latest_solid_subtangle_milestone()
        .await
        .map_err(js_error)?;
    let response = response_to_js_value(hash.as_bytes())?;
    Ok(response)
}

#[wasm_bindgen(js_name = "getMissingTransactions")]
pub async fn get_missing_transactions() -> Result<JsValue, JsValue> {
    let missing_transactions = iota::Client::get_missing_transactions()
        .await
        .map_err(js_error)?;

    let response = response_to_js_value(missing_transactions)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "getNeighbors")]
pub async fn get_neighbors() -> Result<JsValue, JsValue> {
    let neighbors = iota::Client::get_neighbors()
        .await
        .map_err(js_error)?;

    let response = response_to_js_value(neighbors)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "getNewAddress")]
pub async fn get_new_address(seed: String, index: Option<u64>, security: Option<u8>) -> Result<JsValue, JsValue> {
    let encoded_seed = IotaSeed::<Kerl>::from_buf(
        TryteBuf::try_from_str(&seed)
        .map_err(js_error)?
        .as_trits()
        .encode::<T1B1Buf>(),
    )
        .map_err(js_error)?;

    let mut builder = iota::Client::get_new_address(&encoded_seed);

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

#[wasm_bindgen(js_name = "getNodeApiConfiguration")]
pub async fn get_node_api_configuration() -> Result<JsValue, JsValue> {
    let node_api_configuration = iota::Client::get_node_api_configuration()
        .await
        .map_err(js_error)?;

    let response = response_to_js_value(node_api_configuration)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "getNodeInfo")]
pub async fn get_node_info() -> Result<JsValue, JsValue> {
    let node_info = iota::Client::get_node_info()
        .await
        .map_err(js_error)?;
    let response = response_to_js_value(node_info)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "getTips")]
pub async fn get_tips() -> Result<JsValue, JsValue> {
    let tips = iota::Client::get_tips()
        .await
        .map_err(js_error)?;

    let response = response_to_js_value(tips)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "getTransactionsToApprove")]
pub async fn get_transactions_to_approve(
    depth: Option<u8>,
    reference_hash_bytes: JsValue,
) -> Result<JsValue, JsValue> {
    let mut builder = iota::Client::get_transactions_to_approve();

    if let Some(depth) = depth {
        builder = builder.depth(depth);
    }

    if reference_hash_bytes.is_truthy() {
        let reference_hash = create_hash(reference_hash_bytes)?;
        builder = builder.reference(&reference_hash);
    }

    let transactions_to_approve = builder
        .send()
        .await
        .map_err(js_error)?;

    let response = response_to_js_value(transactions_to_approve)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "getTrytes")]
pub async fn get_trytes(hash_bytes: JsValue) -> Result<JsValue, JsValue> {
    let hashes = create_hash_array(hash_bytes)?;
    let trytes = iota::Client::get_trytes(&hashes)
        .await
        .map_err(js_error)?;
    
    let response = response_to_js_value(trytes)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "interruptAttachingToTangle")]
pub async fn interrupt_attaching_to_tangle() -> Result<(), JsValue> {
    iota::Client::interrupt_attaching_to_tangle()
        .await
        .map_err(js_error)?;
    Ok(())
}

#[wasm_bindgen(js_name = "isAddressUsed")]
pub async fn is_address_used(address: String) -> Result<bool, JsValue> {
    let address = create_address(address)?;
    let is_address_used = iota::Client::is_address_used(&address)
            .await
            .map_err(js_error)?;

    Ok(is_address_used)
}

#[wasm_bindgen(js_name = "isPromotable")]
pub async fn is_promotable(tail_hash: JsValue) -> Result<bool, JsValue> {
    let tail_hash = create_hash(tail_hash)?;

    let is_promotable = iota::Client::is_promotable(&tail_hash)
        .await
        .map_err(js_error)?;

    Ok(is_promotable)
}

#[wasm_bindgen(js_name = "prepareTransfers")]
pub async fn prepare_transfers(seed: String) -> Result<JsValue, JsValue> {
    let encoded_seed = IotaSeed::<Kerl>::from_buf(
        TryteBuf::try_from_str(&seed)
        .map_err(js_error)?
        .as_trits()
        .encode::<T1B1Buf>(),
    )
        .map_err(js_error)?;

    // TODO
    Ok(JsValue::from(""))
}

#[wasm_bindgen(js_name = "removeNeighbors")]
pub async fn remove_neighbors(uris: JsValue) -> Result<JsValue, JsValue> {
    let uris: Vec<String> = uris.into_serde().map_err(js_error)?;

    let remove_response = iota::Client::remove_neighbors(uris)
        .await
        .map_err(js_error)?;

    let response = response_to_js_value(remove_response)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "replayBundle")]
pub async fn replay_bundle(
    hash_bytes: JsValue,
    depth: Option<u8>,
    min_weight_magnitude: Option<u8>,
    trytes: JsValue,
    reference_hash_bytes: JsValue,
) -> Result<JsValue, JsValue> {
    let hash = create_hash(hash_bytes)?;

    let mut builder = iota::Client::replay_bundle(&hash)
        .await
        .map_err(js_error)?;

    if reference_hash_bytes.is_truthy() {
        let reference_hash = create_hash(reference_hash_bytes)?;
        builder = builder.reference(reference_hash);
    }

    if let Some(depth) = depth {
        builder = builder.depth(depth);
    }

    if let Some(min_weight_magnitude) = min_weight_magnitude {
        builder = builder.min_weight_magnitude(min_weight_magnitude);
    }

    let replay_response = builder
        .send()
        .await
        .map_err(js_error)?;

    let response: Vec<TransactionDef> = replay_response.iter().map(|tx| TransactionDef::from(tx)).collect();
    let response = response_to_js_value(response)?;
    Ok(response)
}

#[wasm_bindgen(js_name = "sendTransfers")]
pub async fn send_transfers(seed: String, transfers: JsValue, min_weight_magnitude: Option<u8>) -> Result<JsValue, JsValue> {
    let encoded_seed = IotaSeed::<Kerl>::from_buf(
        TryteBuf::try_from_str(&seed)
        .map_err(js_error)?
        .as_trits()
        .encode::<T1B1Buf>(),
    )
        .map_err(js_error)?;

    let address = Address::from_inner_unchecked(
        TryteBuf::try_from_str(&seed)
        .map_err(js_error)?
        .as_trits()
        .encode(),
    );

    let js_transfers: Vec<NewTransfer> = transfers.into_serde().map_err(js_error)?;
    let transfers = js_transfers.iter().map(|transfer| Transfer {
        address: address.clone(),
        value: transfer.value,
        message: None,
        tag: None,
    }).collect();

    let mut builder = iota::Client::send_transfers(&encoded_seed)
        .transfers(transfers);

    if let Some(min_weight_magnitude) = min_weight_magnitude {
        builder = builder.min_weight_magnitude(min_weight_magnitude);
    }

    let transactions = builder
        .send()
        .await
        .map_err(js_error)?;

    let response: Vec<SentTransaction> = transactions.iter().map(|transaction| SentTransaction {
        is_tail: transaction.is_tail()
    }).collect();
    
    let res = response_to_js_value(response)?;
    Ok(res)
}

#[wasm_bindgen(js_name = "traverseBundle")]
pub async fn traverse_bundle(hash_bytes: JsValue) -> Result<JsValue, JsValue> {
    let hash = create_hash(hash_bytes)?;

    let transactions = iota::Client::traverse_bundle(&hash)
        .await
        .map_err(js_error)?;

    let response: Vec<TransactionDef> = transactions.iter().map(|tx| TransactionDef::from(tx)).collect();
    let response = response_to_js_value(&response)?;

    Ok(response)
}