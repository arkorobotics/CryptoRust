use std::u8;

//
//  CRYPTO :: Now in Rust!
//

// You maybe asking yourself, "What fresh hell is this?". Well...
// this is my poor attempt at creating a Rust module for CryptoPals
// See http://cryptopals.com for details.
//
// The goal of this project is to provide a robust and fast crypto module
// written in Rust. Some of these functions may be considered bloated
// or inefficient, but that's ok. The first priority here is robustness, 
// followed by speed. As a side note, this is my first Rust project.. 
// So if you see room for improvement or poor coding practice, feel
// free to correct it :)

#[allow(dead_code)]
pub fn hex_string_to_bytes(hex_string: String) -> Result<Vec<u8>, String>
{
	if hex_string.len() <= 0 
	{
		return Err("Invalid hex vector. Length of hex vector is less than or equal to zero.".to_string());
	}

	let hex_chars: Vec<char> = hex_string.chars().collect();

	let mut hex_words: Vec<u8> = vec![];

	let mut hex_bytes: Vec<u8> = vec![];

	// If the hex string is missing a leading zero, add a zero
	if hex_chars.len() % 2 == 1
	{
		hex_words.push(0);
	}

	for i in 0..hex_chars.len()
	{
		let hex_ch = hex_chars[i].to_string();
		let n: u8 = u8::from_str_radix(&hex_ch, 16).unwrap();
		hex_words.push(n as u8);
	}

	for i in 0..hex_words.len()/2
	{
		hex_bytes.push(((hex_words[2*i] << 4) | hex_words[2*i+1]) as u8);
	}

	Ok(hex_bytes)
}

#[allow(dead_code)]
pub fn bytes_to_hex_string(bytes_input: Vec<u8>) -> Result<String, String>
{
	if bytes_input.len() <= 0 
	{
		return Err("Invalid input vector. Length of byte vector is less than or equal to zero.".to_string());
	}

	let mut output_string: String = String::new();

	for bytes_element in bytes_input.iter()
	{
		output_string.push_str(&format!("{:X}", (bytes_element & 0xF0) >> 4));
		output_string.push_str(&format!("{:X}", bytes_element & 0x0F));
	}

    Ok(output_string)
}

#[allow(dead_code)]
pub fn print_vec(v: Vec<u8>) 
{
    for i in v.iter() 
    {
        print!("[{:X}], ", i);
    }
}

#[allow(dead_code)]
pub fn hex_to_base64(hex_input: Vec<u8>) -> Result<Vec<u8>, String> 
{
	if hex_input.len() == 0
	{
		return Err("Input hex vector is invalid. Length of hex vector is zero.".to_string());
	}

	let mut hex_vec: Vec<u8> = hex_input;

	let mut base64_output: Vec<u8> = vec![];

	let base64_lookup: Vec<u8> 
		= "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string().into_bytes();

	// Since the input hex vector is immutable, we create our own
	// local hex input vector, and add zero padding if necessary
	let hex_str_rmdr = hex_vec.len() % 3;

	match hex_str_rmdr
	{
		0 => { /* No need to add zero padding the hex input */ },
		1 => { hex_vec.push(0 as u8); hex_vec.push(0 as u8); },
		2 => { hex_vec.push(0 as u8)},
		_ => { return Err("Input hex vector is invalid. Remainder fault, how did you get this error??".to_string());
			 }
	}

	let hex_str_len = hex_vec.len();

	let mut hex_char: Vec<u8>;

	for i in 0..(hex_str_len/3)
	{
		hex_char = vec![];
		hex_char.push(hex_vec[3*i]);
		hex_char.push(hex_vec[3*i+1]);
		hex_char.push(hex_vec[3*i+2]);

		let base64_a = (hex_char[0] & 0xFC) >> 2;
		let base64_b = ((hex_char[0] & 0x03) << 4) | ((hex_char[1] & 0xF0) >> 4);
		let base64_c = ((hex_char[1] & 0x0F) << 2) | ((hex_char[2] & 0xC0) >> 6);
		let base64_d = hex_char[2] & 0x3F;

		if base64_a > 64 || base64_b > 64 || base64_c > 64 || base64_d > 64
		{
	    	return Err("Base 64 Table Lookup Error. Index out of bounds.".to_string());
		}

		if i != (hex_str_len/3)-1
		{
			base64_output.push(base64_lookup[base64_a as usize]);
			base64_output.push(base64_lookup[base64_b as usize]);
			base64_output.push(base64_lookup[base64_c as usize]);
			base64_output.push(base64_lookup[base64_d as usize]);
		}
		else
		{
			match hex_str_rmdr
			{
				0 => {
						base64_output.push(base64_lookup[base64_a as usize]);
						base64_output.push(base64_lookup[base64_b as usize]);
						base64_output.push(base64_lookup[base64_c as usize]);
						base64_output.push(base64_lookup[base64_d as usize]);
					 },
				1 => {
						base64_output.push(base64_lookup[base64_a as usize]);
						base64_output.push(base64_lookup[base64_b as usize]);
						base64_output.push('=' as u8);
						base64_output.push('=' as u8);
					 },
				2 => {
						base64_output.push(base64_lookup[base64_a as usize]);
						base64_output.push(base64_lookup[base64_b as usize]);
						base64_output.push(base64_lookup[base64_c as usize]);
						base64_output.push('=' as u8);
					 },
				_ => {	return Err("Invalid hex string. Odd remainder, how did you get this error??".to_string());
					 }
			} 
		}
	}

	Ok(base64_output)
}

