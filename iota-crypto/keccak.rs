#![allow(dead_code)]
//! An implementation of the FIPS-202-defined SHA-3 and SHAKE functions.
//!
//! The `Keccak-f[1600]` permutation is fully unrolled; it's nearly as fast
//! as the Keccak team's optimized permutation.
//!
//! ## Building
//!
//! ```bash
//! cargo build
//! ```
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tiny-keccak = "1.0"
//! ```
//!
//! and this to your crate root:
//!
//! Original implemntation in C:
//! https://github.com/coruus/keccak-tiny
//!
//! Implementor: David Leon Gil
//!
//! Port to rust:
//! Marek Kotewicz (marek.kotewicz@gmail.com)
//!
//! License: CC0, attribution kindly requested. Blame taken too,
//! but not liability.

const RHO: [u32; 24] = [
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43, 62, 18, 39, 61, 20, 44,
];

const PI: [usize; 24] = [
    10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4, 15, 23, 19, 13, 12, 2, 20, 14, 22, 9, 6, 1,
];

const RC: [u64; 24] = [
    1u64,
    0x8082u64,
    0x8000_0000_0000_808au64,
    0x8000_0000_8000_8000u64,
    0x808bu64,
    0x8000_0001u64,
    0x8000_0000_8000_8081u64,
    0x8000_0000_0000_8009u64,
    0x8au64,
    0x88u64,
    0x8000_8009u64,
    0x8000_000au64,
    0x8000_808bu64,
    0x8000_0000_0000_008bu64,
    0x8000_0000_0000_8089u64,
    0x8000_0000_0000_8003u64,
    0x8000_0000_0000_8002u64,
    0x8000_0000_0000_0080u64,
    0x800au64,
    0x8000_0000_8000_000au64,
    0x8000_0000_8000_8081u64,
    0x8000_0000_0000_8080u64,
    0x8000_0001u64,
    0x8000_0000_8000_8008u64,
];

#[allow(unused_assignments)]
/// keccak-f[1600]
pub(crate) fn keccakf(a: &mut [u64; PLEN]) {
    for rc in RC.iter().take(24) {
        let mut array: [u64; 5] = [0; 5];

        // Theta
        unroll! {
            for X in 0..5 {
                unroll! {
                    for Y_COUNT in 0..5 {
                        let y = Y_COUNT * 5;
                        array[X] ^= a[X + y];
                    }
                }
            }
        }

        unroll! {
            for X in 0..5 {
                unroll! {
                    for Y_COUNT in 0..5 {
                        let y = Y_COUNT * 5;
                        a[y + X] ^= array[(X + 4) % 5] ^ array[(X + 1) % 5].rotate_left(1);
                    }
                }
            }
        }

        // Rho and pi
        let mut last = a[1];
        unroll! {
            for X in 0..24 {
                array[0] = a[PI[X]];
                a[PI[X]] = last.rotate_left(RHO[X]);
                last = array[0];
            }
        }

        // Chi
        unroll! {
            for Y_STEP in 0..5 {
                let y = Y_STEP * 5;

                unroll! {
                    for X in 0..5 {
                        array[X] = a[y + X];
                    }
                }

                unroll! {
                    for X in 0..5 {
                        a[y + X] = array[X] ^ ((!array[(X + 1) % 5]) & (array[(X + 2) % 5]));
                    }
                }
            }
        };

        // Iota
        a[0] ^= rc;
    }
}

fn setout(src: &[u8], dst: &mut [u8], len: usize) {
    dst[..len].copy_from_slice(&src[..len]);
}

fn xorin(dst: &mut [u8], src: &[u8]) {
    assert!(dst.len() <= src.len());
    let len = dst.len();
    let mut dst_ptr = dst.as_mut_ptr();
    let mut src_ptr = src.as_ptr();
    for _ in 0..len {
        unsafe {
            *dst_ptr ^= *src_ptr;
            src_ptr = src_ptr.offset(1);
            dst_ptr = dst_ptr.offset(1);
        }
    }
}

/// Total number of lanes.
const PLEN: usize = 25;

/// This structure should be used to create keccak/sha3 hash.
#[derive(Clone, Copy)]
pub(crate) struct Keccak {
    a: [u64; PLEN],
    offset: usize,
    rate: usize,
    delim: u8,
}

macro_rules! impl_constructor {
    ($name:ident, $alias:ident, $bits:expr, $delim:expr) => {
        pub(crate) fn $name() -> Keccak {
            Keccak::new(200 - $bits / 4, $delim)
        }

        pub(crate) fn $alias(data: &[u8], result: &mut [u8]) {
            let mut keccak = Keccak::$name();
            keccak.update(data);
            keccak.finalize(result);
        }
    };
}

