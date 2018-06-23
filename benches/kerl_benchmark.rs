#[macro_use]
extern crate criterion;
extern crate iota_lib_rs;
extern crate rand;

use criterion::Criterion;
use rand::{thread_rng, Rng};

use iota_lib_rs::pow::kerl::Kerl;
use iota_lib_rs::pow::traits::{ICurl, HASH_LENGTH};

fn basic_kerl(mut trits: [i8; HASH_LENGTH]) {
    let mut kerl = Kerl::default();
    kerl.absorb(&mut trits);
    let mut bytes = vec![0; HASH_LENGTH];
    kerl.squeeze(&mut bytes);
}

fn basic_kerl2(mut trits: [i8; HASH_LENGTH * 100]) {
    let mut kerl = Kerl::default();
    kerl.absorb(&mut trits);
    let mut bytes = vec![0; HASH_LENGTH];
    kerl.squeeze(&mut bytes);
}

fn basic_kerl3(mut trits: [i8; HASH_LENGTH * 1000]) {
    let mut kerl = Kerl::default();
    kerl.absorb(&mut trits);
    let mut bytes = vec![0; HASH_LENGTH];
    kerl.squeeze(&mut bytes);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let vec: Vec<i8> = (0..HASH_LENGTH * 1000)
        .map(|_| rng.gen_range(-1, 2))
        .collect();
    
    let mut trits1 = [0; HASH_LENGTH];
    trits1.copy_from_slice(&vec[..243]);
    c.bench_function("Kerl on 243 trits", move |b| b.iter(|| basic_kerl(trits1)));

    // let mut trits2 = [0; HASH_LENGTH * 100];
    // trits2.copy_from_slice(&vec[..24300]);
    // c.bench_function("Kerl on 24300 trits", move |b| {
    //     b.iter(|| basic_kerl2(trits2))
    // });

    // let mut trits3 = [0; HASH_LENGTH * 1000];
    // trits3.copy_from_slice(&vec[0..243000]);
    // c.bench_function("Kerl on 243000 trits", move |b| {
    //     b.iter(|| basic_kerl3(trits3))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
