#[macro_use]
extern crate criterion;
extern crate iota_lib_rs;
extern crate rand;

use criterion::Criterion;
use rand::{thread_rng, Rng};

use iota_lib_rs::pow::kerl::Kerl;
use iota_lib_rs::pow::traits::{ICurl, HASH_LENGTH};

fn basic_kerl(amount: usize) {
    let mut rng = thread_rng();
    let mut kerl = Kerl::default();
    let mut trits: Vec<i8> = (0..HASH_LENGTH * amount)
        .map(|_| rng.gen_range(-1, 2))
        .collect();
    kerl.absorb(&mut trits);
    let mut bytes = vec![0; HASH_LENGTH];
    kerl.squeeze(&mut bytes);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Kerl on 243 trits", |b| b.iter(|| basic_kerl(1)));
    c.bench_function("Kerl on 24300 trits", |b| b.iter(|| basic_kerl(100)));
    c.bench_function("Kerl on 243000 trits", |b| b.iter(|| basic_kerl(1000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
