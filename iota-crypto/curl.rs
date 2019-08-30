use std::fmt;

use super::Result;
use super::{HashMode, Sponge};
use iota_constants::HASH_TRINARY_SIZE as HASH_LENGTH;

/// The length of the internal state
const STATE_LENGTH: usize = 3 * HASH_LENGTH;

const TRUTH_TABLE: [i8; 11] = [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];

/// The Curl struct is a Sponge that uses the Curl
/// hashing algorithm.
///```
/// use iota_crypto::{Sponge, Curl};
/// // Create an array of 243 1s
/// let input = [1; 243];
/// // Create an array of 243 0s
/// let mut out = [0; 243];
/// let mut curl = Curl::default();
/// curl.absorb(&input);
/// curl.squeeze(&mut out);
///```
#[derive(Clone, Copy)]
pub struct Curl {
    number_of_rounds: usize,
    scratchpad: [i8; STATE_LENGTH],
    state: [i8; STATE_LENGTH],
}

impl Default for Curl {
    fn default() -> Curl {
        Curl {
            number_of_rounds: 81,
            scratchpad: [0; STATE_LENGTH],
            state: [0; STATE_LENGTH],
        }
    }
}

impl fmt::Debug for Curl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Curl: [rounds: [{}], scratchpad: {:?}, state: {:?}",
            self.number_of_rounds,
            self.scratchpad.to_vec(),
            self.scratchpad.to_vec(),
        )
    }
}

impl Curl {
    /// Creates a new instance of Curl using the provided mode
    pub fn new(mode: HashMode) -> Result<Curl> {
        let mut curl = Curl::default();
        curl.number_of_rounds = match mode {
            HashMode::CURLP27 => 27,
            HashMode::CURLP81 => 81,
            other => return Err(format_err!("Invalid mode: {}", other)),
        };
        Ok(curl)
    }

    fn transform(&mut self) {
        let mut scratchpad_index = 0;
        for _ in 0..self.number_of_rounds {
            self.scratchpad[0..STATE_LENGTH].copy_from_slice(&self.state[0..STATE_LENGTH]);
            for state_index in 0..STATE_LENGTH {
                let prev_scratchpad_index = scratchpad_index;
                if scratchpad_index < 365 {
                    scratchpad_index += 364;
                } else {
                    scratchpad_index -= 365;
                }
                let truth_index = (self.scratchpad[prev_scratchpad_index]
                    + (self.scratchpad[scratchpad_index] << 2)
                    + 5) as usize;
                self.state[state_index] = TRUTH_TABLE[truth_index];
            }
        }
    }

    /// Provides a view into the internal state
    pub fn state(&self) -> &[i8] {
        &self.state
    }

    /// Provides a mutable view into the internal state
    pub fn state_mut(&mut self) -> &mut [i8] {
        &mut self.state
    }

    /// Number of rounds
    pub fn number_of_rounds(&self) -> usize {
        self.number_of_rounds
    }
}

impl Sponge for Curl {
    fn absorb(&mut self, trits: &[i8]) -> Result<()> {
        for chunk in trits.chunks(HASH_LENGTH) {
            if chunk.len() < HASH_LENGTH {
                self.state[0..chunk.len()].copy_from_slice(chunk);
            } else {
                self.state[0..HASH_LENGTH].copy_from_slice(chunk);
            }
            self.transform();
        }
        Ok(())
    }

