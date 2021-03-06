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
pub fn base64_lut(input_u8: u8) -> Result<u8, String> 
{
	match input_u8 as char
	{
		'A' => {Ok(0)},'B' => {Ok(1)},'C' => {Ok(2)},'D' => {Ok(3)},'E' => {Ok(4)},
		'F' => {Ok(5)},'G' => {Ok(6)},'H' => {Ok(7)},'I' => {Ok(8)},'J' => {Ok(9)},
		'K' => {Ok(10)},'L' => {Ok(11)},'M' => {Ok(12)},'N' => {Ok(13)},'O' => {Ok(14)},
		'P' => {Ok(15)},'Q' => {Ok(16)},'R' => {Ok(17)},'S' => {Ok(18)},'T' => {Ok(19)},
		'U' => {Ok(20)},'V' => {Ok(21)},'W' => {Ok(22)},'X' => {Ok(23)},'Y' => {Ok(24)},
		'Z' => {Ok(25)},'a' => {Ok(26)},'b' => {Ok(27)},'c' => {Ok(28)},'d' => {Ok(29)},
		'e' => {Ok(30)},'f' => {Ok(31)},'g' => {Ok(32)},'h' => {Ok(33)},'i' => {Ok(34)},
		'j' => {Ok(35)},'k' => {Ok(36)},'l' => {Ok(37)},'m' => {Ok(38)},'n' => {Ok(39)},
		'o' => {Ok(40)},'p' => {Ok(41)},'q' => {Ok(42)},'r' => {Ok(43)},'s' => {Ok(44)},
		't' => {Ok(45)},'u' => {Ok(46)},'v' => {Ok(47)},'w' => {Ok(48)},'x' => {Ok(49)},
		'y' => {Ok(50)},'z' => {Ok(51)},'0' => {Ok(52)},'1' => {Ok(53)},'2' => {Ok(54)},
		'3' => {Ok(55)},'4' => {Ok(56)},'5' => {Ok(57)},'6' => {Ok(58)},'7' => {Ok(59)},
		'8' => {Ok(60)},'9' => {Ok(61)},'+' => {Ok(62)},'/' => {Ok(63)},'=' => {Ok(64)},
		_ => {return Err("Input character out of bounds.".to_string())}
	}
}

#[allow(dead_code)]
pub fn base64_to_bytes(base64_input: Vec<u8>) -> Result<Vec<u8>, String> 
{
	if base64_input.len() == 0
	{
		return Err("Input vector is empty.".to_string());
	}

	if base64_input.len() % 4 != 0
	{
		return Err("Input vector is not a valid base64 vector. Is not divisble by 4.".to_string());
	}

	let mut byte_vec: Vec<u8> = vec![];

	let mut _byte_a: u8 = 0;
	let mut _byte_b: u8 = 0;
	let mut _byte_c: u8 = 0;

	for i in 0..(base64_input.len()/4)
	{
		_byte_a = (base64_lut(base64_input[4*i]).unwrap() << 2) | (base64_lut(base64_input[(4*i)+1]).unwrap() >> 4);
		byte_vec.push(_byte_a);

		if base64_lut(base64_input[(4*i)+2]).unwrap() < 64
		{
			_byte_b = (base64_lut(base64_input[(4*i)+1]).unwrap() << 4) | (base64_lut(base64_input[(4*i)+2]).unwrap() >> 2);
			byte_vec.push(_byte_b);
		}
		
		if base64_lut(base64_input[(4*i)+3]).unwrap() < 64
		{
			_byte_c = (base64_lut(base64_input[(4*i)+2]).unwrap() << 6) | base64_lut(base64_input[(4*i)+3]).unwrap();
			byte_vec.push(_byte_c);
		}
	}

	Ok(byte_vec)
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

#[allow(dead_code)]
pub fn hamming_distance(a: Vec<u8>, b: Vec<u8>) -> Result<i32, String>
{
	let mut hamming_distance: i32 = 0;

	if a.len() != b.len()
	{
		return Err("Input buffers are not the same length.".to_string());
	}

	if a.len() == 0
	{
		return Err("Input buffers have a length of zero.".to_string());
	}

	for i in 0..(a.len()*8)
	{
		if (a[i/8]>>(i%8) & 0x01) ^ (b[i/8]>>(i%8) & 0x01) == 1
		{
			hamming_distance += 1;
		}
	}
	
	Ok(hamming_distance)
}

#[allow(dead_code)]
pub fn break_repeating_key_xor(byte_array: Vec<u8>, min: usize, max: usize) -> Result<Vec<u8>, String>
{

    // Find the key length
    let mut best_keylength: usize = 0;
    let mut best_avg: f32 = 8.0;

    for keysize in min..max
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
    		
    		let mut out: f32 = hamming_distance(a.clone(),b.clone()).unwrap() as f32;
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

		let (single_byte, _score): (u8,f32) = find_single_byte_xor(a.clone()).unwrap();
		
		decoded_key.push(single_byte);
	}
	
	Ok(decoded_key)
}


