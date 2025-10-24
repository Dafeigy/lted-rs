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
            "IyBEb2N1bWVudCBiYXNlZCBNZXNzYWdlIFF1ZXVlDQoNCueUseS6juS4gOWghuS5seS4g+WFq+ezn+eahOWOn+WboOaAu+S5iyBSRVNURlVMIOaOpeWPo+S4jeWcqOmZouWGheWcuuaZr+mAgueUqO+8jOWboOatpOmHh+WPluaWsOeahOaWueW8j+aehOW7uldlYi1BZGQtSW7lkoxWU1RP6YCa5L+h55qE5pa55byP44CC5a6e546w5LqM6ICF6YCa5L+h5YW25a6e5piv5om+5Yiw5LqM6ICF6IO95Yqb55qE5Lqk6ZuG77yM5oiR55qE5a6e546w5piv5Z+65LqOV29yZO+8iOWunumZheS4ikV4Y2Vs5Lmf5pyJ77yJ6YeM6Z2iIkN1c3RvbVByb3BlcnRpZXMi6L+Z5LiA5LiN5pi+5byP5ZGI546w57uZ55So5oi355qE5paH5qGj5bGe5oCn77yM6YCa6L+H5LiN5ZCM55qEQ3VzdG9tUHJvcGVydGllc+WumuS5ieeahOaVsOaNrumAmumBk++8iOS6i+WunuS4iu+8jOWPquaYr+WNleS4gOS4gOS4quWAvO+8jOW5tuayoeaciee8k+WtmOOAgemYn+WIl+acuuWItu+8iei/m+ihjOmAmuS/oeeahOOAguWFs+S6jkN1c3RvbVByb3BlcnRpZXPvvIzlj6/ku6Xlj4LogINb5a6Y5pa55paH5qGjXShodHRwczovL2xlYXJuLm1pY3Jvc29mdC5jb20vemgtY24vZG90bmV0L2FwaS9taWNyb3NvZnQub2ZmaWNlLmludGVyb3Aud29yZC5jdXN0b21wcm9wZXJ0eT92aWV3PXdvcmQtcGlhKeeahOS7i+e7je+8jOi/memHjOS4jeWGjei1mOi/sOOAguaOpeS4i+adpeWwhuS7i+e7jeaetuaehOiuvuiuoeS7peWPiuWunueOsOOAgg0KDQojIyDmnrbmnoTorr7orqENCg0K5oyJ54WnQ1PmnrbmnoTov5vooYzliJLliIbnmoTor53vvIxWU1RP56uv5YW25a6e5omN5piv5Lil5qC85oSP5LmJ5LiK55qE5pyN5Yqh56uv77yM6ICMV2ViQWRkSW7liJnmmK/lj5Hotbfor7fmsYLnmoTlrqLmiLfnq6/jgILpppblhYjlhYjku4vnu41ETVHnmoTmoLjlv4Pov5DkvZznmoTkuIDkupvlip/og73jgILlnKhETVHnmoTmnrbmnoTph4zpnaLvvIzmtojmga/pmJ/liJfkuLvopoHov5jmmK/mmK/njrDlnKhWU1RP5L6n55qE77yI5L2G5piv5Zyo5ZCO57ut55qE6L+t5Luj5Lit77yMV2ViQWRkSW7nq6/kuZ/pnIDopoHkvb/nlKjvvInvvIzlubbkvb/nlKjkuIDkuKrnu5HlrprliLDlvZPliY3mlofmoaPnmoRUaW1lcuWunueOsOaMh+S7pOi9ruivou+8jOeUqOS6juafpeivoldlYkFkZElu5Lyg5YWl55qE5raI5oGv77yaDQoNCmBgYGMjDQpwdWJsaWMgdm9pZCBJbml0aWFsaXplVGltZXIoKQ0Kew0KICAgIFN5c3RlbS5XaW5kb3dzLkZvcm1zLlRpbWVyIHRpbWVyID0gbmV3IFN5c3RlbS5XaW5kb3dzLkZvcm1zLlRpbWVyKCk7DQogICAgdGltZXIuSW50ZXJ2YWwgPSA1MDsgLy8gNTAgbXMNCiAgICB0aW1lci5UaWNrICs9IENoZWNrRm9yQ29tbWFuZHM7DQogICAgdGltZXIuU3RhcnQoKTsNCn0NCmBgYA0K5YW25Lit55qEYENoZWNrRm9yQ29tbWFuZHNg5Ye95pWw5YiZ5piv6L2u6K+i5piv5ZCm5pyJ6ZyA6KaB5omn6KGM55qE5pON5L2c44CC5Li65LqG5ZCO57ut5ouT5bGV55qE6KeE6IyD5YyW77yM6L2u6K+i55qE5ZG95Luk44CB6L2u6K+i55qE5Y+C5pWw44CB6L2u6K+i55qE5ZON5bqU5Z2H5a2Y5pS+5LqO5YWs5YWx57G7YENvbW11bmljYXRpb25Qcm90b2NvbGDkuK3vvJoNCg0KYGBgYyMNCnB1YmxpYyBjbGFzcyBDb21tdW5pY2F0aW9uUHJvdG9jb2wNCnsNCiAgICAvLyDnlKjkuo7lrZjlgqjlkb3ku6TnmoTlsZ7mgKflkI0NCiAgICBwdWJsaWMgY29uc3Qgc3RyaW5nIENvbW1hbmRQcm9wZXJ0eSA9ICJjdXJyZW50Q29tbWFuZE5hbWUiOw0KDQogICAgLy8g55So5LqO5a2Y5YKo5Y+C5pWw55qE5bGe5oCn5ZCNDQogICAgcHVibGljIGNvbnN0IHN0cmluZyBQYXJhbXNQcm9wZXJ0eSA9ICJjdXJyZW50Q29tbWFuZFBybXMiOw0KDQogICAgLy8g55So5LqO5a2Y5YKo5ZON5bqU55qE5bGe5oCn5ZCNDQogICAgcHVibGljIGNvbnN0IHN0cmluZyBSZXNwb25zZVByb3BlcnR5ID0gImN1cnJlbnRSZXNwb25zZSI7DQoNCiAgICAvLyDlrprkuYnlj6/og73nmoTlkb3ku6QNCiAgICBwdWJsaWMgY29uc3Qgc3RyaW5nIFNldEFubm90YXRpb25Db21tYW5kID0gIlM=".to_string(), 
            "ZXRBbm5vdGF0aW9uIjsNCiAgICBwdWJsaWMgY29uc3Qgc3RyaW5nIExvY2F0ZVRleHRDb21tYW5kID0gIkxvY2F0ZVRleHQiOw0KICAgIC8vIOWFtuS7luWRveS7pOaIluS5seS4g+WFq+ezn+eahOWcqOi/memHjOe7n+S4gOeuoeeQhg0KfQ0KYGBgDQoNCuWcqOi9ruivouaXtu+8jENoZWNrRm9yQ29tbWFuZHPkvJrmn6Xor6JgQ29tbXVuaWNhdGlvblByb3RvY29sLkNvbW1hbmRQcm9wZXJ0eWDlkoxgQ29tbXVuaWNhdGlvblByb3RvY29sLlBhcmFtc1Byb3BlcnR5YOeahEN1c3RvbVByb3BlcnRpZXPvvIzlubblsIblhbbov5vooYzop6PmnpDjgIHmiafooYzvvJoNCg0KYGBgYyMNCnByaXZhdGUgdm9pZCBDaGVja0ZvckNvbW1hbmRzKG9iamVjdCBzZW5kZXIsIEV2ZW50QXJncyBlKQ0Kew0KICAgIHRyeQ0KICAgIHsNCiAgICAgICAgaWYgKEFwcGxpY2F0aW9uLkRvY3VtZW50cy5Db3VudCA9PSAwKQ0KICAgICAgICAgICAgcmV0dXJuOw0KDQogICAgICAgIFdvcmQuRG9jdW1lbnQgYWN0aXZlRG9jID0gQXBwbGljYXRpb24uQWN0aXZlRG9jdW1lbnQ7DQoNCiAgICAgICAgLy8g5qOA5p+l5piv5ZCm5pyJ5paw5ZG95LukDQogICAgICAgIHN0cmluZyBjdXJyZW50Q29tbWFuZCA9IEdldEN1c3RvbVByb3BlcnR5KENvbW11bmljYXRpb25Qcm90b2NvbC5Db21tYW5kUHJvcGVydHkpOw0KICAgICAgICBzdHJpbmcgY3VycmVudFBhcmFtcyA9IEdldEN1c3RvbVByb3BlcnR5KENvbW11bmljYXRpb25Qcm90b2NvbC5QYXJhbXNQcm9wZXJ0eSk7DQoNCiAgICAgICAgLy8g6K6h566X5b2T5YmN5ZG95Luk55qE5ZOI5biM5YC877yM55So5LqO5qOA5rWL5Y+Y5YyWDQogICAgICAgIHN0cmluZyBjdXJyZW50SGFzaCA9IENhbGN1bGF0ZUhhc2goY3VycmVudENvbW1hbmQgKyBjdXJyZW50UGFyYW1zKTsNCiAgICAgICAgRGVidWcuV3JpdGVMaW5lSWYoKGN1cnJlbnRIYXNoIT0iIiksJCJDYWxjdWxhdGVIYXNoOntjdXJyZW50SGFzaH0iKTsNCiAgICAgICAgaWYgKCFzdHJpbmcuSXNOdWxsT3JFbXB0eShjdXJyZW50Q29tbWFuZCkgJiYgY3VycmVudEhhc2ggIT0gX2xhc3RDb21tYW5kSGFzaCkNCiAgICAgICAgew0KICAgICAgICAgICAgX2xhc3RDb21tYW5kSGFzaCA9IGN1cnJlbnRIYXNoOw0KICAgICAgICAgICAgLy8g5omn6KGM5ZG95LukDQogICAgICAgICAgICBFeGVjdXRlQ29tbWFuZChjdXJyZW50Q29tbWFuZCwgY3VycmVudFBhcmFtcywgYWN0aXZlRG9jKTsNCg0KICAgICAgICAgICAgLy8g5riF56m65ZG95Luk77yM6KGo56S65bey5aSE55CGDQogICAgICAgICAgICBTZXRDdXN0b21Qcm9wZXJ0eShDb21tdW5pY2F0aW9uUHJvdG9jb2wuQ29tbWFuZFByb3BlcnR5LCAiIik7DQogICAgICAgICAgICBTZXRDdXN0b21Qcm9wZXJ0eShDb21tdW5pY2F0aW9uUHJvdG9jb2wuUGFyYW1zUHJvcGVydHksICIiKTsNCg0KICAgICAgICB9DQogICAgfQ0KICAgIGNhdGNoIChFeGNlcHRpb24gZXgpDQogICAgew0KICAgICAgICBEZWJ1Zy5Xcml0ZUxpbmUoJCLmo4Dmn6Xlkb3ku6Tml7blh7rplJk6IHtleC5NZXNzYWdlfSIpOw0KICAgIH0NCn0NCmBgYA0KDQrlk4jluIzlh73mlbBgQ2FsY3VsYXRlSGFzaChzdHJpbmcgdGV4dClg55qE5ZOI5biM5YyW55qE5a+56LGh5pivYGN1cnJlbnRDb21tYW5kICsgY3VycmVudFBhcmFtc2DvvIzkvYblrp7pmYXkuIrlnKjlhbblhbfkvZPnmoTlrp7njrDkuK3miJHlj6blpJbmt7vliqDkuobml7bpl7TmiLPkuI7lhbblkIjlubbvvIzov5nmoLflj6/ku6Xnoa7kv53lvZPnlKjmiLflpJrmrKHkvb/nlKjlkIzkuIDlkb3ku6TlkozlkIzkuIDlj4LmlbDml7bkuZ/kvJrnoa7kv53lkb3ku6Tog73ooqvmraPnoa7miafooYzvvJoNCg0KYGBgYyMNCnByaXZhdGUgc3RyaW5nIENhbGN1bGF0ZUhhc2goc3RyaW5nIG9yaWdpbnRleHQpDQp7DQogICAgaWYgKHN0cmluZy5Jc051bGxPckVtcHR5KG9yaWdpbnRleHQpKSByZXR1cm4gIiI7DQogICAgb3JpZ2ludGV4dCArPSBEYXRlVGltZU9mZnNldC5VdGNOb3cuVG9Vbml4VGltZU1pbGxpc2Vjb25kcygpLlRvU3RyaW5nKCk7DQogICAgdXNpbmc=".to_string(),
            "IChTeXN0ZW0uU2VjdXJpdHkuQ3J5cHRvZ3JhcGh5Lk1ENSBtZDUgPSBTeXN0ZW0uU2VjdXJpdHkuQ3J5cHRvZ3JhcGh5Lk1ENS5DcmVhdGUoKSkNCiAgICB7DQogICAgICAgIGJ5dGVbXSBpbnB1dEJ5dGVzID0gU3lzdGVtLlRleHQuRW5jb2RpbmcuVVRGOC5HZXRCeXRlcyhvcmlnaW50ZXh0KTsNCiAgICAgICAgYnl0ZVtdIGhhc2hCeXRlcyA9IG1kNS5Db21wdXRlSGFzaChpbnB1dEJ5dGVzKTsNCiAgICAgICAgcmV0dXJuIENvbnZlcnQuVG9CYXNlNjRTdHJpbmcoaGFzaEJ5dGVzKTsNCiAgICB9DQp9DQpgYGANCg0K5a+55LqOV2ViQWRkSW7nmoTlrqLmiLfnq6/or7fmsYLvvIzmiJHku6zkuZ/pnIDopoHlsIbor7fmsYLnmoTnu5Pmnpzov5Tlm57nu5nlrqLmiLfnq6/vvIzlhbfkvZPnmoTlrp7njrDliJnmmK/lsIZWU1RP5L6n5omn6KGM55qE57uT5p6c5a2Y5YWl5YiwYENvbW11bmljYXRpb25Qcm90b2NvbC5SZXNwb25zZVByb3BlcnR5YOeahEN1c3RvbVByb3BlcnRpZXPkuK3vvIzorqnlrqLmiLfnq6/lnKjlj5Hotbfor7fmsYLlkI7vvIzkuLvliqjljrvntKLlj5bor6XlgLzjgILmiJbogIXvvIzkuZ/lj6/ku6XogIPomZHlnKjlrqLmiLfnq6/kuZ/lkK/liqjkuIDkuKrlrprml7blmajlrprml7bmn6XnnIvor6XlgLzjgIINCg0K5pyA5ZCO6LS05LiA5LiL5YW35L2T55qE5a6e546w5qGG5p6277yaDQoNCiFbZTg4N2IyMmZlYmIzZTU4ZDE2YzU3NDVmNzJhNTBkODZdKEM6XFVzZXJzXHVzZXJcRG9jdW1lbnRzXHh3ZWNoYXRfZmlsZXNcd3hpZF83b3Y1Mnhtc2F5cm8yMl82NGJjXHRlbXBcUldUZW1wXDIwMjUtMTBcYzc1MDE3ZjZhYjJjNDNiYjFiYThlOTU3OWJkNjdlNDNcZTg4N2IyMmZlYmIzZTU4ZDE2YzU3NDVmNzJhNTBkODYucG5nKQ0KDQojIyDku6PnoIHorr7orqENCg0KIyMjIOato+e7n+iuvuiuoQ0KDQrmiJHlsL3lj6/og73lnLDlsIbmjqXlj6Por7fmsYLnmoTor63ms5XlvoDluLjop4TnmoTmjqXlj6Por7fmsYLor63ms5Xor7fmsYLvvIzlm6DmraTlnKjorr7orqHkuYvml7blsLHkvb/nlKjkuoblvILmraXnvJbnqIvmqKHlvI/ov5vooYzmjqXlj6Porr7orqHjgILmiJHlsIbmjqXlj6PosIPnlKjnmoTog73lipvlsIHoo4XlnKhgYXNzZXRzL0RNUS5qc2Dmlofku7bkuK3jgILpnIDopoHkvb/nlKjml7bvvIzpnIDopoHlnKhgaW5kZXguaHRtbGDkuK3lvJXlhaXor6Xmlofku7bvvJoNCg0KYGBgZGlmZg0KPCFET0NUWVBFIGh0bWw+DQo8aHRtbCBsYW5nPSJ6aC1jbiI+DQogIDxoZWFkPg0KICAgIDwhLS0gT2ZmaWNlIEphdmFTY3JpcHQgQVBJIC0tPg0KICAgIDxzY3JpcHQNCiAgICAgIHR5cGU9InRleHQvamF2YXNjcmlwdCINCiAgICAgIHNyYz0iaHR0cHM6Ly9hcHBzZm9yb2ZmaWNlLm1pY3Jvc29mdC5jb20vbGliLzEvaG9zdGVkL29mZmljZS5qcyINCiAgICA+PC9zY3JpcHQ+DQogICAgPG1ldGEgY2hhcnNldD0iVVRGLTgiIC8+DQorICAgPHNjcmlwdCBzcmM9Ii4vc3JjL2Fzc2V0cy9ETVEuanMiPjwvc2NyaXB0Pg0KICAgIDxsaW5rIHJlbD0iaWNvbiIgaHJlZj0iL2Zhdmljb24uaWNvIiAvPg0KICAgIDxsaW5rIHJlbD0ic3R5bGVzaGVldCIgaHJlZj0iaHR0cHM6Ly9jZG5qcy5jbG91ZGZsYXJlLmNvbS9hamF4L2xpYnMvZm9udC1hd2Vzb21lLzUuMTUuMy9jc3MvYWxsLm1pbi5jc3MiPg0KICAgIDwhLS0gPGxpbmsgcmVsPSJzdHlsZXNoZWV0IiBocmVmPSJodHRwczovL2Nkbi5qc2RlbGl2ci5uZXQvbnBtL2ZvbnQtYXdlc29tZUA0LjcuMC9jc3MvZm9udC1hd2Vzb21lLm1pbi5jc3MiPiAtLT4NCiAgICA8bWV0YSBuYW1lPSJ2aWV3cG9ydCIgY29udGVudD0id2lkdGg9ZGV2aWNlLXdpZHRoLCBpbml0aWFsLXNjYWxlPTEuMCIgLz4NCiAgICA8dGl0bGU+Vml0ZSBBcHA8L3RpdGxlPg0KICA8L2hlYWQ+DQogIDxib2R5Pg0KICAgIDxkaXYgaWQ9ImFwcCI+PC9kaXY+DQogICAgPHNjcmlwdCB0eXBlPSJtb2R1bGUiIHNyYz0iL3NyYy9tYWluLmpzIj48L3NjcmlwdD4NCiAgPC9ib2R5Pg0KPC9odG1sPg0KDQpgYGANCg0K54S25ZCO5Y2z5Y+v6LCD55So5o6l5Y+j44CC5oiR5bey57s=".to_string(),
            "j+WwveWPr+iDveWcsOWwneivleeUqOW8guatpeivt+axgueahOmAu+i+keadpeWunueOsFZTVE/nmoTmjqXlj6PosIPnlKjvvIzlpoLkuIvmmK/kuKTkuKrkvovlrZDvvJoNCg0KLSDmibnms6jnlJ/miJDvvJoNCg0KICBgYGBqYXZhc2NyaXB0DQogIHRyeSB7DQogICAgICBjb25zb2xlLmxvZygi5rWL6K+VU2V0QW5ub3RhdGlvbi4uLiIpOw0KICAgICAgY29uc3QgcmVzdWx0ID0gYXdhaXQgRE1RLlNldEFubm90YXRpb24oYXhpb3NSZXNwb25zZSk7DQogICAgICBjb25zb2xlLmxvZygiU2V0QW5ub3RhdGlvbue7k+aenDoiLCByZXN1bHQpOw0KICAgICAgfSBjYXRjaCAoZXJyb3IpIHsNCiAgICAgIGNvbnNvbGUuZXJyb3IoIua1i+ivleWksei0pToiLCBlcnJvcik7DQogICAgICB9DQogIGBgYA0KDQogIOaJueazqOeUn+aIkOmcgOimgeS9v+eUqOeahOWPguaVsOWSjOS9v+eUqHN1YlRhc2tJROivt+axguiOt+WPluivhOWuoeaOpeWPo+eahOe7k+aenOS4gOiHtO+8jOS9huWcqOWunumZheivt+axglZTVE/ml7bnmoTlj4LmlbDnu5PmnoTmjInlpoLkuIvov5vooYzlrprkuYnvvJoNCg0KICBgYGBqYXZhc2NyaXB0DQogIGNvbnN0IHBhcmFtcyA9IHsNCiAgICAgIHJlY29yZHM6IHJlc3BvbnNlLmRhdGEuZGF0YS5yZWNvcmRzLm1hcCgoaXRlbSkgPT4gKHsNCiAgICAgICAgUmVzdWx0OiBpdGVtLnJlc3VsdCwNCiAgICAgICAgQ29udGVudDogRE1RLnBhcnNlT3JpZ2luVGV4dChpdGVtLmNvbnRlbnQpLA0KICAgICAgICBTdWdnZXN0aW9uOiBpdGVtLnN1Z2dlc3Rpb24sDQogICAgICB9KSksDQogIGBgYA0KDQogIOS9oOWPr+S7peagueaNruWunumZheaDheWGteWcqGBETVEuanNg5Lit6L+b6KGM5L+u5pS544CCDQoNCi0g5paH5pys5a6a5L2N77yaDQoNCiAgYGBgamF2YXNjcmlwdA0KICBjb25zb2xlLmxvZygi5rWL6K+VTG9jYXRlVGV4dC4uLiIpDQogIGNvbnN0IHJlc3VsdCA9IGF3YWl0IERNUS5Mb2NhdGVUZXh0KHRleHRUb0xvY2F0ZSk7DQogIGNvbnNvbGUubG9nKCJMb2NhdGVUZXh057uT5p6cOiIsIHJlc3VsdCk7DQogIGBgYA0KDQogIOaWh+acrOWumuS9jeS4re+8jOWtl+espuS4smB0ZXh0VG9Mb2NhdGVg55qE6ZW/5bqm5LiN5a6c6LaF6L+HMjU277yM5ZCm5YiZ5peg5rOV6L+b6KGM6YCJ5Lit44CCDQoNCuS7peS4iuaYr+S9v+eUqOaOpeWPo+iwg+eUqOeahOaWueazle+8jOivpWNvbW1pdOeahOaetuaehOW9k+WJjeWtmOWcqOS4gOS6m2J1Z++8mldlYkFkZElu5L6n5b6X5Yiw55qE5ZON5bqU57uT5p6c5Y+v6IO96L+Y5pyq562J5b6F5paH5qGj5LiK5LiL5paHIGBhd2FpdCBjb250ZXh0LnN5bmMoKWAg5pu05paw5a6M5q+V5ZCO5bCx6K+75Y+W5LqG5pWw5o2u6YCa6YGT55qE5YC877yM5a+86Ie05a6e6ZmF55qE5ZON5bqU5Ye66ZSZ44CC5aaC5p6c6ZyA6KaB5L+d5oyB6aOO5qC85LiA6Ie077yM5Y+v5Lul5Zyo5ZCO5pyfYHZ1ZS1kZW1vYOWIhuaUr+S4iueahGNvbW1pdCBgZGYwOTZjNjVkNWI3Y2IwNWIyM2FlNDg1NDFlNDQyMTIzYWM0OGZmY2Ag6L+b6KGM5byA5Y+R44CC5Li65LqG5pyA5b+r55qE5Y+v55So77yM5oiR6L+Y5pyJ5ZCO5aSH55qE5Li05pe25YeR5ZCI5L2/55So55qE5L+u5aSN44CCDQoNCiMjIyDkuLTml7bkv67lpI0NCg0K5Li05pe25L+u5aSN6Z2e5bi4566A5Y2V57KX5pq077yaV2ViQWRkSW7nm7TmjqXlkK/liqjkuIDkuKrlrprml7blmajova7or6JSZXNwb25zZVByb3BlcnR555qE5YC877yM5L2G6ZyA6KaB5rOo5oSP55qE5piv5a6a5pe25Zmo55qE6L2u6K+i6Ze06ZqU5LiN6IO96K6+572u5b6X6L+H5bCP77yM5Zyo5oiR55qE5rWL6K+V5LitMTAwMG1z5piv5LiA5Liq5beu5LiN5aSa5ZCI6YCC55qE5YC844CC5LmL5omA5Lul6KaB6L+Z5qC36K6+572u5piv5Zug5Li6IOWcqOe8lui+keOAgeivu+WPlldvcmTnmoTmlofmoaPlsZ7mgKflhoXlrrnpnIDopoHkvb/nlKggYGNvbnRleHQuc3luYygpYCDlh73mlbDlkIzmraXmlofmoaPkuIrkuIvmlofjgILov5nkuIDlh73mlbDnmoTmiafooYzml7bpl7Tlvojpmr7noa7lrprjgIINCg0K5oC75LmL77yM5b2T5Yk=".to_string(),
            "jeWPr+eUqOeahOS7o+eggeS9v+eUqOa1geeoi+WmguS4i++8mg0KDQoxLiBBcHAudnVl5oiWbWFpbi5qcyDmlrDlu7pETVHlrp7kvovlubblkK/liqjjgILkvaDlj6/ku6XlnKjmlrDlu7pETVHlrp7kvovml7bmjIflrppkZWJ1Z+WPguaVsO+8jOWug+eUqOS6juaOp+WItuaOp+WItuWPsOaYr+WQpuaJk+WNsOavj+asoeivt+axguaJk+WNsOeahOivt+axgue7k+aenOOAgg0KDQogICBgYGBkaWZmDQogICBpbXBvcnQgIi4vYXNzZXRzL21haW4uY3NzIjsNCiAgIGltcG9ydCBFbGVtZW50UGx1cyBmcm9tICJlbGVtZW50LXBsdXMiOw0KICAgaW1wb3J0ICJlbGVtZW50LXBsdXMvZGlzdC9pbmRleC5jc3MiOw0KICAgaW1wb3J0IHsgY3JlYXRlQXBwIH0gZnJvbSAidnVlIjsNCiAgIGltcG9ydCBBcHAgZnJvbSAiLi9BcHAudnVlIjsNCiAgIA0KICAgLy8gY3JlYXRlQXBwKEFwcCkubW91bnQoJyNhcHAnKQ0KICAgT2ZmaWNlLm9uUmVhZHkoKCkgPT4gew0KICAgICBjb25zdCBhcHAgPSBjcmVhdGVBcHAoQXBwKTsNCiAgICAgYXBwLnVzZShFbGVtZW50UGx1cyk7DQogICArICBjb25zdCBXZWJBZGRJbkxpc3RlbmVyID0gbmV3IERNUSh0cnVlKTsNCiAgICsgIFdlYkFkZEluTGlzdGVuZXIuc3RhcnQoKTsNCiAgICAgYXBwLm1vdW50KCIjYXBwIik7DQogICB9KTsNCiAgIA0KICAgYGBgDQoNCjIuIGBETVEuanNgIOi/mOaPkOS+m+S6huS4pOS4quacgOWwj+e6p+WIq+eahOaTjeS9nEFQSe+8mg0KDQogICAtIGBzZXRDdXN0b21Qcm9wZXJ0eShwcm9wTmFtZSwgcHJvcFZhbHVlKWAg55So5LqO6K6+572u6Ieq5a6a5LmJ5bGe5oCn44CCYHByb3BOYW1lYOWPguaVsOaYr+W+heivu+WPlueahOaVsOaNrumAmumBk+WQjeensO+8jGBwcm9wVmFsdWVg5YiZ5piv5b6F6K6+572u55qE5YC844CC5Zyo5b2T5YmN5Li76KaB55qE5L2/55So5Zy65pmv5piv5riF56m6Q29tbXVuaWNhdGlvblByb3RvY29s6YeM6Z2i55qE5pWw5o2u6YCa6YGT77yI5q+U5aaC6K+05q+P5qyh6K+75Y+W5LqGQ29tbXVuaWNhdGlvblByb3RvY29sLlJlc3BvbnNlUHJvcGVydHnvvInlkI7muIXnqbrjgILms6jmhI/vvIwqKua4heepuuaYr+aMh+WwhuivpeaVsOaNrumAmumBk+eahOWAvOiuvuS4uuepuuWtl+espuS4smAiImDvvIzogIzpnZ7liKDpmaTor6XmlbDmja7pgJrpgZMqKuOAgg0KICAgLSBgcmVhZFByb3BlcnR5KHByb3BOYW1lLCBkZWJ1ZyA9IGZhbHNlKWDnlKjkuo7or7vlj5boh6rlrprkuYnlsZ7mgKfnmoTlgLzjgILlnKjlvZPliY3nmoTkvb/nlKjlnLrmma/kuLvopoHmmK/ojrflj5ZDb21tdW5pY2F0aW9uUHJvdG9jb2wuUmVzcG9uc2VQcm9wZXJ0eeaVsOaNrumAmumBk+eahOWAvOS7peiOt+WPluivt+axguWTjeW6lOe7k+aenOOAgmBwcm9wTmFtZWDlj4LmlbDmmK/lvoXor7vlj5bnmoTmlbDmja7pgJrpgZPlkI3np7DvvIxgZGVidWc9ZmFsc2VgIOWImeaYr+eUqOS6juaOp+WItuaYr+WQpuWwhue7k+aenOaJk+WNsOWHuuadpeOAgg0KDQogICDlpoLmnpzlkI7nu63pnIDopoHov5vooYzkuozmrKHmianlsZXvvIzlj6/ku6XlnKjmraTln7rnoYDkuIrov5vooYzjgIINCg0KIyMjIFYxLjANCg0K55Sx5LqOIFdvcmQg5o+S5Lu257uI5LqO5LiK57q/5LqG56ys5LiA5Liq54mI5pys77yMV2ViQWRkSW7nmoTkuqTkupLkvqfku6PnoIHnu4Tnu4fkuZ/mnInkuoblj5jljJbjgILlvZPliY1ETVHlnKhXZWJBZGRJbueahOa6kOeggeS4reiiq+aUvuWcqGBzcmNcaG9va3NcdXNlVlNUT0hlbHBlcmDnmoTmlofku7blpLnkuK3vvIzlt7Lmm7TnrKblkIjliY3nq6/lt6XnqIvnmoTku6PnoIHnu5PmnoTop4TojIPjgIJXZWJBZGRJbuWcqOWQr+WKqOaXtuS8muetieW+hU9mZmljZS5qc+WKoOi9veWujOavleWQjuWQr+WKqOS4gOS4qui9ruivouWZqOi9ruivouadpeiHqlZTVE/nmoTmjIfku6TvvIzova7or6LlmajlvZPliY3kvJrova7or6Lku6XkuIvlhoXlrrnvvJoNCg0KLSBgU1NPTG9naW5DYWxsYmFja2A6IOajgOa1i+eUqOaIt+eZu+W9leaAgeOAgg0KLSBgR29Ub1JvdXRlQ2FsbGJhY2tg77ya5qOA5rWL5b2T5YmN55So5oi35Yu+6YA=".to_string(),
            // "ieeahEFQUOi3r+eUseOAgg0KDQrlnKhWU1RP56uv77yM6L2u6K+i55qE5a+56LGh5bCx56iN5aSa5LiA5Lqb77yaDQoNCi0gYGN1cnJlbnRDb21tYW5kTmFtZWDvvJrlvZPliY3mjqXmlLbliLDnmoTmnaXoh6pXZWJBZGRJbueahOaMh+S7pOWQjeensOOAgg0KLSBgY3VycmVudENvbW1hbmRQcm1zYO+8muW9k+WJjeaOpeaUtuWIsOeahOadpeiHqldlYkFkZElu55qE5oyH5Luk5Y+C5pWw44CCDQotIGBjdXJyZW50VXNlckluZm9g77ya5b2T5YmN55So5oi35L+h5oGv44CCDQoNCumcgOimgeaMh+WHuu+8jGN1cnJlbnRVc2VySW5mb+WSjFNTT0xvZ2luQ2FsbGJhY2vpg73mmK/lsZ7kuo7nmbvlvZXnirbmgIHnrqHnkIbnmoTkuIDpg6jliIbvvIznhLbogIzku5bku6zlkITmnInliIblt6XjgIJjdXJyZW50VXNlckluZm/kvZznlKjlr7nosaHmmK/mlbTkuKpXb3Jk5bqU55So56iL5bqP77yM5Zug5q2k5bqU6K+l5Zyo55So5oi355m75b2V6L+H5LiA5qyh5ZCO5Zyo5omA5pyJ55qEV29yZOW6lOeUqOeoi+W6j+mHjOmDveiDveS/neeVmeatpOmhueS/oeaBr++8m+iAjFNTT0xvZ2luQ2FsbGJhY2vliJnmmK/nmbvlvZXmgIHnmoTmo4DmtYvvvIzooqvnlKjkuo7op6blj5HnmbvlvZXooYzkuLrjgIINCg0KIyMg5LiL5LiA6Zi25q61DQoNCuW9k+WJjeeahOS7o+eggeiuvuiuoeS4re+8jOi/mOS4jeiDveWBmuWIsFdlYkFkZElu55qE5YWz6Zet5qOA5rWL77yM6L+Z5piv5Zug5Li6V2ViQWRkSW7lubbkuI3mmK/lkK/liqjkuIDkuKrmtY/op4jlmajvvIzogIzmmK/kvb/nlKjkuoZXZWJWaWV3MuaKgOacr+WunueOsOmhtemdoueahOa4suafk+OAgg0KDQo=".to_string()
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
    }
}