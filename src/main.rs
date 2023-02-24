//use first_set::encrypt_with_repeating_xor::encrypt_sigle_line;
#![allow(dead_code)]
use std::fs;
use std::str;

use crate::first_set::find_aes_encoding::find_most_repetetive_line;
use crate::first_set::find_aes_encoding::LineScore;
use crate::first_set::{base64::base64_to_bytes, decrypt_aes_128::decrypt_message};

mod first_set;

fn main() {
    //let to_encrypt = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    //println!("{:?}", encrypt_sigle_line(to_encrypt, "HOTTER"))
    //let to_decrypt = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    //println!("{:#?}", single_bytes_decipher(to_decrypt))
    let path = "/home/fjk/Coding/Rust/cryptopals/src/recources/8.txt";
    //let bytes = hex::decode(&fs::read_to_string(path).expect("Error reading text from file"))
    //   .expect("Error decoding hex");
    let content = fs::read_to_string(path).expect("Error reading file");
    print!("{:#?}", find_most_repetetive_line(content.as_str()));
}
