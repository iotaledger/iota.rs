#[macro_use]
extern crate criterion;
extern crate iota_lib_rs;
extern crate num_cpus;
extern crate rand;

use criterion::Criterion;
use rand::{thread_rng, Rng};

use iota_lib_rs::crypto::PearlDiver;

const MIN_WEIGHT_MAGNITUDE: usize = 14;

fn basic_pow(mut trits: [i8; 8019]) {
    let mut pearl_diver = PearlDiver::default();
    pearl_diver
        .search(&mut trits, MIN_WEIGHT_MAGNITUDE, None)
        .unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut trits = [0; 8019];
    for trit in trits.iter_mut() {
        *trit = rng.gen_range(-1, 2);
    }
    c.bench_function("Run PoW", move |b| b.iter(|| basic_pow(trits)));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(5);
    targets = criterion_benchmark
}
criterion_main!(benches);
