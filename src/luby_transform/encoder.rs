use super::prng::PRNG;
use std::collections::HashSet;

/// Encoder for Luby Transform codes
/// 
/// This encoder is responsible for generating encoded blocks from the source data blocks
/// using the Luby Transform algorithm.
pub struct Encoder {
    source_blocks: Vec<Vec<i32>>,
    prng: PRNG,
    k: usize,
}

impl Encoder {
    /// Creates a new Encoder with the given source blocks
    pub fn new(source_blocks: Vec<Vec<i32>>, delta: f64, c: f64) -> Self {
        let k = source_blocks.len();
        let prng = PRNG::new(k, delta, c);
        
        Self {
            source_blocks,
            prng,
            k,
        }
    }
    
    /// Creates a new Encoder with default parameters
    pub fn new_default(source_blocks: Vec<Vec<i32>>, seed: Option<i64>) -> Self {
        let k = source_blocks.len();
        let mut prng = PRNG::new_default(k);
        prng.set_seed(seed.unwrap_or(0));
        
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
    pub fn generate_encoded_block(&mut self, seed: Option<i64>) -> (i64, usize, HashSet<usize>,Vec<i32>) {
        // Use the PRNG to get source block indices
        let (blockseed, d, indices) = self.prng.get_src_blocks(seed);
        
        // XOR the selected source blocks
        let encoded_block = self.xor_blocks(&indices);
        
        (blockseed, d, indices, encoded_block)
    }
    
    /// XORs the specified source blocks together
    fn xor_blocks(&self, indices: &HashSet<usize>) -> Vec<i32> {
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
                    println!("Source block {} length {} does not match first block length {}", idx, self.source_blocks[idx].len(), result.len());
                    panic!("Source blocks must be of the same length");
                }
                
                // XOR operation directly on i32 values
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
        let encoder = Encoder::new_default(source_blocks, Some(42));
        assert_eq!(encoder.source_block_count(), 3);
    }
    
    #[test]
    #[allow(unused_variables)]
    fn test_encoded_block_generation() {
        let source_blocks = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let source_blocks_len = source_blocks.len();
        let mut encoder = Encoder::new_default(source_blocks, Some(1));
        
        // Generate a block with a known seed for reproducibility
        for i in 2412..2430 {
            let (seed, d, indices, encoded_block) = encoder.generate_encoded_block(None);
            // println!("encoded_block = {}", encoded_block);
            println!("indices = {:?}", indices);
            println!("d = {}", d);
            println!("seed = {}", seed);
            // assert_eq!(seed, 1);
            assert!(d >= 1 && d <= source_blocks_len);
        }
        // assert_eq!(encoded_block.len(), 3);
        println!("After Iteration, Single call");
        let (seed, d, indices, encoded_block) = encoder.generate_encoded_block(None);
        println!("indices = {:?}", indices);
        println!("d = {}", d);
        println!("seed = {}", seed); 
    }
}

// function blobToBase64(blob) {
//   return new Promise((resolve, reject) => {
//     const reader = new FileReader();
//     reader.onload = () => {
//       // reader.result 的格式是 "data:application/octet-stream;base64,AAAA..."
//       // 我们通常只需要逗号后面的部分
//       const dataUrl = reader.result;
//       // 如果编码器需要纯Base64字符串（不包含Data URL前缀），则进行拆分
//       const base64:string = dataUrl.split(',')[1];
//       resolve(base64);
//     };
//     reader.onerror = reject;
//     reader.readAsDataURL(blob); // 这个方法会生成Data URL
//   })
// }

// const handleFileClick = async() => {
//   const input = document.createElement('input');
//   input.type = 'file';
//   input.onchange = async (event) => {
//     file.value = event.target.files[0];
//     const chunkSize = 1024 * 2; // 2KB   
//     chunks.value = [];
//     console.log(file.value)
    
//     // 1. 先将整个文件转换为Base64字符串
//     const fullBase64 = await blobToBase64(file.value);
    
//     // 2. 将Base64字符串转换为二进制数据
//     const binaryString = atob(fullBase64);
//     const totalBytes = binaryString.length;
    
//     // 3. 按块大小切分
//     for (let i = 0; i < totalBytes; i += chunkSize) {
//       // 获取当前块的二进制字符串
//       let chunkBinary = binaryString.substring(i, Math.min(i + chunkSize, totalBytes));
      
//       // 如果不足块大小，用空字符填充（对应0）
//       if (chunkBinary.length < chunkSize) {
//         chunkBinary = chunkBinary.padEnd(chunkSize, '\0');
//       }
      
//       // 将二进制字符串转换回Base64
//       const base64String = btoa(chunkBinary);
//       chunks.value.push(base64String);
//     }

//     console.log('文件切片数量:', chunks.value.length);
//     console.log('每个切片长度:', chunks.value[0]?.length); // 所有切片长度相等
//     console.log('文件切片:', chunks.value); 
//   };
//   input.click();
// };