    fn squeeze(&mut self, out: &mut [i8]) -> Result<()> {
        let trit_length = out.len();
        let hash_length = trit_length / HASH_LENGTH;

        for chunk in out.chunks_mut(HASH_LENGTH) {
            chunk.copy_from_slice(&self.state[0..HASH_LENGTH]);
            self.transform();
        }

        let last = trit_length - hash_length * HASH_LENGTH;
        out[trit_length - last..].copy_from_slice(&self.state[0..last]);
        if trit_length % HASH_LENGTH != 0 {
            self.transform();
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.state = [0; STATE_LENGTH];
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use iota_conversion::Trinary;
    const TRYTES: &str = "RSWWSFXPQJUBJROQBRQZWZXZJWMUBVIVMHPPTYSNW9YQIQQF9RCSJJCVZG9ZWITXNCSBBDHEEKDRBHVTWCZ9SZOOZHVBPCQNPKTWFNZAWGCZ9QDIMKRVINMIRZBPKRKQAIPGOHBTHTGYXTBJLSURDSPEOJ9UKJECUKCCPVIQQHDUYKVKISCEIEGVOQWRBAYXWGSJUTEVG9RPQLPTKYCRAJ9YNCUMDVDYDQCKRJOAPXCSUDAJGETALJINHEVNAARIPONBWXUOQUFGNOCUSSLYWKOZMZUKLNITZIFXFWQAYVJCVMDTRSHORGNSTKX9Z9DLWNHZSMNOYTU9AUCGYBVIITEPEKIXBCOFCMQPBGXYJKSHPXNUKFTXIJVYRFILAVXEWTUICZCYYPCEHNTK9SLGVL9RLAMYTAEPONCBHDXSEQZOXO9XCFUCPPMKEBR9IEJGQOPPILHFXHMIULJYXZJASQEGCQDVYFOM9ETXAGVMSCHHQLFPATWOSMZIDL9AHMSDCE9UENACG9OVFAEIPPQYBCLXDMXXA9UBJFQQBCYKETPNKHNOUKCSSYLWZDLKUARXNVKKKHNRBVSTVKQCZL9RY9BDTDTPUTFUBGRMSTOTXLWUHDMSGYRDSZLIPGQXIDMNCNBOAOI9WFUCXSRLJFIVTIPIAZUK9EDUJJ9B9YCJEZQQELLHVCWDNRH9FUXDGZRGOVXGOKORTCQQA9JXNROLETYCNLRMBGXBL9DQKMOAZCBJGWLNJLGRSTYBKLGFVRUF9QOPZVQFGMDJA9TBVGFJDBAHEVOLW9GNU9NICLCQJBOAJBAHHBZJGOFUCQMBGYQLCWNKSZPPBQMSJTJLM9GXOZHTNDLGIRCSIJAZTENQVQDHFSOQM9WVNWQQJNOPZMEISSCLOADMRNWALBBSLSWNCTOSNHNLWZBVCFIOGFPCPRKQSRGKFXGTWUSCPZSKQNLQJGKDLOXSBJMEHQPDZGSENUKWAHRNONDTBLHNAKGLOMCFYRCGMDOVANPFHMQRFCZIQHCGVORJJNYMTORDKPJPLA9LWAKAWXLIFEVLKHRKCDG9QPQCPGVKIVBENQJTJGZKFTNZHIMQISVBNLHAYSSVJKTIELGTETKPVRQXNAPWOBGQGFRMMK9UQDWJHSQMYQQTCBMVQKUVGJEAGTEQDN9TCRRAZHDPSPIYVNKPGJSJZASZQBM9WXEDWGAOQPPZFLAMZLEZGXPYSOJRWL9ZH9NOJTUKXNTCRRDO9GKULXBAVDRIZBOKJYVJUSHIX9F9O9ACYCAHUKBIEPVZWVJAJGSDQNZNWLIWVSKFJUMOYDMVUFLUXT9CEQEVRFBJVPCTJQCORM9JHLYFSMUVMFDXZFNCUFZZIKREIUIHUSHRPPOUKGFKWX9COXBAZMQBBFRFIBGEAVKBWKNTBMLPHLOUYOXPIQIZQWGOVUWQABTJT9ZZPNBABQFYRCQLXDHDEX9PULVTCQLWPTJLRSVZQEEYVBVY9KCNEZXQLEGADSTJBYOXEVGVTUFKNCNWMEDKDUMTKCMRPGKDCCBDHDVVSMPOPUBZOMZTXJSQNVVGXNPPBVSBL9WWXWQNMHRMQFEQYKWNCSW9URI9FYPT9UZMAFMMGUKFYTWPCQKVJ9DIHRJFMXRZUGI9TMTFUQHGXNBITDSORZORQIAMKY9VRYKLEHNRNFSEFBHF9KXIQAEZEJNQOENJVMWLMHI9GNZPXYUIFAJIVCLAGKUZIKTJKGNQVTXJORWIQDHUPBBPPYOUPFAABBVMMYATXERQHPECDVYGWDGXFJKOMOBXKRZD9MCQ9LGDGGGMYGUAFGMQTUHZOAPLKPNPCIKUNEMQIZOCM9COAOMZSJ9GVWZBZYXMCNALENZ9PRYMHENPWGKX9ULUIGJUJRKFJPBTTHCRZQKEAHT9DC9GSWQEGDTZFHACZMLFYDVOWZADBNMEM9XXEOMHCNJMDSUAJRQTBUWKJF9RZHK9ACGUNI9URFIHLXBXCEODONPXBSCWP9WNAEYNALKQHGULUQGAFL9LB9NBLLCACLQFGQMXRHGBTMI9YKAJKVELRWWKJAPKMSYMJTDYMZ9PJEEYIRXRMMFLRSFSHIXUL9NEJABLRUGHJFL9RASMSKOI9VCFRZ9GWTMODUUESIJBHWWHZYCLDENBFSJQPIOYC9MBGOOXSWEMLVU9L9WJXKZKVDBDMFSVHHISSSNILUMWULMVMESQUIHDGBDXROXGH9MTNFSLWJZRAPOKKRGXAAQBFPYPAAXLSTMNSNDTTJQSDQORNJS9BBGQ9KQJZYPAQ9JYQZJ9B9KQDAXUACZWRUNGMBOQLQZUHFNCKVQGORRZGAHES9PWJUKZWUJSBMNZFILBNBQQKLXITCTQDDBV9UDAOQOUPWMXTXWFWVMCXIXLRMRWMAYYQJPCEAAOFEOGZQMEDAGYGCTKUJBS9AGEXJAFHWWDZRYEN9DN9HVCMLFURISLYSWKXHJKXMHUWZXUQARMYPGKRKQMHVR9JEYXJRPNZINYNCGZHHUNHBAIJHLYZIZGGIDFWVNXZQADLEDJFTIUTQWCQSX9QNGUZXGXJYUUTFSZPQKXBA9DFRQRLTLUJENKESDGTZRGRSLTNYTITXRXRGVLWBTEWPJXZYLGHLQBAVYVOSABIVTQYQM9FIQKCBRRUEMVVTMERLWOK";
    const HASH: &str =
        "TIXEPIEYMGURTQ9ABVYVQSWMNGCVQFASMFAEQWUZCLIWLCDIGYVXOEJBBEMZOIHAYSUQMEFOGZBXUMHQW";

    #[test]
    fn test_curl_works() {
        let size = 8019;
        let mut in_trits = TRYTES.trits();
        let mut hash_trits = vec![0; HASH_LENGTH];
        let mut curl = Curl::default();
        curl.absorb(&mut in_trits[..size]).unwrap();
        curl.squeeze(&mut hash_trits).unwrap();
        let out_trytes = hash_trits.trytes().unwrap();
        assert_eq!(HASH, out_trytes);
    }
}
