#[macro_use]
extern crate criterion;
extern crate iota_lib_rs;
extern crate num_cpus;
extern crate rand;

use criterion::Criterion;

use rand::{thread_rng, Rng};

use std::sync::{Arc, Mutex};

use iota_lib_rs::pow::curl::Curl;
use iota_lib_rs::pow::pearl_diver::PearlDiver;
use iota_lib_rs::pow::traits::*;

const MIN_WEIGHT_MAGNITUDE: usize = 9;

fn basic_pow(amount: usize) {
    let mut rng = thread_rng();
    let mut pearl_diver = PearlDiver::new();
    let mut curl = Curl::default();
    for _ in 0..amount {
        let mut trits: Vec<i8> = (0..8019).map(|_| rng.gen_range(-1, 2)).collect();
        let t = Arc::new(Mutex::new(trits));
        pearl_diver.search(&Arc::clone(&t), MIN_WEIGHT_MAGNITUDE);
        let mut hash_trits = [0; HASH_LENGTH];
        curl.reset();
        curl.absorb(&mut t.lock().unwrap());
        curl.squeeze(&mut hash_trits);
        for j in (HASH_LENGTH - MIN_WEIGHT_MAGNITUDE..HASH_LENGTH - 1).rev() {
            assert_eq!(hash_trits[j], 0);
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Pow 1 time", |b| b.iter(|| basic_pow(1)));
    //c.bench_function("Pow 5 times", |b| b.iter(|| basic_pow(5)));
    //c.bench_function("Pow 10 times", |b| b.iter(|| basic_pow(10)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
