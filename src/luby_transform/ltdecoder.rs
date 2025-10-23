use std::collections::HashMap;
use super::prng::PRNG;
#[allow(unused)]
pub struct LtDecoder {
    k: usize,
    block_size: usize,
    prng: PRNG,
    received_blocks: HashMap<usize, Vec<u8>>,
    decoded_blocks: HashMap<usize, Vec<u8>>,
    current_round: usize,
}


impl LtDecoder {
    pub fn new(k: usize, block_size: usize, prng: PRNG) -> Self {
        Self {
            k,
            block_size,
            prng,
            received_blocks: HashMap::new(),
            decoded_blocks: HashMap::new(),
            current_round: 0,
        }
    }
}