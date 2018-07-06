use std::cmp;

fn sum(a: i8, b: i8) -> i8 {
    let s = a + b;
    match s {
        2 => -1,
        -2 => 1,
        _ => s,
    }
}

fn cons(a: i8, b: i8) -> i8 {
    if a == b {
        a
    } else {
        0
    }
}

fn any(a: i8, b: i8) -> i8 {
    let s = a + b;
    if s > 0 {
        1
    } else if s < 0 {
        -1
    } else {
        0
    }
}

fn full_add(a: i8, b: i8, c: i8) -> (i8, i8) {
    let s_a = sum(a, b);
    let c_a = cons(a, b);
    let c_b = cons(s_a, c);
    let c_out = any(c_a, c_b);
    let s_out = sum(s_a, c);

    (s_out, c_out)
}

/// Adds two slices of trits and returns the result
pub fn add(a: &[i8], b: &[i8]) -> Vec<i8> {
    let mut out = vec![0; cmp::max(a.len(), b.len())];
    let mut carry = 0;
    let mut a_i: i8;
    let mut b_i: i8;

    for i in 0..out.len() {
        a_i = if i < a.len() { a[i] } else { 0 };
        b_i = if i < b.len() { b[i] } else { 0 };
        let f_a = full_add(a_i, b_i, carry);
        out[i] = f_a.0;
        carry = f_a.1;
    }

    out
}
