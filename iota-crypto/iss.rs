use iota_constants;

use crate::Result;

use super::curl::Curl;
use super::kerl::Kerl;
use super::{hash_with_mode, HashMode, Sponge};
use iota_constants::HASH_TRINARY_SIZE as HASH_LENGTH;

/// Number of fragment chunks
pub const NUMBER_OF_FRAGMENT_CHUNKS: usize = 27;
/// Length of a fragment
pub const FRAGMENT_LENGTH: usize = HASH_LENGTH * NUMBER_OF_FRAGMENT_CHUNKS;
/// The amount of valid security levels
pub const NUMBER_OF_SECURITY_LEVELS: usize = 3;
/// The width of tryte
pub const TRYTE_WIDTH: usize = 3;
/// Normalized fragment length
pub const NORMALIZED_FRAGMENT_LENGTH: usize = HASH_LENGTH / TRYTE_WIDTH / NUMBER_OF_SECURITY_LEVELS;

/// Create a subseed
///
/// * `mode` - The hashing mode to use
/// * `seed` - The generation seed
/// * `index` - How many address permutations to iterate through
pub fn subseed(mode: HashMode, seed: &[i8], index: usize) -> Result<[i8; HASH_LENGTH]> {
    let mut subseed_preimage = seed.to_vec();
    for _ in 0..index {
        for trit in &mut subseed_preimage {
            *trit += 1;
            if *trit > iota_constants::MAX_TRIT_VALUE {
                *trit = iota_constants::MIN_TRIT_VALUE;
            } else {
                break;
            }
        }
    }
    let mut subseed = [0; HASH_LENGTH];
    hash_with_mode(mode, &subseed_preimage, &mut subseed)?;
    Ok(subseed)
}

/// Key a subseed
///
/// * `mode` - The hashing mode to use
/// * `subseed` - Subseed used for key generation
/// * `number_of_fragments` - Number of fragments to generate
pub fn key(mode: HashMode, subseed: &mut [i8], number_of_fragments: usize) -> Result<Vec<i8>> {
    ensure!(
        subseed.len() == HASH_LENGTH,
        "Invalid subseed length: {}",
        subseed.len()
    );

    let mut key = vec![0; FRAGMENT_LENGTH * number_of_fragments];
    hash_with_mode(mode, subseed, &mut key)?;

    Ok(key)
}

/// Generate digests
///
/// * `mode` - The hashing mode to use
/// * `key` - kKey slice used to generate digests
pub fn digests(mode: HashMode, key: &[i8]) -> Result<Vec<i8>> {
    ensure!(
        !key.is_empty() && key.len() % FRAGMENT_LENGTH == 0,
        "Invalid key length: {}",
        key.len()
    );
    match mode {
        HashMode::CURLP27 | HashMode::CURLP81 => {
            let mut curl = Curl::new(mode)?;
            Ok(digests_helper(&mut curl, key)?)
        }
        HashMode::Kerl => {
            let mut kerl = Kerl::default();
            Ok(digests_helper(&mut kerl, key)?)
        }
    }
}

fn digests_helper(hash: &mut impl Sponge, key: &[i8]) -> Result<Vec<i8>> {
    let mut digests = vec![0; key.len() / FRAGMENT_LENGTH * HASH_LENGTH];
    for i in 0..key.len() / FRAGMENT_LENGTH {
        let mut buffer = key[i * FRAGMENT_LENGTH..(i + 1) * FRAGMENT_LENGTH].to_vec();
        for j in 0..NUMBER_OF_FRAGMENT_CHUNKS {
            for _ in 0..iota_constants::MAX_TRYTE_VALUE - iota_constants::MIN_TRYTE_VALUE {
                hash.reset();
                let offset = j * HASH_LENGTH;
                hash.absorb(&buffer[offset..offset + HASH_LENGTH])?;
                hash.squeeze(&mut buffer[offset..offset + HASH_LENGTH])?;
            }
        }
        hash.reset();
        hash.absorb(&buffer)?;
        let offset = i * HASH_LENGTH;
        hash.squeeze(&mut digests[offset..offset + HASH_LENGTH])?;
    }
    Ok(digests)
}

/// Generate address
///
/// * `mode` - The hashing mode to use
/// * `digests` - Digests used to generate address
pub fn address(mode: HashMode, digests: &mut [i8]) -> Result<[i8; HASH_LENGTH]> {
    ensure!(
        !digests.is_empty() && digests.len() % HASH_LENGTH == 0,
        "Invalid key length: {}",
        digests.len()
    );
    let mut address = [0; HASH_LENGTH];
    hash_with_mode(mode, digests, &mut address)?;
    Ok(address)
}

