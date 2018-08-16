#[macro_use]
extern crate criterion;
extern crate futures;
extern crate iota_lib_rs;
extern crate num_cpus;
extern crate rand;

use criterion::Criterion;
use rand::{thread_rng, Rng};

use futures::executor::block_on;
use iota_lib_rs::crypto::PearlDiver;

const MIN_WEIGHT_MAGNITUDE: usize = 9;

fn basic_pow(trits: Vec<i8>) {
    let mut pearl_diver = PearlDiver::default();
    block_on(pearl_diver.search(trits, MIN_WEIGHT_MAGNITUDE, None)).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut trits = vec![0; 8019];
    c.bench_function("Run PoW", move |b| {
        b.iter(|| {
            for trit in trits.iter_mut() {
                *trit = rng.gen_range(-1, 2);
            }
            basic_pow(trits.clone());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
