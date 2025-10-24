use luby_transform as lt;
use lt::encoder::Encoder;
use lt::decoder::Decoder;
fn main(){
    let source_blocks = vec![
            "same_length_str_1".to_string(),
            "same_length_str_2".to_string(),
            "same_length_str_3".to_string(),
            "same_length_str_4".to_string(),
            "same_length_str_5".to_string(),
            "same_length_str_6".to_string(),
            "same_length_str_7".to_string(),
            "same_length_str_8".to_string(),
            "same_length_str_9".to_string(),
            "same_length_str_9".to_string(),

        ];
    
    // 获取第一个源块的长度作为块大小
    let block_size = source_blocks[0].len();
    let k = source_blocks.len();
    
    // 创建编码器和解码器
    let mut encoder = Encoder::new_default(source_blocks.clone(), Some(1));
    let mut decoder = Decoder::new_default(k, block_size);
    
    println!("开始编码和解码过程...");
    
    // 生成编码块并添加到解码器
    for i in 0..100 { // 生成略多于源块数量的编码块以提高解码概率
        let (seed, d, indices, encoded_block) = encoder.generate_encoded_block(None);
        println!("编码块 #{}, seed: {}, d: {}, 索引: {:?}", i, seed, d, indices);
        
        // 将编码块添加到解码器
        decoder.add_encoded_block(seed, d, encoded_block);
        
        // 检查是否解码完成
        if decoder.is_complete() {
            println!("解码完成！使用了 {} 个编码块", i + 1);
            break;
        }
    }
    
    // 输出解码结果
    if decoder.is_complete() {
        println!("\n解码结果:");
        if let Some(decoded_blocks) = decoder.get_all_decoded_blocks() {
            for (i, block) in decoded_blocks.iter().enumerate() {
                println!("块 {}: {}", i, block);
                // 验证解码结果是否正确
                assert_eq!(block, &source_blocks[i], "解码结果不正确！");
            }
            println!("所有块解码成功且结果正确！");
        }
    } else {
        println!("解码未完成，仅解码了 {} 个块（共 {} 个）", 
                 decoder.decoded_count(), k);
    }
    
}