/// Generate normalize bundle
///
/// * `bundle` - Bundle to normalize
pub fn normalized_bundle(bundle: &[i8]) -> Result<[i8; HASH_LENGTH / TRYTE_WIDTH]> {
    ensure!(
        bundle.len() == HASH_LENGTH,
        "Invalid bundle length: {}",
        bundle.len()
    );
    let mut normalized_bundle = [0; HASH_LENGTH / TRYTE_WIDTH];
    normalized_bundle_in_place(bundle, &mut normalized_bundle);
    Ok(normalized_bundle)
}

/// Normalize a bundle in place
///
/// * `bundle` - Bundle to normalize
/// * `normalized_bundle` - Destination slice to modify in place
pub fn normalized_bundle_in_place(bundle: &[i8], normalized_bundle: &mut [i8]) {
    for i in 0..NUMBER_OF_SECURITY_LEVELS {
        let mut sum = 0;
        let offset = HASH_LENGTH / TRYTE_WIDTH / NUMBER_OF_SECURITY_LEVELS;
        for j in i * offset..(i + 1) * offset {
            normalized_bundle[j] = bundle[j * TRYTE_WIDTH]
                + bundle[j * TRYTE_WIDTH + 1] * 3
                + bundle[j * TRYTE_WIDTH + 2] * 9;
            sum += normalized_bundle[j];
        }
        if sum > 0 {
            while sum > 0 {
                for trit in normalized_bundle
                    .iter_mut()
                    .skip(i * offset)
                    .take((i + 1) * offset)
                {
                    if *trit > iota_constants::MIN_TRYTE_VALUE {
                        *trit -= 1;
                        break;
                    }
                }
                sum -= 1;
            }
        } else {
            while sum < 0 {
                for trit in normalized_bundle
                    .iter_mut()
                    .skip(i * offset)
                    .take((i + 1) * offset)
                {
                    if *trit < iota_constants::MAX_TRYTE_VALUE {
                        *trit += 1;
                        break;
                    }
                }
                sum += 1;
            }
        }
    }
}

/// Generate a signature fragment
///
/// * `mode` - The hashing mode to use
/// * `normalized_bundle_fragment` - Normalized bundle fragment to sign
/// * `key_fragment` -  Key fragment to use
pub fn signature_fragment(
    mode: HashMode,
    normalized_bundle_fragment: &[i8],
    key_fragment: &[i8],
) -> Result<Vec<i8>> {
    ensure!(
        normalized_bundle_fragment.len() == NORMALIZED_FRAGMENT_LENGTH,
        "Invalid normalized bundle fragment length: {}",
        normalized_bundle_fragment.len()
    );
    ensure!(
        key_fragment.len() == FRAGMENT_LENGTH,
        "Invalid key fragment length: {}",
        key_fragment.len()
    );
    let mut signature_fragment = key_fragment.to_vec();
    match mode {
        HashMode::CURLP27 | HashMode::CURLP81 => {
            let mut curl = Curl::new(mode).unwrap();
            signature_fragment_helper(
                &mut curl,
                &normalized_bundle_fragment,
                &mut signature_fragment,
            )?;
        }
        HashMode::Kerl => {
            let mut kerl = Kerl::default();
            signature_fragment_helper(
                &mut kerl,
                &normalized_bundle_fragment,
                &mut signature_fragment,
            )?;
        }
    }
    Ok(signature_fragment)
}

fn signature_fragment_helper(
    hash: &mut impl Sponge,
    normalized_bundle_fragment: &[i8],
    out: &mut [i8],
) -> Result<()> {
    for (j, trit) in normalized_bundle_fragment
        .iter()
        .enumerate()
        .take(NUMBER_OF_FRAGMENT_CHUNKS)
    {
        for _ in 0..iota_constants::MAX_TRYTE_VALUE - *trit {
            hash.reset();
            let offset = j * HASH_LENGTH;
            hash.absorb(&out[offset..offset + HASH_LENGTH])?;
            hash.squeeze(&mut out[offset..offset + HASH_LENGTH])?;
        }
    }
    Ok(())
}

