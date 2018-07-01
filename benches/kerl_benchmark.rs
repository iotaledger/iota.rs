#[macro_use]
extern crate criterion;
extern crate iota_lib_rs;
extern crate rand;

use criterion::Criterion;
use rand::{thread_rng, Rng};

use iota_lib_rs::pow::kerl::Kerl;
use iota_lib_rs::pow::{Sponge, HASH_LENGTH};

fn basic_kerl(trits: [i8; HASH_LENGTH]) {
    let mut kerl = Kerl::default();
    kerl.absorb(&trits);
    let mut bytes = vec![0; HASH_LENGTH];
    kerl.squeeze(&mut bytes);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut trits = [0; HASH_LENGTH];
    for trit in trits.iter_mut() {
        *trit = rng.gen_range(-1, 2);
    }
    c.bench_function("Kerl on 243 trits", move |b| b.iter(|| basic_kerl(trits)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
