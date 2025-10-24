use wasm_bindgen::prelude::*;
use js_sys::Array;
use std::convert::TryFrom;
use std::collections::HashSet;

// Re-export the luby_transform module
pub mod luby_transform;

// Re-export PRNG and related functions for backward compatibility
pub use luby_transform::prng::PRNG;
pub use luby_transform::prng::{gen_tau, gen_rho, gen_mu, gen_rsd_cdf, DEFAULT_C, DEFAULT_DELTA};
pub use luby_transform::encoder::Encoder;
pub use luby_transform::decoder::Decoder;

#[wasm_bindgen]
pub struct LubyTransformEncoder {
    encoder: Encoder,
}

#[wasm_bindgen]
pub struct LubyTransformDecoder {
    decoder: Decoder,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct EncodedBlock {
    seed: i64,
    degree: usize,
    data: String,
}

#[wasm_bindgen]
impl EncodedBlock {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: i64, degree: usize, data: String) -> Self {
        Self {
            seed,
            degree,
            data,
        }
    }
    
    #[wasm_bindgen(getter)]
    pub fn seed(&self) -> i64 {
        self.seed
    }
    
    #[wasm_bindgen(getter)]
    pub fn degree(&self) -> usize {
        self.degree
    }
    
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> String {
        self.data.clone()
    }
}

#[wasm_bindgen]
impl LubyTransformEncoder {
    #[wasm_bindgen(constructor)]
    pub fn new(source_blocks: Vec<String>, seed: Option<i64>) -> Self {
        Self {
            encoder: Encoder::new_default(source_blocks, seed),
        }
    }
    
    pub fn generate_block(&mut self, seed: Option<i64>) -> EncodedBlock {
        let (blockseed, d, _, encoded_block) = self.encoder.generate_encoded_block(seed);
        EncodedBlock::new(blockseed, d, encoded_block)
    }
    
    pub fn source_block_count(&self) -> usize {
        self.encoder.source_block_count()
    }
}

#[wasm_bindgen]
impl LubyTransformDecoder {
    #[wasm_bindgen(constructor)]
    pub fn new(k: usize, block_size: usize) -> Self {
        Self {
            decoder: Decoder::new_default(k, block_size),
        }
    }
    
    pub fn add_encoded_block(&mut self, seed: i64, degree: usize, data: String) -> usize {
        self.decoder.add_encoded_block(seed, degree, data)
    }
    
    pub fn decoded_count(&self) -> usize {
        self.decoder.decoded_count()
    }
    
    pub fn is_complete(&self) -> bool {
        self.decoder.is_complete()
    }
    
    pub fn get_all_decoded_blocks(&self) -> Option<Array> {
        if let Some(blocks) = self.decoder.get_all_decoded_blocks() {
            let js_array = Array::new();
            for block in blocks {
                js_array.push(&JsValue::from_str(&block));
            }
            Some(js_array)
        } else {
            None
        }
    }
    
    pub fn current_round(&self) -> usize {
        self.decoder.current_round()
    }
}

#[wasm_bindgen]
pub fn encode_file_blocks(blocks: Vec<String>, seed: Option<i64>, num_encoded_blocks: usize) -> Array {
    let mut encoder = LubyTransformEncoder::new(blocks, seed);
    let result = Array::new();
    
    for _ in 0..num_encoded_blocks {
        let block = encoder.generate_block(None);
        result.push(&JsValue::from(block));
    }
    
    result
}

#[wasm_bindgen]
pub fn init() {
    // This function can be called to ensure the wasm module is initialized properly
    // It can also be used to perform any setup tasks if needed
}

// Remove default export of function pointer as wasm-bindgen doesn't support it
