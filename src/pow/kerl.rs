use super::curl::*;
use super::traits::{ICurl, HASH_LENGTH};
use core;
use std::collections::HashSet;
use tiny_keccak::Keccak;
use utils::converter::array_copy;

const BIT_HASH_LENGTH: usize = 384;
const BYTE_HASH_LENGTH: usize = BIT_HASH_LENGTH / 8;

const RADIX: i32 = 3;
const MAX_TRIT_VALUE: i32 = (RADIX - 1) / 2;
const MIN_TRIT_VALUE: i32 = -MAX_TRIT_VALUE;

const BYTE_LENGTH: usize = 48;
const INT_LENGTH: usize = BYTE_LENGTH / 4;

const HALF_3: [u32; 12] = [
    0xa5ce8964, 0x9f007669, 0x1484504f, 0x3ade00d9, 0x0c24486e, 0x50979d57, 0x79a4c702, 0x48bbae36,
    0xa9f6808b, 0xaa06a805, 0xa87fabdf, 0x5e69ebef,
];

#[derive(Clone)]
pub struct Kerl {
    keccak: Keccak,
    state: [i32; HASH_LENGTH],
}

impl Default for Kerl {
    fn default() -> Kerl {
        Kerl {
            keccak: Keccak::new_keccak384(),
            state: [0; HASH_LENGTH],
        }
    }
}

impl ICurl for Kerl {
    fn absorb(&mut self, trits: &[i32]) {
        let len = trits.len();
        self.absorb_offset(trits, 0, len);
    }

    fn absorb_offset(&mut self, trits: &[i32], offset: usize, length: usize) {
        assert!(trits.len() % HASH_LENGTH == 0);
        let mut length = length;
        let mut offset = offset;
        let mut bytes = [0; BYTE_LENGTH];
        while length > 0 {
            trits_to_bytes(&trits[offset..offset + HASH_LENGTH], &mut bytes);
            self.keccak.update(&bytes);
            offset += HASH_LENGTH;
            length -= HASH_LENGTH;
        }
    }

    fn squeeze(&mut self, out: &mut [i32]) {
        let len = out.len();
        self.squeeze_offset(out, 0, len);
    }

    fn squeeze_offset(&mut self, out: &mut [i32], offset: usize, length: usize) {
        assert!(out.len() % HASH_LENGTH == 0);
        let mut offset = offset;
        let mut length = length;
        let mut bytes = [0; BYTE_LENGTH];
        while length > 0 {
            self.keccak.pad();
            self.keccak.fill_block();
            self.keccak.squeeze(&mut bytes);
            self.reset();
            bytes_to_trits(&mut bytes.to_owned(), &mut out[offset..offset + HASH_LENGTH]);
            for b in bytes.iter_mut() {
                *b ^= 0xFF
            }
            self.keccak.update(&bytes);
            offset += HASH_LENGTH;
            length -= HASH_LENGTH;
        }
    }

    fn state(&self) -> &[i32] {
        &self.state
    }
    fn state_mut(&mut self) -> &mut [i32] {
        &mut self.state
    }
}

impl Kerl {
    pub fn reset(&mut self) {
        self.keccak = Keccak::new_keccak384();
    }
}

