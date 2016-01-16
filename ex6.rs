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

    // Find the key length
    let mut best_keylength: usize = 0;
    let mut best_avg: f32 = 8.0;

    for keysize in 2..40
    {
    	let mut avg: f32 = 0.0;

    	for i in 0..byte_array.len()/keysize -1
    	{
    		let mut a: Vec<u8> = vec![];
    		let mut b: Vec<u8> = vec![];
    		
    		for x in 0..keysize
    		{
    			a.push(byte_array[i*keysize+x]);
    		}

    		for y in keysize..keysize*2
    		{
    			b.push(byte_array[i*keysize+y]);
    		}
    		
    		let mut out: f32 = crypto::hamming_distance(a.clone(),b.clone()).unwrap() as f32;
    		out = out / (((keysize as i32)) as f32);

    		avg = avg + out;
    	}

    	avg = avg / (byte_array.len() as f32/keysize as f32);

    	if avg < best_avg
    	{
    		best_keylength = keysize;
    		best_avg = avg;
    	}
    }

	//println!("{}", best_keylength);

	let mut decoded_key: Vec<u8> = vec![];

	// Crack single-byte XOR key
	for x in 0..best_keylength
	{
		// Generate byte array of each repeating nth byte
		let mut a: Vec<u8> = vec![];
		
		for i in 0..byte_array.len()/best_keylength
		{
			a.push(byte_array[i*best_keylength + x]);
		}

		let (single_byte, score): (u8,f32) = crypto::find_single_byte_xor(a.clone()).unwrap();
		
		decoded_key.push(single_byte);
	}

	let decoded_key_string = unsafe {str::from_utf8_unchecked(&decoded_key)};
	println!("{}", decoded_key_string);
}


