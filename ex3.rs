use std::str;
mod crypto;

fn main() 
{	
	let hex_string = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
	
	println!("Input Hex String: {}", hex_string);

	let encoded: Vec<u8> = crypto::hex_string_to_bytes(hex_string).unwrap();
	
	let single_byte: u8 = crypto::find_single_byte_xor(encoded.clone()).unwrap();
	
	let single_byte_vec: Vec<u8> = crypto::generate_single_byte_vec(single_byte,encoded.len()).unwrap();

	let output_byte: Vec<u8> = crypto::fixed_xor(encoded.clone(),single_byte_vec.clone()).unwrap();

	let decoded = str::from_utf8(&output_byte).unwrap();

	println!("Decoded String: {}", decoded);
}

