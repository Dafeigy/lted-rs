## Apply Luby Transform in Rust and WebAssembly

Rewrite the logic in Rust with the help of TRAE.

### Example

```rust
use luby_transform::{Encoder, Decoder};
fn main(){
    let total_blocks = 512;
    let source_blocks = (0..total_blocks)
        .map(|_| vec![1, 2, 3])
        .collect::<Vec<_>>();
    
    // 获取第一个源块的长度作为块大小
    let block_size = source_blocks[0].len();
    let k = source_blocks.len();
    
    // 创建编码器和解码器
    let mut encoder = Encoder::new_default(source_blocks.clone(), Some(10));
    let mut decoder = Decoder::new_default(k, block_size);
    
    println!("Start Encode and Decode Process...\n");
    let mut i = 0;
    // 生成编码块并添加到解码器
    while !decoder.is_complete() { // 生成略多于源块数量的编码块以提高解码概率
        let (seed, d, indices, encoded_block) = encoder.generate_encoded_block(None);
        println!("Encoded Block #{}, seed: {}, d: {}, block_ids: {:?}", i, seed, d, indices);
        i += 1;
        
        // 将编码块添加到解码器
        decoder.add_encoded_block(seed, d, encoded_block);
        
        // 检查是否解码完成
        if decoder.is_complete() {
            println!("Decode Process Completed! Used {} Encoded Blocks", i + 1);
            break;
        }
    }
    
    // 输出解码结果
    if decoder.is_complete() {
        println!("\nDecoded Results:");
        if let Some(decoded_blocks) = decoder.get_all_decoded_blocks() {
            for (i, block) in decoded_blocks.iter().enumerate() {
                // println!("Block {}: {}", i, block);
                // 验证解码结果是否正确
                assert_eq!(block, &source_blocks[i], "Decoded Result Incorrect!");
            }
            println!("All Blocks Decoded Successfully and Results Correct!");
        }
    } else {
        println!("Decode Process Not Completed, Only {} Blocks Decoded (Total {} Blocks)",  
                 decoder.decoded_count(), k);
    }
    
}
```

## To build WASM
```bash
wasm-pack build --target web 
```