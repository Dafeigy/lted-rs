use super::prng::PRNG;
#[allow(unused,unused_imports)]
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use base64::{Engine as _, engine::general_purpose};

#[allow(unused,unused_imports)]
pub struct LtEncoder {
    file_name: String,
    block_size: usize,
    prng: PRNG,
    f_bytes: String, // base64编码的文件内容
}

impl LtEncoder {
    pub fn new(file_name: String, block_size: usize, prng: PRNG) -> Self {
        // 检查文件是否存在
        let metadata = std::fs::metadata(&file_name).expect("文件不存在");
        if !metadata.is_file() {
            panic!("提供的路径不是一个文件");
        }

        // 打开文件并进行base64编码
        let f_bytes = match Self::_read_file_and_encode_base64_static(&file_name) {
            Ok(encoded) => encoded,
            Err(e) => panic!("读取文件失败: {}", e),
        };

        Self {
            file_name,
            block_size,
            prng,
            f_bytes,
        }
    }
    
    /// 静态方法：读取文件内容并将其转换为base64编码字符串
    fn _read_file_and_encode_base64_static(file_path: &str) -> Result<String, std::io::Error> {
        // 打开文件
        let mut file = File::open(file_path)?;
        
        // 读取文件内容到Vec<u8>
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        // 使用base64编码
        let encoded = general_purpose::STANDARD.encode(buffer);
        Ok(encoded)
    }

    pub fn _split_base64_into_blocks(&self, base64_str: &str) -> Vec<String> {
        base64_str.chars()
            .collect::<Vec<_>>()
            .chunks(self.block_size)
            .map(|chunk| chunk.iter().collect())
            .collect()
    }
    
    /// 实例方法：获取文件的base64编码内容
    pub fn get_f_bytes(&self) -> &String {
        &self.f_bytes
    }
    
    /// 实例方法：读取文件内容并将其转换为base64编码字符串（保留向后兼容性）
    pub fn _read_file_and_encode_base64(&self) -> Result<String, std::io::Error> {
        Ok(self.f_bytes.clone())
    }

    pub fn _set_f_bytes(&mut self, f_bytes: String) {
        self.f_bytes = f_bytes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_and_encode_base64() {
        // 使用new方法创建实例，这样会正确初始化f_bytes
        let encoder = LtEncoder::new(
            "test.txt".to_string(),
            1024,
            PRNG::new_default(10),
        );
        
        let result = encoder._read_file_and_encode_base64();
        assert!(result.is_ok());
        println!("\n\nSuccess To base64 encode the file: {}\nBase 64 results: {}", &encoder.file_name, result.as_ref().unwrap());
        let encoded = result.unwrap();
        assert!(!encoded.is_empty());
        
        // 测试get_f_bytes方法
        let f_bytes = encoder.get_f_bytes();
        assert_eq!(encoded, *f_bytes);
    }

    #[test]
    fn test_set_f_bytes() {
        let mut encoder = LtEncoder::new(
            "test.txt".to_string(),
            1024,
            PRNG::new_default(10),
        );
        
        let new_f_bytes = "new_base64_encoded_string".to_string();
        encoder._set_f_bytes(new_f_bytes.clone());
        
        let f_bytes = encoder.get_f_bytes();
        assert_eq!(new_f_bytes, *f_bytes);
    }
}