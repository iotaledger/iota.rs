// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// gets the BIP32 account path from a given account_index/address_internal/address_index
#[macro_export]
macro_rules! account_path {
    ($account_index:expr) => {
        format!("m/44'/4218'/{}'", $account_index)
    };
}

use crate::{Error, Result};

use bee_crypto::ternary::sponge::{Kerl, Sponge};
use bee_message::prelude::{Address, Ed25519Address};
use bee_signing_ext::{
    binary::{BIP32Path, Ed25519PrivateKey, Ed25519Seed},
    Seed,
};
use bee_ternary::{b1t6, T1B1Buf, T3B1Buf, Trits, TryteBuf};
use bee_transaction::bundled::{Address as TryteAddress, BundledTransactionField};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use core::convert::TryInto;
use std::ops::Range;

const HARDEND: u32 = 1 << 31;

/// Builder of find_addresses API
pub struct GetAddressesBuilder<'a> {
    seed: &'a Seed,
    account_index: Option<usize>,
    range: Option<Range<usize>>,
    bech32_hrp: String,
}

impl<'a> GetAddressesBuilder<'a> {
    /// Create find_addresses builder
    pub fn new(seed: &'a Seed) -> Self {
        Self {
            seed,
            account_index: None,
            range: None,
            bech32_hrp: "atoi".into(),
        }
    }

    /// Sets the account index.
    pub fn with_account_index(mut self, account_index: usize) -> Self {
        self.account_index = Some(account_index);
        self
    }

    /// Set range to the builder
    pub fn with_range(mut self, range: Range<usize>) -> Self {
        self.range = Some(range);
        self
    }

    /// Set range to the builder
    pub fn with_bech32_hpr(mut self, bech32_hrp: String) -> Self {
        self.bech32_hrp = bech32_hrp;
        self
    }

    /// Consume the builder and get a vector of address
    pub fn finish(self) -> Result<Vec<Ed25519Address>> {
        Ok(self
            .get_all()?
            .into_iter()
            .filter(|(_, internal)| !internal)
            .map(|(a, _)| a)
            .collect::<Vec<Ed25519Address>>())
    }

    /// Consume the builder and get the vector of address
    pub fn get_all(self) -> Result<Vec<(Ed25519Address, bool)>> {
        let mut path = self
            .account_index
            .map(|i| BIP32Path::from_str(&crate::account_path!(i)).expect("invalid account index"))
            .ok_or_else(|| Error::ChrysalisAddressError(String::from("missing account index")))?;

        let range = match self.range {
            Some(r) => r,
            None => 0..20,
        };

        let seed = match self.seed {
            Seed::Ed25519(s) => s,
            _ => panic!("Other seed scheme isn't supported yet."),
        };

        let mut addresses = Vec::new();
        for i in range {
            let address = generate_address(&seed, &mut path, i, false);
            let internal_address = generate_address(&seed, &mut path, i, true);
            if let Address::Ed25519(address) = address {
                addresses.push((address, false));
            }
            if let Address::Ed25519(internal_address) = internal_address {
                addresses.push((internal_address, true));
            }
        }

        Ok(addresses)
    }
}

fn generate_address(
    seed: &Ed25519Seed,
    path: &mut BIP32Path,
    index: usize,
    internal: bool,
) -> Address {
    path.push(internal as u32 + HARDEND);
    path.push(index as u32 + HARDEND);

    let public_key = Ed25519PrivateKey::generate_from_seed(seed, &path)
        .expect("Invalid Seed & BIP32Path. Probably because the index of path is not hardened.")
        .generate_public_key()
        .to_bytes();
    // Hash the public key to get the address
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(public_key);
    let mut result: [u8; 32] = [0; 32];
    hasher.finalize_variable(|res| {
        result = res.try_into().expect("Invalid Length of Public Key");
    });

    path.pop();
    path.pop();

    Address::Ed25519(Ed25519Address::new(result))
}