macro_rules! impl_global_alias {
    ($alias:ident, $size:expr) => {
        pub(crate) fn $alias(data: &[u8]) -> [u8; $size / 8] {
            let mut result = [0u8; $size / 8];
            Keccak::$alias(data, &mut result);
            result
        }
    };
}

impl_global_alias!(shake128, 128);
impl_global_alias!(shake256, 256);
impl_global_alias!(keccak224, 224);
impl_global_alias!(keccak256, 256);
impl_global_alias!(keccak384, 384);
impl_global_alias!(keccak512, 512);
impl_global_alias!(sha3_224, 224);
impl_global_alias!(sha3_256, 256);
impl_global_alias!(sha3_384, 384);
impl_global_alias!(sha3_512, 512);

impl Keccak {
    pub(crate) fn new(rate: usize, delim: u8) -> Keccak {
        Keccak {
            a: [0; PLEN],
            offset: 0,
            rate,
            delim,
        }
    }

    impl_constructor!(new_shake128, shake128, 128, 0x1f);
    impl_constructor!(new_shake256, shake256, 256, 0x1f);
    impl_constructor!(new_keccak224, keccak224, 224, 0x01);
    impl_constructor!(new_keccak256, keccak256, 256, 0x01);
    impl_constructor!(new_keccak384, keccak384, 384, 0x01);
    impl_constructor!(new_keccak512, keccak512, 512, 0x01);
    impl_constructor!(new_sha3_224, sha3_224, 224, 0x06);
    impl_constructor!(new_sha3_256, sha3_256, 256, 0x06);
    impl_constructor!(new_sha3_384, sha3_384, 384, 0x06);
    impl_constructor!(new_sha3_512, sha3_512, 512, 0x06);

    fn a_bytes(&self) -> &[u8; PLEN * 8] {
        unsafe { &*(&self.a as *const [u64; 25] as *const [u8; 200]) }
    }

    fn a_mut_bytes(&mut self) -> &mut [u8; PLEN * 8] {
        unsafe { &mut *(&mut self.a as *mut [u64; 25] as *mut [u8; 200]) }
    }

    pub(crate) fn update(&mut self, input: &[u8]) {
        self.absorb(input);
    }

    #[inline]
    pub(crate) fn keccakf(&mut self) {
        keccakf(&mut self.a);
    }

    pub(crate) fn finalize(mut self, output: &mut [u8]) {
        self.pad();

        // apply keccakf
        keccakf(&mut self.a);

        // squeeze output
        self.squeeze(output);
    }

    // Absorb input
    pub(crate) fn absorb(&mut self, input: &[u8]) {
        //first foldp
        let mut ip = 0;
        let mut l = input.len();
        let mut rate = self.rate - self.offset;
        let mut offset = self.offset;
        while l >= rate {
            xorin(&mut self.a_mut_bytes()[offset..][..rate], &input[ip..]);
            keccakf(&mut self.a);
            ip += rate;
            l -= rate;
            rate = self.rate;
            offset = 0;
        }

        // Xor in the last block
        xorin(&mut self.a_mut_bytes()[offset..][..l], &input[ip..]);
        self.offset = offset + l;
    }

    pub(crate) fn pad(&mut self) {
        let offset = self.offset;
        let rate = self.rate;
        let delim = self.delim;
        let aa = self.a_mut_bytes();
        aa[offset] ^= delim;
        aa[rate - 1] ^= 0x80;
    }

    pub(crate) fn fill_block(&mut self) {
        self.keccakf();
        self.offset = 0;
    }

    // squeeze output
    pub(crate) fn squeeze(&mut self, output: &mut [u8]) {
        // second foldp
        let mut op = 0;
        let mut l = output.len();
        while l >= self.rate {
            setout(self.a_bytes(), &mut output[op..], self.rate);
            keccakf(&mut self.a);
            op += self.rate;
            l -= self.rate;
        }

        setout(self.a_bytes(), &mut output[op..], l);
    }

    #[inline]
    pub(crate) fn xof(mut self) -> XofReader {
        self.pad();

        keccakf(&mut self.a);

        XofReader {
            keccak: self,
            offset: 0,
        }
    }
}

pub(crate) struct XofReader {
    keccak: Keccak,
    offset: usize,
}

impl XofReader {
    pub(crate) fn squeeze(&mut self, output: &mut [u8]) {
        // second foldp
        let mut op = 0;
        let mut l = output.len();
        let mut rate = self.keccak.rate - self.offset;
        let mut offset = self.offset;
        while l >= rate {
            setout(&self.keccak.a_bytes()[offset..], &mut output[op..], rate);
            self.keccak.keccakf();
            op += rate;
            l -= rate;
            rate = self.keccak.rate;
            offset = 0;
        }

        setout(&self.keccak.a_bytes()[offset..], &mut output[op..], l);
        self.offset = offset + l;
    }
}
