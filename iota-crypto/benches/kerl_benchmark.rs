#[macro_use]
extern crate criterion;
extern crate iota_crypto;
extern crate rand;

use criterion::Criterion;
use iota_constants::HASH_TRINARY_SIZE;
use iota_crypto::{Kerl, Sponge};
use rand::{thread_rng, Rng};

fn basic_kerl(trits: [i8; HASH_TRINARY_SIZE]) {
    let mut kerl = Kerl::default();
    kerl.absorb(&trits).unwrap();
    let mut bytes = vec![0; HASH_TRINARY_SIZE];
    kerl.squeeze(&mut bytes).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut trits = [0; HASH_TRINARY_SIZE];
    for trit in trits.iter_mut() {
        *trit = rng.gen_range(-1, 2);
    }
    c.bench_function("Kerl on 243 trits", move |b| b.iter(|| basic_kerl(trits)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