/// Encode an Ed25519Address to a TryteAddress
pub fn encode_migration_address(ed25519_address: Ed25519Address) -> TryteAddress {
    // Compute the BLAKE2b-256 hash H of A.
    let mut hasher = VarBlake2b::new(32).expect("Invalid output size");
    hasher.update(ed25519_address);
    let mut result: [u8; 32] = [0; 32];
    hasher.finalize_variable(|res| {
        result = res.try_into().expect("Can't convert hash result");
    });
    // let encoded = b1t6::encode::<T1B1Buf>(&[ed25519_address.as_ref(), &result[0..4]].concat());
    // let decoded = b1t6::decode(&encoded).unwrap();
    // Append the first 4 bytes of H to A, resulting in 36 bytes.
    let trytes = b1t6::encode::<T1B1Buf>(&[ed25519_address.as_ref(), &result[0..4]].concat())
        .iter_trytes()
        .map(char::from)
        .collect::<String>();
    // Prepend TRANSFER and pad with 9 to get 81 Trytes
    let transfer_address = format!("TRANSFER{}9", trytes);
    TryteAddress::from_inner_unchecked(
        TryteBuf::try_from_str(&transfer_address)
            .unwrap()
            .as_trits()
            .encode(),
    )
}

/// Decode a TryteAddress to an Ed25519Address
pub fn decode_migration_address(tryte_address: TryteAddress) -> Result<Ed25519Address> {
    let tryte_string = tryte_address
        .to_inner()
        .encode::<T3B1Buf>()
        .iter_trytes()
        .map(char::from)
        .collect::<String>();
    if &tryte_string[0..8] != "TRANSFER" {
        return Err(Error::ChrysalisAddressError(
            "Invalid address, doesn't start with 'TRANSFER'".into(),
        ));
    }
    if &tryte_string[tryte_string.len() - 1..] != "9" {
        return Err(Error::ChrysalisAddressError(
            "Invalid address, doesn't end with '9'".into(),
        ));
    }

    // TODO get this working
    // panicked at 'called `Result::unwrap()` on an `Err` value: InvalidTrytes(['H', 'L'])', iota-client\src\chrysalis2\address.rs:182:91
    let ed25519_address_bytes = b1t6::decode(&tryte_address.to_inner().subslice(24..240)).unwrap();
    println!("ed25519_address_bytes: {:?}", ed25519_address_bytes);

    //The first 32 bytes of the result are called A and the last 4 bytes H.
    let mut hasher = VarBlake2b::new(32).expect("Invalid output size");
    hasher.update(&ed25519_address_bytes[0..32]);
    let mut result: [u8; 32] = [0; 32];
    hasher.finalize_variable(|res| {
        result = res.try_into().expect("Can't convert hash result");
    });
    //Check that H matches the first 4 bytes of the BLAKE2b-256 hash of A.
    if ed25519_address_bytes[33..37] != result[0..4] {
        return Err(Error::ChrysalisAddressError(
            "Blake2b hash of the Ed25519Address doesn't match".into(),
        ));
    }

    Ok(Ed25519Address::new(
        ed25519_address_bytes[0..32].try_into().unwrap(),
    ))
}

/// Add 9 Trytes checksum to an address and return it as String
pub fn add_tryte_checksum(address: TryteAddress) -> String {
    let mut kerl = Kerl::new();
    let hash = kerl
        .digest(
            Trits::try_from_raw(
                &[address.to_inner().as_i8_slice(), &[0, 0, 0]].concat(),
                243,
            )
            .unwrap(),
        )
        .unwrap()
        .iter_trytes()
        .map(char::from)
        .collect::<String>();

    format!(
        "{}{}",
        address
            .to_inner()
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>(),
        &hash[72..81]
    )
}

#[test]
fn test_migration_address() {
    let ed25519_address = Ed25519Address::new(
        hex::decode("6f9e8510b88b0ea4fbc684df90ba310540370a0403067b22cef4971fec3e8bb8")
            .unwrap()
            .try_into()
            .unwrap(),
    );
    let encoded_address = encode_migration_address(ed25519_address.clone());
    let migration_address = add_tryte_checksum(encoded_address.clone());
    assert_eq!(migration_address.len(), 90);
    assert_eq!(&migration_address, "TRANSFERCDJWLVPAIXRWNAPXV9WYKVUZWWKXVBE9JBABJ9D9C9F9OEGADYO9CWDAGZHBRWIXLXG9MAJV9RJEOLXSJW");
    let decoded_address = decode_migration_address(encoded_address).unwrap();
    assert_eq!(decoded_address, ed25519_address);
}
