use rand::prelude::*;

use iota_constants;

/// Generates a cryptographically secure random seed
pub fn generate_new_seed() -> String {
    let mut rng = thread_rng();
    let mut seed = String::new();
    for _i in 0..iota_constants::SEED_LENGTH_MAX {
        let x = rng.gen_range(0, iota_constants::TRYTE_ALPHABET.len());
        seed.push(iota_constants::TRYTE_ALPHABET[x]);
    }
    seed
}

#[cfg(test)]
mod tests {
    use iota_constants;

    use iota_validation::input_validator;

    use super::generate_new_seed;

    #[test]
    fn generate_valid_seed() {
        let seed = generate_new_seed();
        assert!(input_validator::is_address(&seed));
        assert_eq!(seed.len(), iota_constants::SEED_LENGTH_MAX);
    }
}
