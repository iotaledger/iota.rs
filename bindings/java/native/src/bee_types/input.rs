use anyhow::{anyhow, Error};
use getset::{CopyGetters, Getters};
use std::convert::TryFrom;

use bee_rest_api::types::{dtos::OutputDto as RustOutputDto, responses::OutputResponse as RustOutputResponse};

pub struct UTXOInput {}
