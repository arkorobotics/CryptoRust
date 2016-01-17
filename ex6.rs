use std::str;
use std::io::{BufReader,BufRead};
use std::fs::File;
mod crypto;

fn main() 
{	
	let file = File::open("ex6.txt").unwrap();

	let mut byte_array: Vec<u8> = vec![];

	// Load file into byte array buffer
    for line in BufReader::new(file).lines() 
    {
    	let base64_string = String::from(line.unwrap());

    	let base64_vec: Vec<u8> = base64_string.into_bytes();

		let bytes_vec: Vec<u8> = crypto::base64_to_bytes(base64_vec).unwrap();

		for byte_element in bytes_vec.iter()
		{
			byte_array.push(byte_element.clone());
		}
    }

    let decoded_key = crypto::break_repeating_key_xor(byte_array.clone(), 2, 40).unwrap();

	let decoded_key_string = unsafe {str::from_utf8_unchecked(&decoded_key)};
	println!("{}", decoded_key_string);
}
