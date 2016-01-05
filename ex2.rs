mod crypto;

fn main() 
{	
	let a_str = String::from("1c0111001f010100061a024b53535009181c");
	
	println!("Input A String: {}", a_str);

	let a: Vec<u8> = crypto::hex_string_to_bytes(a_str).unwrap();

	let b_str = String::from("686974207468652062756c6c277320657965");
	
	println!("Input B String: {}", b_str);

	let b: Vec<u8> = crypto::hex_string_to_bytes(b_str).unwrap();

	match crypto::fixed_xor(a,b)
	{
		Ok(n) => println!("Output XOR string: {}", crypto::bytes_to_hex_string(n).unwrap()),
        Err(err) => println!("Error: {}", err),
	}
}