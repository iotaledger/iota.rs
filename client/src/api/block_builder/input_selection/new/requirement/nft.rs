// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::block::output::{NftId, Output};

pub(crate) fn fulfill_nft_requirement(
    nft_id: &NftId,
    available_inputs: &[Output],
    selected_inputs: &[Output],
    outputs: &[Output],
) -> Vec<Output> {
    todo!()
}