pub fn trits_to_bytes(trits: &[i32], bytes: &mut [u8]) {
    assert_eq!(trits.len(), HASH_LENGTH);
    assert_eq!(bytes.len(), BYTE_LENGTH);

    // We _know_ that the sizes match.
    // So this is safe enough to do and saves us a few allocations.
    let base: &mut [u32] =
        unsafe { core::slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut u32, 12) };

    base.clone_from_slice(&[0; 12]);

    let mut size = 1;
    let mut all_minus_1 = true;

    for t in trits[0..HASH_LENGTH - 1].iter() {
        if *t != -1 {
            all_minus_1 = false;
            break;
        }
    }

    if all_minus_1 {
        base.clone_from_slice(&HALF_3);
        bigint_not(base);
        bigint_add_base(base, 1_u32);
    } else {
        for t in trits[0..HASH_LENGTH - 1].iter().rev() {
            // multiply by radix
            {
                let sz = size;
                let mut carry: u32 = 0;

                for b in base.iter_mut().take(sz) {
                    let v = u64::from(*b) * (RADIX as u64) + u64::from(carry);
                    let (newcarry, newbase) = ((v >> 32) as u32, v as u32);
                    carry = newcarry;
                    *b = newbase;
                }

                if carry > 0 {
                    base[sz] = carry;
                    size += 1;
                }
            }

            let trit = (t + 1) as u32;
            // addition
            {
                let sz = bigint_add_base(base, trit) as usize;
                if sz > size {
                    size = sz;
                }
            }
        }

        if !is_null(base) {
            if bigint_cmp(&HALF_3, &base) <= 0 {
                // base >= HALF_3
                // just do base - HALF_3
                bigint_sub(base, &HALF_3);
            } else {
                // we don't have a wrapping sub.
                // so let's use some bit magic to achieve it
                let mut tmp = HALF_3;
                bigint_sub(&mut tmp, &base);
                bigint_not(&mut tmp);
                bigint_add_base(&mut tmp, 1);
                base.clone_from_slice(&tmp);
            }
        }
    }
    bytes.reverse();
}

pub fn bytes_to_trits(bytes: &mut [u8], trits: &mut [i32]) {
    assert_eq!(bytes.len(), BYTE_LENGTH);
    assert_eq!(trits.len(), HASH_LENGTH);

    trits[HASH_LENGTH - 1] = 0;

    bytes.reverse();
    // We _know_ that the sizes match.
    // So this is safe enough to do and saves us a few allocations.
    let base: &mut [u32] =
        unsafe { core::slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut u32, 12) };

    if is_null(&base) {
        trits.clone_from_slice(&[0; HASH_LENGTH]);
        return;
    }

    let mut flip_trits = false;

    if base[INT_LENGTH - 1] >> 31 == 0 {
        // positive number
        // we need to add HALF_3 to move it into positvie unsigned space
        bigint_add(base, &HALF_3);
    } else {
        // negative number
        bigint_not(base);
        if bigint_cmp(&base, &HALF_3) > 0 {
            bigint_sub(base, &HALF_3);
            flip_trits = true;
        } else {
            bigint_add_base(base, 1);
            let mut tmp = HALF_3;
            bigint_sub(&mut tmp, &base);
            base.clone_from_slice(&tmp);
        }
    }

    let mut rem;
    for trit in trits.iter_mut().take(HASH_LENGTH - 1) {
        rem = 0;
        for j in (0..INT_LENGTH).rev() {
            let lhs = (u64::from(rem) << 32) | (u64::from(base[j]));
            let rhs = RADIX as u64;
            let q = (lhs / rhs) as u32;
            let r = (lhs % rhs) as u32;

            base[j] = q;
            rem = r;
        }
        *trit = rem as i32 - 1;
    }

    if flip_trits {
        for v in trits.iter_mut() {
            *v = -*v;
        }
    }
}

fn bigint_not(base: &mut [u32]) {
    for i in base.iter_mut() {
        *i = !*i;
    }
}

fn bigint_add_base(base: &mut [u32], rh: u32) -> u32 {
    let mut res = full_add(base[0], rh, false);
    base[0] = res.0;
    let mut j = 0;
    while res.1 {
        res = full_add(base[j], 0, true);
        base[j] = res.0;
        j += 1;
    }
    j as u32
}

fn bigint_add(base: &mut [u32], rh: &[u32]) {
    let mut carry = false;

    for (a, b) in base.iter_mut().zip(rh.iter()) {
        let (v, c) = full_add(*a, *b, carry);
        *a = v;
        carry = c;
    }
}

fn bigint_cmp(lh: &[u32], rh: &[u32]) -> i8 {
    for (a, b) in lh.iter().rev().zip(rh.iter().rev()) {
        if a < b {
            return -1;
        } else if a > b {
            return 1;
        }
    }
    return 0;
}