#[allow(dead_code)]
pub fn fixed_xor(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, String>
{
	if a.len() != b.len()
	{
		return Err("Input buffers are not the same length.".to_string());
	}

	if a.len() == 0
	{
		return Err("Input buffers have a length of zero.".to_string());
	}

	let mut c: Vec<u8> = vec![];

	for i in 0..a.len()
	{
		c.push(a[i]^b[i]);
	}

	Ok(c)
}

#[allow(dead_code)]
pub fn frequency_u8(a: Vec<u8>) -> Result<Vec<u8>, String>
{
	if a.len() == 0
	{
		return Err("Input buffer has a length of zero.".to_string());
	}

	let mut b: Vec<u8> = vec![0; 256];

	for i in 0..a.len()
	{
		b[a[i] as usize] += 1;
	}

	Ok(b)
}

#[allow(dead_code)]
pub fn find_single_byte_xor(a: Vec<u8>) -> Result<(u8,f32), String>
{
	let a_len = a.len();

	if a_len == 0
	{
		return Err("Input buffer has a length of zero.".to_string());
	}

	// Initialize to null, as zero can never be the best score
	let mut score: f32 = 0.000;
	let mut best: f32 = 0.000;
	let mut single_byte: u8 = 0;

	for c in 1..255
	{
		let single_byte_vec = generate_single_byte_vec(c, a_len).unwrap();

		let encoded_vec = fixed_xor(a.clone(),single_byte_vec).unwrap();

		for i in 0..encoded_vec.len()
		{
			match encoded_vec[i] as char
			{
			    'E' | 'e' => score += 12.702,
			    'T' | 't' => score += 9.056,
			    'A' | 'a' => score += 8.167,
			    'O' | 'o' => score += 7.507,
			    'I' | 'i' => score += 6.966,
			    'N' | 'n' => score += 6.749,
			    'S' | 's' => score += 6.327,
			    'H' | 'h' => score += 6.094,
			    'R' | 'r' => score += 5.987,
			    'D' | 'd' => score += 4.253,
			    'L' | 'l' => score += 4.025,
			    'C' | 'c' => score += 2.782,
			    'U' | 'u' => score += 2.758,
			    'M' | 'm' => score += 2.406,
			    'W' | 'w' => score += 2.360,
			    'F' | 'f' => score += 2.228,
			    'G' | 'g' => score += 2.015,
			    'Y' | 'y' => score += 1.974,
			    'P' | 'p' => score += 1.929,
			    'B' | 'b' => score += 1.492,
			    'V' | 'v' => score += 0.978,
			    'K' | 'k' => score += 0.772,
			    'J' | 'j' => score += 0.153,
			    'X' | 'x' => score += 0.150,
			    'Q' | 'q' => score += 0.095,
			    'Z' | 'z' => score += 0.074,
			    ' ' => score += 13.1,
			    '0' => score += 8.4,
			    '1' => score += 8.4,
			    '2' => score += 8.4,
			    '3' => score += 8.4,
			    '4' => score += 8.4,
			    '5' => score += 8.4,
			    '6' => score += 8.4,
			    '7' => score += 8.4,
			    '8' => score += 8.4,
			    '9' => score += 8.4,
			    '\'' => score += 8.4,
			    '"' => score += 8.4,
			    '.' => score += 8.4,
			    ',' => score += 8.4,
			    '!' => score += 8.4,
			    '?' => score += 8.4,
			    _ => score += 0.000
			}
		}

		if score > best
		{
			single_byte = c;
			best = score;
		}
		score = 0.000;
	}

	Ok((single_byte,best))
}

#[allow(dead_code)]
pub fn generate_single_byte_vec(single_byte: u8, length: usize) -> Result<Vec<u8>, String>
{
	if length == 0
	{
		return Err("Length is zero.".to_string());
	}

	let mut single_byte_vec: Vec<u8> = vec![];

	for _ in 0..length
	{
		single_byte_vec.push(single_byte);
	}

	Ok(single_byte_vec)
}

#[allow(dead_code)]
pub fn generate_repeating_byte_vec(input_vec: Vec<u8>, repeating_length: usize) -> Result<Vec<u8>, String>
{
	if input_vec.len() == 0
	{
		return Err("Input vector is zero.".to_string());
	}

	if repeating_length == 0
	{
		return Err("Required repeating vector length is zero.".to_string());
	}

	if input_vec.len() > repeating_length
	{
		return Err("Input vector is larger than the repeating vector length.".to_string());
	}

	let mut repeating_byte_vec: Vec<u8> = vec![];

	for i in 0..repeating_length
	{
		repeating_byte_vec.push(input_vec[i%input_vec.len()]);
	}

	Ok(repeating_byte_vec)
}