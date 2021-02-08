// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_crypto::ternary::bigint::{binary_representation::U32Repr, endianness::BigEndian, I384};

// constants
pub const HASH_CHUNK_LEN: usize = 27;
pub const MAX_TRYTE_VALUE: i8 = 13;
pub const MESSAGE_FRAGMENT_LENGTH: usize = 27;

/// I384 big-endian `u32` 3^81
pub const TRITS82_BE_U32: I384<BigEndian, U32Repr> = I384::<BigEndian, U32Repr>::from_array([
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    1_301_861_838,
    2_705_975_348,
    3_065_973_865,
    3_580_722_371,
]);
