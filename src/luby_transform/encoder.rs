use super::prng::PRNG;
use std::collections::HashSet;

/// Encoder for Luby Transform codes
/// 
/// This encoder is responsible for generating encoded blocks from the source data blocks
/// using the Luby Transform algorithm.
pub struct Encoder {
    source_blocks: Vec<Vec<u8>>,
    prng: PRNG,
    k: usize,
}

impl Encoder {
    /// Creates a new Encoder with the given source blocks
    pub fn new(source_blocks: Vec<Vec<u8>>, delta: f64, c: f64) -> Self {
        let k = source_blocks.len();
        let prng = PRNG::new(k, delta, c);
        
        Self {
            source_blocks,
            prng,
            k,
        }
    }
    
    /// Creates a new Encoder with default parameters
    pub fn new_default(source_blocks: Vec<Vec<u8>>) -> Self {
        let k = source_blocks.len();
        let prng = PRNG::new_default(k);
        
        Self {
            source_blocks,
            prng,
            k,
        }
    }
    
    /// Generates a single encoded block
    /// 
    /// Returns a tuple containing:
    /// - The seed used for this block (for decoder)
    /// - The degree of the block
    /// - The encoded data block
    pub fn generate_encoded_block(&mut self, seed: Option<i64>) -> (i64, usize, Vec<u8>) {
        // Use the PRNG to get source block indices
        let (blockseed, d, indices) = self.prng.get_src_blocks(seed);
        
        // XOR the selected source blocks
        let encoded_block = self.xor_blocks(&indices);
        
        (blockseed, d, encoded_block)
    }
    
    /// XORs the specified source blocks together
    fn xor_blocks(&self, indices: &HashSet<usize>) -> Vec<u8> {
        if indices.is_empty() {
            return Vec::new();
        }
        
        // Get the first block as the starting point
        let first_idx = *indices.iter().next().unwrap();
        let mut result = self.source_blocks[first_idx].clone();
        
        // XOR with the remaining blocks
        for &idx in indices.iter().skip(1) {
            if !self.source_blocks[idx].is_empty() {
                // Ensure blocks are the same length
                if result.len() != self.source_blocks[idx].len() {
                    panic!("Source blocks must be of the same length");
                }
                
                // XOR operation
                for i in 0..result.len() {
                    result[i] ^= self.source_blocks[idx][i];
                }
            }
        }
        
        result
    }
    
    /// Get the number of source blocks
    pub fn source_block_count(&self) -> usize {
        self.k
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encoder_initialization() {
        let source_blocks = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let encoder = Encoder::new_default(source_blocks);
        assert_eq!(encoder.source_block_count(), 3);
    }
    
    #[test]
    fn test_encoded_block_generation() {
        let source_blocks = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let source_blocks_len = source_blocks.len();
        let mut encoder = Encoder::new_default(source_blocks);
        
        // Generate a block with a known seed for reproducibility
        let (seed, d, encoded_block) = encoder.generate_encoded_block(Some(42));
        for i in 0..encoded_block.len() {
            // assert_eq!(encoded_block[i], source_blocks[i][0] ^ source_blocks[i][1]);
            println!("encoded_block[{}] = {}", i, encoded_block[i]);
        }
        assert_eq!(seed, 42);
        assert!(d >= 1 && d <= source_blocks_len);
        assert_eq!(encoded_block.len(), 3);
    }
}