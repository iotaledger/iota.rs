#[macro_use]
extern crate criterion;
extern crate iota_lib_rs;
extern crate num_cpus;
extern crate rand;

use criterion::Criterion;

use rand::{thread_rng, Rng};

use iota_lib_rs::pow::curl::Curl;
use iota_lib_rs::pow::pearl_diver::search;

const MIN_WEIGHT_MAGNITUDE: usize = 14;

fn basic_pow(trits: [i8; 8019]) {
   let (_, t) = search(trits, MIN_WEIGHT_MAGNITUDE);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut curl = Curl::default();
    let vec: Vec<i8> = (0..8019).map(|_| rng.gen_range(-1, 2)).collect();
    let mut trits = [0; 8019];
    trits.copy_from_slice(&vec);
    c.bench_function("Run PoW", move |b| b.iter(|| basic_pow(trits)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
