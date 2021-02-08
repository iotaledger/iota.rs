// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub fn success(chunk: &[i8]) -> f64 {
    let p = prod(chunk);
    let sum_1 = sum_half(chunk);
    let sum_2 = sum_square(chunk);
    term_28(chunk, &sum_1, &sum_2, &p)
        + term_29(&sum_1, &sum_2, &p)
        + term_30(&chunk, &sum_1, &sum_2, &p)
}

pub fn prod(chunk: &[i8]) -> Vec<f64> {
    let mut p: f64 = 1.0;
    let mut result: Vec<f64> = chunk
        .iter()
        .rev()
        .map(|&x| {
            p *= (14.0_f64 + f64::from(x)) / 27.0_f64;
            p
        })
        .collect();
    result.reverse();
    result
}

pub fn sum_half(chunk: &[i8]) -> Vec<f64> {
    let mut s: f64 = 0.0;
    let mut result: Vec<f64> = chunk
        .iter()
        .rev()
        .map(|&x| {
            s += (13.0_f64 - f64::from(x)) / 2.0_f64;
            s
        })
        .collect();
    result.reverse();
    result
}

pub fn sum_square(chunk: &[i8]) -> Vec<f64> {
    let mut s: f64 = 0.0;
    let mut result: Vec<f64> = chunk
        .iter()
        .rev()
        .map(|&x| {
            s += (f64::from(x) + 13.0_f64).powf(2.0);
            s
        })
        .collect();
    result.reverse();
    result
}

pub fn term_28(chunk: &[i8], sum_1: &[f64], sum_2: &[f64], p: &[f64]) -> f64 {
    (-2.0_f64 * (f64::from(-chunk[0]) + sum_1[1]).powf(2.0) / sum_2[1]).exp() * p[0]
}

pub fn term_29(sum_1: &[f64], sum_2: &[f64], p: &[f64]) -> f64 {
    let mut s = 0.0_f64;
    for i in 2..14 {
        s += (-2.0f64 * (13.0_f64 * ((i - 1) as f64) + sum_1[i - 1]).powf(2.0) / sum_2[i - 1])
            .exp()
            * p[i - 1];
    }
    s
}

pub fn term_30(chunk: &[i8], sum_1: &[f64], sum_2: &[f64], p: &[f64]) -> f64 {
    let mut s = 0.0_f64;
    for i in 1..14 {
        let c = f64::from(chunk[i - 1]);
        s += (-2.0f64 * (13.0_f64 * ((i - 1) as f64) - c + sum_1[i]).powf(2.0) / sum_2[i]).exp()
            * (13.0_f64 - c)
            / 27.0_f64
            * p[i];
    }
    s
}
