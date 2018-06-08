use super::traits::{ICurl, HASH_LENGTH};
use utils::converter::array_copy;

const STATE_LENGTH: usize = 3 * HASH_LENGTH;
const TRUTH_TABLE: [i32; 11] = [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];
pub enum Mode {
    CURLP27,
    CURLP81,
}

#[derive(Clone)]
pub struct Curl {
    number_of_rounds: i32,
    scratchpad: [i32; STATE_LENGTH],
    state: [i32; STATE_LENGTH],
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

impl Curl {
    pub fn new(mode: Mode) -> Curl {
        let mut curl = Curl::default();
        curl.number_of_rounds = match mode {
            Mode::CURLP27 => 27,
            Mode::CURLP81 => 81,
        };
        curl.state = [0; STATE_LENGTH];
        curl
    }

    fn transform(&mut self) {
        let mut scratchpad_index = 0;
        let mut prev_scratchpad_index = 0;
        for _round in 0..self.number_of_rounds {
            array_copy(&mut self.state, 0, &mut self.scratchpad, 0, STATE_LENGTH);
            for state_index in 0..STATE_LENGTH {
                prev_scratchpad_index = scratchpad_index;
                if scratchpad_index < 365 {
                    scratchpad_index += 364;
                } else {
                    scratchpad_index -= 365;
                }
                self.state[state_index] = TRUTH_TABLE[(self.scratchpad[prev_scratchpad_index]
                                                          + (self.scratchpad[scratchpad_index] << 2)
                                                          + 5)
                                                          as usize];
            }
        }
    }

    pub fn reset(&mut self) {
        self.state = [0; STATE_LENGTH];
    }
}

impl ICurl for Curl {
    fn absorb(&mut self, trits: &[i32]) {
        self.absorb_offset(trits, 0, trits.len());
    }

    fn absorb_offset(&mut self, trits: &[i32], offset: usize, length: usize) {
        let mut offset = offset;
        let mut length = if length < HASH_LENGTH {
            length
        } else {
            HASH_LENGTH
        };
        while length > 0 {
            array_copy(trits, offset, &mut self.state, 0, length);
            self.transform();
            offset += HASH_LENGTH;
            length -= HASH_LENGTH;
        }
    }

    fn squeeze(&mut self, trits: &mut [i32]) {
        let len = trits.len();
        self.squeeze_offset(trits, 0, len);
    }

    fn squeeze_offset(&mut self, out: &mut [i32], offset: usize, length: usize) {
        let mut length = length;
        let mut tmp_offset = offset;
        while length > 0 {
            array_copy(
                &mut self.state,
                0,
                out,
                tmp_offset,
                if length < HASH_LENGTH {
                    length
                } else {
                    HASH_LENGTH
                },
            );
            self.transform();
            tmp_offset += HASH_LENGTH;
            length -= HASH_LENGTH;
        }
    }

    fn state(&self) -> &[i32] {
        &self.state
    }

    fn state_mut(&mut self) -> &mut [i32] {
        &mut self.state
    }
}
