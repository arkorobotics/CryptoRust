use std::str;
use std::io::{BufReader,BufRead};
use std::fs::File;
mod crypto;

fn main() 
{	
	let file = File::open("ex4.txt").unwrap();

	let mut best_score: f32 = 0.000;
	let mut best_decoded: String = String::new();

    for line in BufReader::new(file).lines() 
    {
    	let hex_string = String::from(line.unwrap());
	
		let encoded: Vec<u8> = crypto::hex_string_to_bytes(hex_string).unwrap();
		
		let (single_byte, score): (u8,f32) = crypto::find_single_byte_xor(encoded.clone()).unwrap();
		
		let single_byte_vec: Vec<u8> = crypto::generate_single_byte_vec(single_byte,encoded.len()).unwrap();

		let output_byte: Vec<u8> = crypto::fixed_xor(encoded.clone(),single_byte_vec.clone()).unwrap();

		let decoded = unsafe {str::from_utf8_unchecked(&output_byte)};
		
		if score > best_score
		{
			best_decoded = decoded.to_string();
			best_score = score;
		}
    }
    println!("{}", best_decoded);
}