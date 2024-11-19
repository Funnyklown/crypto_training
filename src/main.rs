
use std::result;
use std::collections::HashMap;
use base64::*;
use hex::FromHex;
use anyhow::*;
use prelude::BASE64_STANDARD;


fn hexto64(hex: &str) -> Result<String, Error>{

    let byte = hex::decode(hex)?;

    let result = BASE64_STANDARD.encode(byte);

    Ok(result)
}

fn fixed_xor(input1: &str, input2: &str) -> Result<String, Error>{

    let mut byte1 = hex::decode(input1)?;
    let byte2 = hex::decode(input2)?;

    for i in 0..byte1.len(){
        byte1[i] ^= byte2[i];
    }

    let result = hex::encode(byte1);
    Ok(result)
}
/* 
fn single_byte_xor_cipher(cipher_text: &str) -> Result<String, Error>{

    // Create an empty HashMap to store character frequencies
    let mut char_frequency: HashMap<char, usize> = HashMap::new(); // Key: char, Value: usize

    // Iterate over each character in the input string
    for ch in cipher_text.chars() {
        // Check if the character is already present in the HashMap
        // If it is present, increment its frequency count
        // If it is not present, insert it into the HashMap with a frequency count of 1
        let count = char_frequency.entry(ch).or_insert(0);
        *count += 1;
    }

    let mut values = char_frequency.into_values().collect();
    let test = fixed_xor(cipher_text, char);
    Ok()
}
*/
fn main(){
    println!("cum");
}

#[cfg(test)]
mod tests{
    use crate::{fixed_xor, hexto64};

    #[test]
    fn test_hexto64(){
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hexto64(input).unwrap(), expected_output, "Not matching");
    }

    #[test]
    fn test_fixedxor(){
        let input1 = "1c0111001f010100061a024b53535009181c";
        let input2 = "686974207468652062756c6c277320657965";
        let expected_output = "746865206b696420646f6e277420706c6179";
        assert_eq!(fixed_xor(input1, input2).unwrap(), expected_output, "Not matching");
    }
}