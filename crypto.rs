use std::u8;

pub fn hex_to_base64(hex_input: Vec<u8>) -> Result<Vec<u8>, String> 
{
	if hex_input.len() <= 0
	{
		return Err("Invalid hex vector. Length of hex vector is less than or equal to zero.".to_string());
	}

	let mut hex_vec: Vec<u8> = hex_input;

	let mut base64_output: Vec<u8> = vec![];

	let base64_lookup: Vec<u8> 
		= "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string().into_bytes();

	let hex_str_rmdr = hex_vec.len() % 3;

	if hex_str_rmdr == 0
	{
		// No need to modify the hex input
	}
	else if hex_str_rmdr == 1
	{
	    hex_vec.push(0 as u8);
	    hex_vec.push(0 as u8);
	}
	else if hex_str_rmdr == 2
	{
		hex_vec.push(0 as u8);
	}
	else 
	{
	    return Err("Invalid hex vector. Odd remainder, how did you get this error??".to_string());
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
			if hex_str_rmdr == 0
			{
				base64_output.push(base64_lookup[base64_a as usize]);
				base64_output.push(base64_lookup[base64_b as usize]);
				base64_output.push(base64_lookup[base64_c as usize]);
				base64_output.push(base64_lookup[base64_d as usize]);
			}
			else if hex_str_rmdr == 1
			{
				base64_output.push(base64_lookup[base64_a as usize]);
				base64_output.push(base64_lookup[base64_b as usize]);
				base64_output.push('=' as u8);
				base64_output.push('=' as u8);
			}
			else if hex_str_rmdr == 2
			{
				base64_output.push(base64_lookup[base64_a as usize]);
				base64_output.push(base64_lookup[base64_b as usize]);
				base64_output.push(base64_lookup[base64_c as usize]);
				base64_output.push('=' as u8);
			}
			else 
			{
			    return Err("Invalid hex string. Odd remainder, how did you get this error??".to_string());
			}  
		}
	}

	Ok(base64_output)
}

pub fn hex_string_to_bytes(hex_string: String) -> Result<Vec<u8>, String>
{
	if hex_string.len() <= 0 
	{
		return Err("Invalid hex vector. Length of hex vector is less than or equal to zero.".to_string());
	}

	let hex_chars: Vec<char> = hex_string.chars().collect();

	let mut hex_words: Vec<u8> = vec![];

	let mut hex_bytes: Vec<u8> = vec![];

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