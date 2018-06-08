use utils::constants;
use rand::prelude::*;

pub fn generate_new_seed() -> String {
    let mut rng = thread_rng();
    let mut seed = String::new();
    for i in 0..constants::SEED_LENGTH_MAX {
        let x = rng.gen_range(0, constants::TRYTE_ALPHABET.len());
        seed.push(constants::TRYTE_ALPHABET[x]);
    }
   seed
}