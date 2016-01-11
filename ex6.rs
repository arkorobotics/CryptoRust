use std::str;
use std::io::{BufReader,BufRead};
use std::fs::File;
mod crypto;

fn main() 
{	
	/*
	let file = File::open("ex6.txt").unwrap();

    for line in BufReader::new(file).lines() 
    {
    	let hex_string = String::from(line.unwrap());
    }
    */
    
    let string_a = String::from("this is a test");
    let string_b = String::from("wokka wokka!!!");

    let a: Vec<u8> = string_a.into_bytes();
    let b: Vec<u8> = string_b.into_bytes();

    let hamming_dist: i32 = crypto::hamming_distance(a.clone(),b.clone()).unwrap();

    println!("{}", hamming_dist);

    // TODO: Write a base64 to hex converter
}