/// Generate a digest
///
/// * `mode` - The hashing mode to use
/// * `normalized_bundle_fragment` - Normalized bundle fragment to digest
/// * `signature_fragment` -  Signature fragment to use
pub fn digest(
    mode: HashMode,
    normalized_bundle_fragment: &[i8],
    signature_fragment: &[i8],
) -> Result<[i8; HASH_LENGTH]> {
    ensure!(
        normalized_bundle_fragment.len() == HASH_LENGTH / TRYTE_WIDTH / NUMBER_OF_SECURITY_LEVELS,
        "Invalid normalized bundle fragment length: {}",
        normalized_bundle_fragment.len()
    );
    ensure!(
        signature_fragment.len() == FRAGMENT_LENGTH,
        "Invalid signature fragment length: {}",
        signature_fragment.len()
    );
    let mut digest = [0; HASH_LENGTH];
    match mode {
        HashMode::CURLP27 | HashMode::CURLP81 => {
            let mut curl = Curl::new(mode).unwrap();
            digest_in_place(
                &mut curl,
                normalized_bundle_fragment,
                signature_fragment,
                &mut digest,
            )?;
        }
        HashMode::Kerl => {
            let mut kerl = Kerl::default();
            digest_in_place(
                &mut kerl,
                normalized_bundle_fragment,
                signature_fragment,
                &mut digest,
            )?;
        }
    }
    Ok(digest)
}

/// Hash digest in place
///
/// * `mode` - The hashing mode to use
/// * `normalized_bundle_fragment` - Normalized bundle fragment to digest
/// * `signature_fragment` -  Signature fragment to use
/// * `digest` - Destination slice to modify in place
pub fn digest_in_place(
    hash: &mut impl Sponge,
    normalized_bundle_fragment: &[i8],
    signature_fragment: &[i8],
    digest: &mut [i8],
) -> Result<()> {
    let mut buffer = signature_fragment[0..FRAGMENT_LENGTH].to_vec();
    for (j, trit) in normalized_bundle_fragment
        .iter()
        .enumerate()
        .take(NUMBER_OF_FRAGMENT_CHUNKS)
    {
        for _ in 0..*trit - iota_constants::MIN_TRYTE_VALUE {
            hash.reset();
            let offset = j * HASH_LENGTH;
            hash.absorb(&buffer[offset..offset + HASH_LENGTH])?;
            hash.squeeze(&mut buffer[offset..offset + HASH_LENGTH])?;
        }
    }
    hash.reset();
    hash.absorb(&buffer)?;
    hash.squeeze(digest)?;
    Ok(())
}

/// Retrieve the merkle root
///
/// * `mode` - The hashing mode to use
/// * `hash` - Hash to absorb
/// * `trits` - Trits to absorb
/// * `offset` - Trit offset to start at
/// * `index` - Used to alternate the order trits and hash are absorbed
/// * `size` - Number of hash iterations
pub fn get_merkle_root(
    mode: HashMode,
    hash: &[i8],
    trits: &mut [i8],
    offset: usize,
    index: usize,
    size: usize,
) -> Result<[i8; HASH_LENGTH]> {
    match mode {
        HashMode::CURLP27 | HashMode::CURLP81 => {
            let mut curl = Curl::new(mode).unwrap();
            get_merkle_root_helper(&mut curl, hash, trits, offset, index, size)
        }
        HashMode::Kerl => {
            let mut kerl = Kerl::default();
            get_merkle_root_helper(&mut kerl, hash, trits, offset, index, size)
        }
    }
}

fn get_merkle_root_helper(
    curl: &mut impl Sponge,
    hash: &[i8],
    trits: &[i8],
    offset: usize,
    index: usize,
    size: usize,
) -> Result<[i8; HASH_LENGTH]> {
    let empty = [0; HASH_LENGTH];
    let mut index = index;
    let mut tmp = [0; HASH_LENGTH];
    for i in 0..size {
        curl.reset();
        if (index & 1) == 0 {
            curl.absorb(hash)?;
            let offset = offset + i * HASH_LENGTH;
            curl.absorb(&trits[offset..offset + HASH_LENGTH])?;
        } else {
            let offset = offset + i * HASH_LENGTH;
            curl.absorb(&trits[offset..offset + HASH_LENGTH])?;
            curl.absorb(hash)?;
        }
        curl.squeeze(&mut tmp)?;
        index >>= 1;
    }
    if index != 0 {
        return Ok(empty);
    }
    Ok(tmp)
}

#[cfg(test)]
mod tests {

    use super::*;

