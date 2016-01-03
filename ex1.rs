mod crypto;

fn main() 
{	
	let hex_string = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
	
	println!("Input Hex String: {}", hex_string);

	let hex_input: Vec<u8> = crypto::hex_string_to_bytes(hex_string).unwrap();

	match crypto::hex_to_base64(hex_input)
	{
		Ok(n) => println!("Output base64 string: {}", String::from_utf8(n).unwrap()),
        Err(err) => println!("Error: {}", err),
	}
}