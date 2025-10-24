## Apply Luby Transform in Rust and WebAssembly

Rewrite the logic in Rust with the help of TRAE.

### Example

```rust
use luby_transform as lt;
use lt::encoder::Encoder;
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
            "same_length_str_0".to_string(),

        ];
    let mut encoder = Encoder::new_default(source_blocks, seed=Some(1));
    for _ in 0..10 {
        let (seed, d, indices, encoded_block) = encoder.generate_encoded_block(None);
        println!("seed: {}, d: {}, indices: {:?}", seed, d, indices);
        println!("encoded_block: {:?}", encoded_block);
    }
    
}
```

## To build WASM
```bash
wasm-pack build --target web 
```