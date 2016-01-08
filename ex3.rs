use std::str;
mod crypto;

fn print_vec(v: Vec<u8>) {
    for i in v.iter() {
        print!("[{}], ", i)
    }
}

fn main() 
{	
	let hex_string = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
	
	println!("Input Hex String: {}", hex_string);

	let encoded: Vec<u8> = crypto::hex_string_to_bytes(hex_string).unwrap();
	
	let encoded_freq: Vec<u8> = crypto::frequency_u8(encoded.clone()).unwrap();

	let best_score: u8 = crypto::best_score_u8(encoded_freq).unwrap();

	println!("{}", best_score);

	let key = String::from("58585858585858585858585858585858585858585858585858585858585858585858");
	
	let key_vec: Vec<u8> = crypto::hex_string_to_bytes(key).unwrap();

	let output_byte: Vec<u8> = crypto::fixed_xor(encoded,key_vec).unwrap();
		
	let sparkle_heart = str::from_utf8(&output_byte).unwrap();

	println!("{}", sparkle_heart);
}

