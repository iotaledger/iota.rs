use super::constants;
use rand::prelude::*;

/// Generates a cryptographically secure random seed
pub fn generate_new_seed() -> String {
    let mut rng = thread_rng();
    let mut seed = String::new();
    for _i in 0..constants::SEED_LENGTH_MAX {
        let x = rng.gen_range(0, constants::TRYTE_ALPHABET.len());
        seed.push(constants::TRYTE_ALPHABET[x]);
    }
    seed
}

#[cfg(test)]
mod tests {
    use super::generate_new_seed;
    use crate::utils::constants;
    use crate::utils::input_validator;

    #[test]
    fn generate_valid_seed() {
        let seed = generate_new_seed();
        assert!(input_validator::is_address(&seed));
        assert_eq!(seed.len(), constants::SEED_LENGTH_MAX);
    }
}
