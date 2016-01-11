mod crypto;

fn main() 
{	
	let message_string = String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");

	let message: Vec<u8> = message_string.into_bytes();

	let key_string: String = String::from("ICE");

	let key: Vec<u8> = key_string.into_bytes();

	let repeating_key: Vec<u8> = crypto::generate_repeating_byte_vec(key.clone(), message.len()).unwrap();
	
	let encoded_message: Vec<u8> = crypto::fixed_xor(message.clone(),repeating_key.clone()).unwrap();

	println!("{}", crypto::bytes_to_hex_string(encoded_message).unwrap());
}