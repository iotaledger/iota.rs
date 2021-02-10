// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_bundle_miner::success;

#[test]
pub fn test_success() {
    let chunk: Vec<i8> = vec![
        -13, -13, -13, -13, -13, -7, 0, 11, 9, -7, 9, 7, -3, -12, 11, 12, 6, 10, 6, 0, 1, -4, -6,
        5, 2, 5, 10,
    ];
    let expected = 2.6947539605615567e-9_f64;
    let actual = success(&chunk);
    assert_eq!(true, (expected - actual).abs() < expected * 1e-9);

    let chunk: Vec<i8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let expected = 1.2658620550621852e-13_f64;
    let actual = success(&chunk);
    assert_eq!(true, (expected - actual).abs() < expected * 1e-9);
}
