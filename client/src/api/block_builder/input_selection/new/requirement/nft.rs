// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    block::output::{NftId, Output},
    error::{Error, Result},
    secret::types::InputSigningData,
};

pub(crate) fn fulfill_nft_requirement(
    nft_id: &NftId,
    available_inputs: &mut Vec<InputSigningData>,
    selected_inputs: &[Output],
    outputs: &[Output],
) -> Result<Vec<InputSigningData>> {
    todo!()
}
