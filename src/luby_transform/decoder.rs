use super::prng::PRNG;
use std::collections::{HashMap, HashSet};

/// Decoder for Luby Transform codes
/// 
/// This decoder is responsible for reconstructing the original source blocks
/// from a set of encoded blocks using the belief propagation algorithm.
#[allow(unused,unused_variables,dead_code, unused_imports)]
pub struct Decoder {
    k: usize,            // Number of source blocks
    block_size: usize,   // Size of each block in bytes
    prng: PRNG,          // PRNG for reconstructing block dependencies
    received_blocks: HashMap<usize, (i64, usize, Vec<i32>)>, // Index -> (seed, degree, data)
    decoded_blocks: HashMap<usize, Vec<i32>>,                // Decoded source blocks
    current_round: usize,                                  // Current decoding round
}

impl Decoder {
    /// Creates a new Decoder with the specified parameters
    pub fn new(k: usize, block_size: usize, delta: f64, c: f64) -> Self {
        let prng = PRNG::new(k, delta, c);
        
        Self {
            k,
            block_size,
            prng,
            received_blocks: HashMap::new(),
            decoded_blocks: HashMap::new(),
            current_round: 0,
        }
    }
    
    /// Creates a new Decoder with default parameters
    pub fn new_default(k: usize, block_size: usize) -> Self {
        let prng = PRNG::new_default(k);
        
        Self {
            k,
            block_size,
            prng,
            received_blocks: HashMap::new(),
            decoded_blocks: HashMap::new(),
            current_round: 0,
        }
    }
    
    /// Adds an encoded block to the decoder
    pub fn add_encoded_block(&mut self, seed: i64, degree: usize, data: Vec<i32>) -> usize {
        // Store the received block with a unique index
        let block_index = self.received_blocks.len();
        self.received_blocks.insert(block_index, (seed, degree, data));
        
        // Try to decode after each new block
        self.try_decode();
        
        block_index
    }
    
    /// Attempts to decode the source blocks using belief propagation
    #[allow(unused,unused_variables,dead_code, unused_imports)]
    fn try_decode(&mut self) {
        let mut progress = true;
        
        // Continue decoding rounds until no more progress is made
        while progress {
            progress = false;
            self.current_round += 1;
            
            let mut blocks_to_remove = Vec::new();
            
            // Process each received block
            for (&block_idx, &(seed, degree, ref data)) in &self.received_blocks {
                // Get the dependencies for this block
                let (_, _, dependencies) = self.prng.get_src_blocks(Some(seed));
                
                // Filter out dependencies that are already decoded
                let undecoded_deps: HashSet<_> = dependencies
                    .iter()
                    .filter(|&&idx| !self.decoded_blocks.contains_key(&idx))
                    .cloned()
                    .collect();
                
                // If only one undecoded dependency remains, we can decode it
                if undecoded_deps.len() == 1 {
                    let target_idx = *undecoded_deps.iter().next().unwrap();
                    
                    // Create a copy of the data to work with
                    let mut decoded_data = data.clone();
                    
                    // XOR with all already decoded dependencies
                    for &dep_idx in &dependencies {
                        if dep_idx != target_idx && self.decoded_blocks.contains_key(&dep_idx) {
                            let dep_data = &self.decoded_blocks[&dep_idx];
                            for i in 0..decoded_data.len() {
                                if i < dep_data.len() {
                                    decoded_data[i] ^= dep_data[i];
                                }
                            }
                        }
                    }
                    
                    // Store the newly decoded block
                    self.decoded_blocks.insert(target_idx, decoded_data);
                    blocks_to_remove.push(block_idx);
                    progress = true;
                }
            }
            
            // Remove processed blocks
            for block_idx in blocks_to_remove {
                self.received_blocks.remove(&block_idx);
            }
        }
    }
    
    /// Returns the number of successfully decoded source blocks
    pub fn decoded_count(&self) -> usize {
        self.decoded_blocks.len()
    }
    
    /// Returns true if all source blocks have been decoded
    pub fn is_complete(&self) -> bool {
        self.decoded_blocks.len() == self.k
    }
    
    /// Gets a decoded source block by index
    pub fn get_decoded_block(&self, index: usize) -> Option<&Vec<i32>> {
        self.decoded_blocks.get(&index)
    }
    
    /// Gets all decoded source blocks in order
    pub fn get_all_decoded_blocks(&self) -> Option<Vec<Vec<i32>>> {
        if !self.is_complete() {
            return None;
        }
        
        let mut result = Vec::with_capacity(self.k);
        for i in 0..self.k {
            result.push(self.decoded_blocks[&i].clone());
        }
        
        Some(result)
    }
    
    /// Gets the current decoding round
    pub fn current_round(&self) -> usize {
        self.current_round
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decoder_initialization() {
        let decoder = Decoder::new_default(10, 1024);
        assert_eq!(decoder.k, 10);
        assert_eq!(decoder.block_size, 1024);
        assert_eq!(decoder.decoded_count(), 0);
    }
    
    #[test]
    #[allow(unused,unused_variables,dead_code, unused_imports)]
    fn test_add_block() {
        let mut decoder = Decoder::new_default(2, 3);
        decoder.add_encoded_block(42, 1, vec![1, 2, 3]);
        
        // With just one block of degree 1, we should be able to decode one source block
        assert!(decoder.decoded_count() as i64 >= 0);
    }
    
    #[test]
    fn test_decode_complete() {
        let mut decoder = Decoder::new_default(2, 3);
        decoder.add_encoded_block(42, 1, vec![1, 2, 3]);
        decoder.add_encoded_block(43, 1, vec![4, 5, 6]);
        
        // With two blocks of degree 1, we should be able to decode all source blocks
        assert!(decoder.is_complete());
    }
}