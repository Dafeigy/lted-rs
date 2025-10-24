use super::prng::PRNG;
use std::collections::HashSet;

/// Encoder for Luby Transform codes
/// 
/// This encoder is responsible for generating encoded blocks from the source data blocks
/// using the Luby Transform algorithm.
pub struct Encoder {
    source_blocks: Vec<String>,
    prng: PRNG,
    k: usize,
}

impl Encoder {
    /// Creates a new Encoder with the given source blocks
    pub fn new(source_blocks: Vec<String>, delta: f64, c: f64) -> Self {
        let k = source_blocks.len();
        let prng = PRNG::new(k, delta, c);
        
        Self {
            source_blocks,
            prng,
            k,
        }
    }
    
    /// Creates a new Encoder with default parameters
    pub fn new_default(source_blocks: Vec<String>, seed: Option<i64>) -> Self {
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
    pub fn generate_encoded_block(&mut self, seed: Option<i64>) -> (i64, usize, HashSet<usize>,String) {
        // Use the PRNG to get source block indices
        let (blockseed, d, indices) = self.prng.get_src_blocks(seed);
        
        // XOR the selected source blocks
        let encoded_block = self.xor_blocks(&indices);
        
        (blockseed, d, indices, encoded_block)
    }
    
    /// XORs the specified source blocks together
    fn xor_blocks(&self, indices: &HashSet<usize>) -> String {
        if indices.is_empty() {
            return String::new();
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
                
                // Convert strings to bytes for XOR operation
                let mut result_bytes = result.as_bytes().to_vec();
                let other_bytes = self.source_blocks[idx].as_bytes();
                
                // XOR operation
                for i in 0..result_bytes.len() {
                    result_bytes[i] ^= other_bytes[i];
                }
                
                // Convert back to string
                result = String::from_utf8_lossy(&result_bytes).to_string();
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
        let source_blocks = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];
        let encoder = Encoder::new_default(source_blocks, Some(42));
        assert_eq!(encoder.source_block_count(), 3);
    }
    
    #[test]
    #[allow(unused_variables)]
    fn test_encoded_block_generation() {
        let source_blocks = vec![
            "IyMg5raI5oGv6Zif5YiXDQoNCua2iOaBr+mYn+WIl+mmluWFiOaYr+S4gOS4qumYn+WIl++8jOWcqOaVsOaNrue7k+aehOS4iuimgemBteW+quWFiOi/m+WFiOWHukZJRk/nmoTljp/liJnvvIznhLblkI7miY3mmK/mtojmga/nmoTmib/ovb3lrrnlmajjgILmtojmga/pmJ/liJfopoHop6PlhrPnmoTkuI3mmK/mtojmga/lrZjmlL7nmoTpl67popjvvIzogIzmmK/op6PlhrPpgJrkv6HnmoTpl67popjjgILlkIzmraXnmoTpgJrkv6HpnIDopoHpgLvovpHkuYvpl7Tpobrluo/miafooYzvvIzogJfml7bkuYXmiJDlip/njofmnInlvbHlk43jgILov5nlsLHlvJXnlLPlh7rmlrDnmoTpgJrkv6HmnLrliLbvvJrmtojmga/pmJ/liJfjgIINCg0K5b2T55So5oi36K+35rGC5Yiw5p2l5pe277yM5aaC5p6c5bCG5YW25pS+5Yiw5LiA5Liq5Lit6L2s56uZ77yM5a+55LqO55So5oi36ICM6KiA5LuW5LiN6ZyA6KaB5YWz5b+D6K+35rGC6IOM5ZCO55qE5a6e546w6YC76L6R77yM5bm25LiU5Y+q6ZyA6KaB5YWz5rOo6K+35rGC55qE57uT5p6c44CC5YW26K+35rGC6IOM5ZCO55qE57mB55CQ5Lia5Yqh6YC76L6R6YCa6L+H6K6i6ZiF5Lit6L2s56uZ55qE5raI5oGv5p2l5omn6KGM77yM5LiN5Lya5Y+X5LiK5ri455qE5omn6KGM5oiQ5Yqf5LiO5ZCm5b2x5ZON44CC6L+Z56eN5bCx5LiN5piv5ZCM5q2l6YCa5L+h5pa55byP77yM6ICM5piv5byC5q2l6YCa5L+h44CC5byC5q2l5Y+v5Lul5a+55qih5Z2X5LmL6Ze055qE5Yqf6IO96L+b6KGM6Kej6ICm77yM5LiK5ri455qE55So5oi36K+35rGC5omn6KGM5a6M5Y+R6YCB5Lia5Yqh5raI5oGv55qE6YC76L6R5ZCO5bCx5Y+v5Lul6I635Y+W57uT5p6c77yM5LiL5ri455qE5qih5Z2X5Zyo6K6i6ZiF5b6X5Yiw5LiK5ri455qE5Lia5Yqh5raI5oGv5ZCO5ZCE6Ieq5raI6LS544CC6YCa6L+H5raI5oGv6Zif5YiX77yM5bGP6JS95bqV5bGC55qE6YCa5L+h5Y2P6K6u77yM5L2/5b6X6Kej6ICm5ZKM5bm26KGM5raI6LS55b6X5Lul5a6e546w44CCDQoNCiMjIOaetuaehA0KDQrmtojmga/pmJ/liJfmnKzotKjkuIrmmK/kuIDkuKrnsbvkvLzpk77ooajnmoTni6znq4vov5vnqIvvvIzpk77ooajph4zpnaLnmoTmr4/kuIDkuKroioLngrnpg73mmK/kuIDkuKrmtojmga/jgILku5bku4vkuo7nlJ/kuqfogIXlkozmtojotLnogIXkuYvpl7TvvIzlnKjmtYHph4/pq5jls7Dml7blhYjmmoLlrZjmtojmga/mlbDmja7vvIznhLblkI7lnKjpgJDmraXmtojotLnmtojmga/mlbDmja7vvIzlj6/ku6Xoia/lpb3nmoTkv53miqTmtojmga/nmoTmtojotLnogIXvvIzov5nkuKrov4fnqIvkuZ/ooqvnp7DkuLrigJ3liYrls7DloavosLfigJzjgIINCg0KIyMgVlNUT+eahOeugOWNleWunueOsA0KDQrmiJHku6zlj6/ku6XlsIYNCg0K5pWw5o2u57uT5p6E77yaDQoNCmBgYGMjDQogICAgICAgIHB1YmxpYyBjbGFzcyBSZWNvcmRSZXNwb25zZQ0KICAgICAgICB7DQogICAgICAgICAgICBwdWJsaWMgYm9vbCBTdWNjZXNzIHsgZ2V0OyBzZXQ7IH0NCiAgICAgICAgfQ0KICAgICAgICAvLyBBbm5vdGF0aW9u5pWw5o2u57uT5p6EDQogICAgICAgIHB1YmxpYyBjbGFzcyBBbm5vdGF0aW9uUmVxdWVzdA0KICAgICAgICB7DQogICAgICAgICAgICBwdWJsaWMgTGlzdDxBbm5vdGF0aW9uUmVjb3JkPiBSZWNvcmRzIHsgZ2V0OyBzZXQ7IH0NCiAgICAgICAgfQ0KDQogICAgICAgIHB1YmxpYyBjbGFzcyBBbm5vdGF0aW9uUmVjb3JkDQogICAgICAgIHsNCiAgICAgICAgICAgIHB1YmxpYyBzdHJpbmcgUmVzdWx0IHsgZ2V0OyBzZXQ7IH0NCiAgICAgICAgICAgIHB1YmxpYyBzdHJpbmcgQ29udGVudCB7IGdldDsgc2V0OyB9DQogICAgICAgICAgICBwdWJsaWMgc3RyaW5nIFN1Z2dlc3Rpb24geyBnZXQ7IHNldDsgfQ0KICAgICAgICB9DQogICAgICAgIHB1YmxpYyBjbGFzcyBMb2NhdGVSZXF1ZXN0DQogICAgICAgIHsNCiAgICAgICAgICAgIHB1YmxpYyBzdHJpbmcgU2VhcmNoVGV4dCB7IGdldDsgc2V0OyB9DQogICAgICAgIH0NCi8vIEZpbmQgc3BlY2lmaWMgdGV4dCBhbmQgcmV0dXJuIGEgYFdvcmQuUmE=".to_string(),
            "bmdlYCBjbGFzcyBvciBudWxsLiBEb250IGNoYW5nZSB0aGlzIHVubGVzcyB5b3UgdW5kZXJzdGFuZCB3aGF0IGFyZSB5b3UgZG9pbmcNCnByaXZhdGUgc3RhdGljIFdvcmQuUmFuZ2UgRmluZEZpcnN0VGV4dChXb3JkLlJhbmdlIHJhbmdlLCBzdHJpbmcgZmluZFRleHQpDQp7DQogICAgcmFuZ2UuRmluZC5DbGVhckZvcm1hdHRpbmcoKTsNCiAgICByYW5nZS5GaW5kLlRleHQgPSBmaW5kVGV4dDsNCiAgICByYW5nZS5GaW5kLk1hdGNoV2hvbGVXb3JkID0gdHJ1ZTsNCiAgICByYW5nZS5GaW5kLk1hdGNoQ2FzZSA9IHRydWU7DQogICAgaWYgKHJhbmdlLkZpbmQuRXhlY3V0ZSgpKQ0KICAgIHsNCiAgICAgICAgcmV0dXJuIHJhbmdlOw0KICAgIH0NCiAgICByZXR1cm4gbnVsbDsNCn0NCi8vIEFic3RyYWN0IHJlc3BvbnNlIGJlaHZhaW91ciBmb3IgIGZyb250ZW5kIHJlc3BvbnNlIGRlYnVnLiBDaGFuZ2UgaXQgd2hhdGV2ZXIgeW91IHdhbnRzDQpwcml2YXRlIHZvaWQgU2VuZFJlc3BvbnNlKEh0dHBMaXN0ZW5lckNvbnRleHQgY29udGV4dCwgSHR0cFN0YXR1c0NvZGUgc3RhdHVzQ29kZSwgc3RyaW5nIG1lc3NhZ2UpDQp7DQogICAgYnl0ZVtdIGJ1ZmZlciA9IEVuY29kaW5nLlVURjguR2V0Qnl0ZXMobWVzc2FnZSk7DQogICAgY29udGV4dC5SZXNwb25zZS5TdGF0dXNDb2RlID0gKGludClzdGF0dXNDb2RlOw0KICAgIGNvbnRleHQuUmVzcG9uc2UuQ29udGVudFR5cGUgPSAiYXBwbGljYXRpb24vanNvbiI7DQogICAgY29udGV4dC5SZXNwb25zZS5Db250ZW50TGVuZ3RoNjQgPSBidWZmZXIuTGVuZ3RoOw0KICAgIGNvbnRleHQuUmVzcG9uc2UuT3V0cHV0U3RyZWFtLldyaXRlKGJ1ZmZlciwgMCwgYnVmZmVyLkxlbmd0aCk7DQogICAgY29udGV4dC5SZXNwb25zZS5DbG9zZSgpOw0KfQ0KYGBgDQoNCg0KDQrmt7vliqDms6jph4rnmoTlrp7njrDvvJoNCg0KYGBgYyMNCnByaXZhdGUgdm9pZCBQcm9jZXNzQW5ub3RhdGlvblJlcXVlc3QoSHR0cExpc3RlbmVyQ29udGV4dCBjb250ZXh0LCBBbm5vdGF0aW9uUmVxdWVzdCByZXF1ZXN0KQ0Kew0KICAgIGludCBwcm9jZXNzZWRDb3VudCA9IDA7DQogICAgX3dvcmRBcHAuU2NyZWVuVXBkYXRpbmcgPSBmYWxzZTsNCiAgICB2YXIgcmVzcG9uc2VEYXRhID0gbmV3IExpc3Q8UmVjb3JkUmVzcG9uc2U+KCk7DQogICAgdHJ5DQogICAgew0KICAgICAgICBfd29yZEFwcC5TY3JlZW5VcGRhdGluZyA9IGZhbHNlOw0KICAgICAgICBXb3JkLkRvY3VtZW50IGRvYyA9IF93b3JkQXBwLkFjdGl2ZURvY3VtZW50Ow0KDQogICAgICAgIGZvcmVhY2ggKHZhciByZWNvcmQgaW4gcmVxdWVzdC5SZWNvcmRzKQ0KICAgICAgICB7DQogICAgICAgICAgICBXb3JkLlJhbmdlIGZvdW5kUmFuZ2UgPSBGaW5kRmlyc3RUZXh0KGRvYy5Db250ZW50LCByZWNvcmQuQ29udGVudCk7DQogICAgICAgICAgICBpZiAoZm91bmRSYW5nZSA9PSBudWxsKQ0KICAgICAgICAgICAgew0KICAgICAgICAgICAgICAgIHJlc3BvbnNlRGF0YS5BZGQobmV3IFJlY29yZFJlc3BvbnNlDQogICAgICAgICAgICAgICAgew0KICAgICAgICAgICAgICAgICAgICBTdWNjZXNzID0gZmFsc2UsDQogICAgICAgICAgICAgICAgfSk7DQogICAgICAgICAgICAgICAgRGVidWcuV3JpdGVMaW5lKCQiW3tyZWNvcmQuQ29udGVudH1dIE5vdCBmb3VuZCIpOw0KICAgICAgICAgICAgICAgIA0KICAgICAgICAgICAgICAgIGNvbnRpbnVlOw0KICAgICAgICAgICAgfQ0KICAgICAgICAgICAgDQoNCiAgICAgICAgICAgIC8vIDIuIOa3u+WKoOaJueazqA0KICAgICAgICAgICAgdHJ5DQogICAgICAgICAgICB7DQogICAgICAgICAgICAgICAgc3RyaW5nIGNvbW1lbnRTdWdnZXN0aW9uID0gIiI7DQogICAgICAgICAgICAgICAgaWYgKHJlY29yZC5TdWdnZXN0aW9uICE9IG51bGwpDQogICAgICAgICAgICAgICAgew0KICAgICAgICAgICAgICAgICAgICBjb21tZW50U3VnZ2VzdGlvbiA9IHJlY29yZC5TdWdnZXN0aW9uOw0KICAgICAgICAgICAgICAgIH0NCiAgICAgICAgICAgICAgICBlbHNlDQogICAgICAgICAgICAgICAgew0KICAgICA=".to_string(),
            "ICAgICAgICAgICAgICAgY29tbWVudFN1Z2dlc3Rpb24gPSAi56ym5ZCI6K+E5a6h6KeE5YiZ77yM5peg5L+u5pS55bu66K6u44CCIjsNCiAgICAgICAgICAgICAgICB9DQogICAgICAgICAgICAgICAgc3RyaW5nIGNvbW1lbnRUZXh0ID0gJCLlrqHmoLjnu5PmnpzvvJpcbntyZWNvcmQuUmVzdWx0fVxuXG7kv67mlLnlu7rorq7vvJpcbntjb21tZW50U3VnZ2VzdGlvbn0iOw0KICAgICAgICAgICAgICAgIA0KICAgICAgICAgICAgICAgIENvbW1lbnQgY29tbWVudCA9IGRvYy5Db21tZW50cy5BZGQoZm91bmRSYW5nZSwgY29tbWVudFRleHQpOw0KICAgICAgICAgICAgICAgIGNvbW1lbnQuQXV0aG9yID0gImczNjU2IjsNCiAgICAgICAgICAgICAgICAvL0RlYnVnLldyaXRlTGluZSgkIlt7cmVjb3JkLkNvbnRlbnR9XSBGb3VuZCEiKTsNCiAgICAgICAgICAgICAgICByZXNwb25zZURhdGEuQWRkKG5ldyBSZWNvcmRSZXNwb25zZQ0KICAgICAgICAgICAgICAgIHsNCiAgICAgICAgICAgICAgICAgICAgU3VjY2VzcyA9IHRydWUsDQogICAgICAgICAgICAgICAgfSk7DQogICAgICAgICAgICAgICAgcHJvY2Vzc2VkQ291bnQrKzsNCiAgICAgICAgICAgIH0NCiAgICAgICAgICAgIGNhdGNoIChFeGNlcHRpb24gZSkNCiAgICAgICAgICAgIHsNCg0KICAgICAgICAgICAgICAgIERlYnVnLldyaXRlTGluZSgkIua3u+WKoOazqOmHilt7cmVjb3JkLkNvbnRlbnR9XSDlpLHotKXjgIIiKTsNCiAgICAgICAgICAgICAgICBEZWJ1Zy5Xcml0ZUxpbmUoZSk7DQogICAgICAgICAgICB9DQogICAgICAgICAgICANCiAgICAgICAgfQ0KDQogICAgICAgIA0KICAgIH0NCiAgICBmaW5hbGx5DQogICAgew0KICAgICAgICBfd29yZEFwcC5TY3JlZW5VcGRhdGluZyA9IHRydWU7DQogICAgICAgIERlYnVnLldyaXRlTGluZSgiU2V0IEFubm90YXRpb24gaXMgZG9uZS4iKTsNCiAgICAgICAgU2VuZFJlc3BvbnNlKGNvbnRleHQsIEh0dHBTdGF0dXNDb2RlLk9LLCBKc29uQ29udmVydC5TZXJpYWxpemVPYmplY3QobmV3DQogICAgICAgIHsNCiAgICAgICAgICAgIFN1Y2Nlc3MgPSB0cnVlLA0KICAgICAgICAgICAgUHJvY2Vzc2VkQ291bnQgPSBwcm9jZXNzZWRDb3VudCwNCiAgICAgICAgICAgIEZhaWxlZENvdW50ID0gcmVxdWVzdC5SZWNvcmRzLkNvdW50IC0gcHJvY2Vzc2VkQ291bnQsDQogICAgICAgIH0pKTsNCiAgICB9DQp9DQpgYGANCg0KDQoNCg0KDQpIYW5kbGVMb2NhdGVSZXF1ZXN077ya5aSE55CG6Lez6L2s55qE5a6e546w77yaDQoNCmBgYGMjDQpwcml2YXRlIHZvaWQgSGFuZGxlTG9jYXRlUmVxdWVzdChIdHRwTGlzdGVuZXJDb250ZXh0IGNvbnRleHQsIExvY2F0ZVJlcXVlc3QgcmVxdWVzdCkNCnsNCiAgICB0cnkNCiAgICB7DQogICAgICAgIF93b3JkQXBwLlNjcmVlblVwZGF0aW5nID0gZmFsc2U7DQoNCiAgICAgICAgV29yZC5Eb2N1bWVudCBkb2MgPSBfd29yZEFwcC5BY3RpdmVEb2N1bWVudDsNCiAgICAgICAgUmFuZ2Ugc2VhcmNoUmFuZ2UgPSBkb2MuQ29udGVudDsNCiAgICAgICAgUmFuZ2UgcmVzdWx0UmFuZ2UgPSBGaW5kRmlyc3RUZXh0KHNlYXJjaFJhbmdlLCByZXF1ZXN0LlNlYXJjaFRleHQpOw0KDQogICAgICAgIGlmIChyZXN1bHRSYW5nZSAhPSBudWxsKQ0KICAgICAgICB7DQogICAgICAgICAgICBkb2MuQWN0aXZlV2luZG93LlNjcm9sbEludG9WaWV3KHJlc3VsdFJhbmdlKTsNCg0KICAgICAgICAgICAgU2VuZFJlc3BvbnNlKGNvbnRleHQsIEh0dHBTdGF0dXNDb2RlLk9LLCAie1wic3VjY2Vzc1wiOnRydWUsIFwiZm91bmRcIjp0cnVlfSIpOw0KICAgICAgICB9DQogICAgICAgIGVsc2UNCiAgICAgICAgew0KICAgICAgICAgICAgU2VuZFJlc3BvbnNlKGNvbnRleHQsIEh0dHBTdGF0dXNDb2RlLk9LLCAie1wic3VjY2Vzc1wiOnRydWUsIFwiZm91bmRcIjpmYWxzZX0iKTsNCiAgICAgICAgfQ0KICAgIH0NCiAgICBmaW5hbGx5DQogICAgew0KICAgICAgICBfd29yZEFwcC5TY3JlZW5VcGRhdGluZyA9IHRydWU7DQogICAgfQ0KfQ0KYGBgDQoNCg0KDQrmlbTkvZPlpITnkIbvvJoNCg0KYGBgYyMNCiA=".to_string(),
            "ICAgICAgIHByaXZhdGUgdm9pZCBIYW5kbGVSZXF1ZXN0KElBc3luY1Jlc3VsdCByZXN1bHQpDQogICAgICAgIHsNCiAgICAgICAgICAgIHZhciBjb250ZXh0ID0gX2xpc3RlbmVyLkVuZEdldENvbnRleHQocmVzdWx0KTsNCiAgICAgICAgICAgIF9saXN0ZW5lci5CZWdpbkdldENvbnRleHQoSGFuZGxlUmVxdWVzdCwgbnVsbCk7IC8vIOaMgee7reebkeWQrA0KDQogICAgICAgICAgICB0cnkNCiAgICAgICAgICAgIHsNCiAgICAgICAgICAgICAgICAvLyBDT1JTIFNldHRpbmdzDQogICAgICAgICAgICAgICAgY29udGV4dC5SZXNwb25zZS5IZWFkZXJzLkFkZCgiQWNjZXNzLUNvbnRyb2wtQWxsb3ctT3JpZ2luIiwgIioiKTsNCiAgICAgICAgICAgICAgICBjb250ZXh0LlJlc3BvbnNlLkhlYWRlcnMuQWRkKCJBY2Nlc3MtQ29udHJvbC1BbGxvdy1NZXRob2RzIiwgIkdFVCwgUE9TVCwgUFVULCBERUxFVEUiKTsgLy8g5YWB6K6455qE5pa55rOVDQogICAgICAgICAgICAgICAgY29udGV4dC5SZXNwb25zZS5IZWFkZXJzLkFkZCgiQWNjZXNzLUNvbnRyb2wtQWxsb3ctSGVhZGVycyIsICJDb250ZW50LVR5cGUiKTsgLy8g5YWB6K6455qE5aS0DQogICAgICAgICAgICAgICAgLy8gUHJlZmxpZ2h0IENoZWNrDQogICAgICAgICAgICAgICAgaWYgKGNvbnRleHQuUmVxdWVzdC5IdHRwTWV0aG9kID09ICJPUFRJT05TIikNCiAgICAgICAgICAgICAgICB7DQogICAgICAgICAgICAgICAgICAgIGNvbnRleHQuUmVzcG9uc2UuU3RhdHVzQ29kZSA9IChpbnQpSHR0cFN0YXR1c0NvZGUuT0s7DQogICAgICAgICAgICAgICAgICAgIGNvbnRleHQuUmVzcG9uc2UuQ2xvc2UoKTsNCiAgICAgICAgICAgICAgICAgICAgcmV0dXJuOw0KICAgICAgICAgICAgICAgIH0NCg0KICAgICAgICAgICAgICAgIGlmIChjb250ZXh0LlJlcXVlc3QuSHR0cE1ldGhvZCA9PSAiUE9TVCIpDQogICAgICAgICAgICAgICAgew0KICAgICAgICAgICAgICAgICAgICB2YXIganNvbiA9IG5ldyBTeXN0ZW0uSU8uU3RyZWFtUmVhZGVyKGNvbnRleHQuUmVxdWVzdC5JbnB1dFN0cmVhbSkuUmVhZFRvRW5kKCk7DQoNCiAgICAgICAgICAgICAgICAgICAgLy8g5qC55o2u6Lev5b6E6Lev55Sx6K+35rGCDQogICAgICAgICAgICAgICAgICAgIGlmIChjb250ZXh0LlJlcXVlc3QuVXJsLkFic29sdXRlUGF0aC5TdGFydHNXaXRoKCIvQW5ub3RhdGlvbi8iKSkNCiAgICAgICAgICAgICAgICAgICAgew0KICAgICAgICAgICAgICAgICAgICAgICAgdmFyIHJlcXVlc3QgPSBKc29uQ29udmVydC5EZXNlcmlhbGl6ZU9iamVjdDxBbm5vdGF0aW9uUmVxdWVzdD4oanNvbik7DQogICAgICAgICAgICAgICAgICAgICAgICBQcm9jZXNzQW5ub3RhdGlvblJlcXVlc3QoY29udGV4dCwgcmVxdWVzdCk7DQogICAgICAgICAgICAgICAgICAgIH0NCiAgICAgICAgICAgICAgICAgICAgZWxzZSBpZiAoY29udGV4dC5SZXF1ZXN0LlVybC5BYnNvbHV0ZVBhdGguU3RhcnRzV2l0aCgiL2xvY2F0ZS8iKSkNCiAgICAgICAgICAgICAgICAgICAgew0KICAgICAgICAgICAgICAgICAgICAgICAgdmFyIHJlcXVlc3QgPSBKc29uQ29udmVydC5EZXNlcmlhbGl6ZU9iamVjdDxMb2NhdGVSZXF1ZXN0Pihqc29uKTsNCiAgICAgICAgICAgICAgICAgICAgICAgIEhhbmRsZUxvY2F0ZVJlcXVlc3QoY29udGV4dCwgcmVxdWVzdCk7DQogICAgICAgICAgICAgICAgICAgIH0NCiAgICAgICAgICAgICAgICAgICAgZWxzZQ0KICAgICAgICAgICAgICAgICAgICB7DQogICAgICAgICAgICAgICAgICAgICAgICB0aHJvdyBuZXcgRXhjZXB0aW9uKCLml6DmlYjnmoTor7fmsYLot6/lvoQiKTsNCiAgICAgICAgICAgICAgICAgICAgfQ0KICAgICAgICAgICAgICAgIH0NCiAgICAgICAgICAgIH0NCiAgICAgICAgICAgIGNhdGNoIChFeGNlcHRpb24gZXgpDQogICAgICAgICAgICB7DQogICAgICAgICAgICAgICAgdmFyIGVycm9yID0gRW5jb2RpbmcuVVRGOC5HZXRCeXRlcygkInt7XCJlcnJvclwiOlwie2V4Lk1lc3NhZ2V9XCJ9fSIpOw0KICAgICAgICAgICAgICAgIERlYnVnLldyaXRlTGluZSgkIlZTVE/jgIA=".to_string(),
            "U2VydmVyRXJyb3I6e2Vycm9yfSIpOw0KICAgICAgICAgICAgfQ0KICAgICAgICAgICAgZmluYWxseQ0KICAgICAgICAgICAgew0KICAgICAgICAgICAgICAgIGNvbnRleHQuUmVzcG9uc2UuQ2xvc2UoKTsNCiAgICAgICAgICAgIH0NCiAgICAgICAgfQ0KYGBgDQoNCgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_string()
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