fn bigint_sub(base: &mut [u32], rh: &[u32]) {
    let mut noborrow = true;
    for (a, b) in base.iter_mut().zip(rh) {
        let (v, c) = full_add(*a, !*b, noborrow);
        *a = v;
        noborrow = c;
    }
    assert!(noborrow);
}

fn is_null(base: &[u32]) -> bool {
    for b in base.iter() {
        if *b != 0 {
            return false;
        }
    }
    return true;
}

fn full_add(ia: u32, ib: u32, carry: bool) -> (u32, bool) {
    let a = u64::from(ia);
    let b = u64::from(ib);

    let mut v = a + b;
    let mut l = v >> 32;
    let mut r = v & 0xFFFFFFFF;

    let carry1 = l != 0;

    if carry {
        v = r + 1;
    }
    l = (v >> 32) & 0xFFFFFFFF;
    r = v & 0xFFFFFFFF;
    let carry2 = l != 0;
    (r as u32, carry1 || carry2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::converter::*;

    #[test]
    fn kerl_one_absorb() {
        let mut trits: Vec<i32> = trits_from_string(
            "GYOMKVTSNHVJNCNFBBAH9AAMXLPLLLROQY99QN9DLSJUHDPBLCFFAIQXZA9BKMBJCYSFHFPXAHDWZFEIZ",
        );
        let mut kerl = Kerl::default();
        kerl.absorb(&trits);
        kerl.squeeze(&mut trits);
        assert_eq!(
            trits_to_string(&trits).unwrap(),
            "OXJCNFHUNAHWDLKKPELTBFUCVW9KLXKOGWERKTJXQMXTKFKNWNNXYD9DMJJABSEIONOSJTTEVKVDQEWTW"
        );
    }

    #[test]
    fn kerl_multi_squeeze_multi_absorb() {
        let trits: Vec<i32> = "G9JYBOMPUXHYHKSNRNMMSSZCSHOFYOYNZRSZMAAYWDYEIMVVOGKPJBVBM9TD\
PULSFUNMTVXRKFIDOHUXXVYDLFSZYZTWQYTE9SPYYWYTXJYQ9IFGYOLZXWZBKWZN9QOOTBQMWMUBLEWUEEASRHRTNIQW\
JQNDWRYLCA"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
.collect();

        let mut kerl = Kerl::default();
        kerl.absorb(&trits);

        let mut out = vec![0; 486];

        kerl.squeeze(&mut out);
        assert_eq!(
            trits_to_string(&out).unwrap(),
            "LUCKQVACOGBFYSPPVSSOXJEKNSQQRQKPZC9NXFSMQNRQCGGUL9OHVVKBDSKEQEBKXRNUJSRXYVHJTXBPD\
             WQGNSCDCBAIRHAQCOWZEBSNHIJIGPZQITIBJQ9LNTDIBTCQ9EUWKHFLGFUVGGUWJONK9GBCDUIMAYMMQX"
        );
    }

    #[test]
    fn kerl_multi_squeeze() {
        let trits: Vec<i32> =
            "9MIDYNHBWMBCXVDEFOFWINXTERALUKYYPPHKP9JJFGJEIUY9MUDVNFZHMMWZUYUSWAIOWEVTHNWMHANBH"
                .chars()
                .flat_map(char_to_trits)
                .cloned()
                .collect();
        let mut kerl = Kerl::default();
        kerl.absorb(&trits);

        let mut out = vec![0; 486];
        kerl.squeeze(&mut out);
        assert_eq!(
            trits_to_string(&out).unwrap(),
            "G9JYBOMPUXHYHKSNRNMMSSZCSHOFYOYNZRSZMAAYWDYEIMVVOGKPJBVBM9TDPULSFUNMTVXRKFIDOHUXX\
             VYDLFSZYZTWQYTE9SPYYWYTXJYQ9IFGYOLZXWZBKWZN9QOOTBQMWMUBLEWUEEASRHRTNIQWJQNDWRYLCA"
        );
    }

}
