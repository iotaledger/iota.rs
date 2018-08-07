#[macro_use]
extern crate criterion;
extern crate iota_lib_rs;
extern crate num_cpus;
extern crate rand;
extern crate futures;

use criterion::Criterion;
use rand::{thread_rng, Rng};

use iota_lib_rs::crypto::PearlDiver;
use futures::executor::block_on;

const MIN_WEIGHT_MAGNITUDE: usize = 9;

fn basic_pow(trits: [i8; 8019]) {
    let pearl_diver = PearlDiver::default();
    block_on(pearl_diver
        .search(trits.to_vec(), MIN_WEIGHT_MAGNITUDE, None))
        .unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut trits = [0; 8019];
    c.bench_function("Run PoW", move |b| {
        b.iter(|| {
            for trit in trits.iter_mut() {
                *trit = rng.gen_range(-1, 2);
            }
            basic_pow(trits);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