    use iota_conversion::Trinary;
    const SEED: &str =
        "NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN";
    const MESSAGE: &str = "JCRNMXX9DIEVJJG9VW9QDUMVDGDVHANQDTCPPOPHLTBUBXULSIALRBVUINDPNGUFZLKDPOK9WBJMYCXF9MFQN9ZKMROOXHULIDDXRNWMDENBWJWVVA9XPNHQUVDFSMQ9ETWKWGLOLYPWW9GQPVNDYJIRDBWVCBUHUEGELSTLEXGAMMQAHSUEABKUSFOVGYRQBXJMORXIDTIPENPAFIUV9DOGZCAEPRJQOISRZDZBWWQQJVQDS9YGCMNADNVSUTXXAONPHBFCMWSVFYYXXWDZXFP9SZGLRCHHGKLNAQPMAXHFUUSQEKDAPH9GFVHMYDITCTFSIJEZFADOJVDOEXOTDDPZYLKKDHCGPXYMGRKAGOEQYHTCTGKMZOKMZJLCQOYE9KFVRQLXDPBALUSEQSQDFPPUYALCDYWSHANNQYKIMAZMKQQ9XVCSJHAWXLY9IIREZTSOFRMRGKDQPIEMDXTBDTY9DKOAIUEGNLUSRFZYPRNUOHFGDYIWFVKIUNYBGBHICRQTLDQQUTJX9DDSQANVKMCDZ9VEQBCHHSATVFIDYR9XUSDJHQDRBVK9JUUZVWGCCWVXAC9ZIOKBWOKCTCJVXIJFBSTLNZCPJMAKDPYLTHMOKLFDNONJLLDBDXNFKPKUBKDU9QFSXGVXS9PEDBDDBGFESSKCWUWMTOGHDLOPRILYYPSAQVTSQYLIPK9ATVMMYSTASHEZEFWBUNR9XKGCHR9MB";

    #[test]
    fn address_generation_curl() {
        let seed_trits = SEED.trits();
        let mut subseed = subseed(HashMode::CURLP81, &seed_trits, 0).unwrap();
        let key = key(HashMode::CURLP81, &mut subseed, 2).unwrap();
        let mut digest = digests(HashMode::CURLP81, &key).unwrap();
        let address = address(HashMode::CURLP81, &mut digest).unwrap();
        assert_eq!(
            &address.trytes().unwrap(),
            "D9XCNSCCAJGLWSQOQAQNFWANPYKYMCQ9VCOMROLDVLONPPLDFVPIZNAPVZLQMPFYJPAHUKIAEKNCQIYJZ"
        );
    }

    #[test]
    fn address_generation_kerl() {
        let seed_trits = SEED.trits();
        let mut subseed = subseed(HashMode::Kerl, &seed_trits, 0).unwrap();
        let key = key(HashMode::Kerl, &mut subseed, 2).unwrap();
        let mut digest = digests(HashMode::Kerl, &key).unwrap();
        let address = address(HashMode::Kerl, &mut digest).unwrap();
        assert_eq!(
            &address.trytes().unwrap(),
            "MDWYEJJHJDIUVPKDY9EACGDJUOP9TLYDWETUBOYCBLYXYYYJYUXYUTCTPTDGJYFKMQMCNZDQPTBE9AFIW"
        );
    }

    #[test]
    fn resolve_signature_curl() {
        let modes = [HashMode::CURLP81, HashMode::Kerl];
        for &mode in modes.iter() {
            let seed_trits = SEED.trits();
            let mut subseed = subseed(mode, &seed_trits, 10).unwrap();
            let key = key(mode, &mut subseed, 1).unwrap();

            let mut kerl = Kerl::default();
            let message_trits = MESSAGE.trits();
            kerl.absorb(&message_trits).unwrap();
            let mut message_hash = [0; HASH_LENGTH];
            kerl.squeeze(&mut message_hash).unwrap();
            let normalized_fragment =
                normalized_bundle(&message_hash).unwrap()[..NUMBER_OF_FRAGMENT_CHUNKS].to_vec();
            let signature = signature_fragment(mode, &normalized_fragment, &key).unwrap();
            let mut sig_digest = digest(mode, &normalized_fragment, &signature).unwrap();
            let signed_address = address(mode, &mut sig_digest).unwrap().to_vec();
            let mut digest = digests(mode, &key).unwrap();
            let address = address(mode, &mut digest).unwrap().to_vec();
            assert_eq!(&address, &signed_address);
        }
    }
}
