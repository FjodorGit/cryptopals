//use first_set::encrypt_with_repeating_xor::encrypt_sigle_line;
#![allow(dead_code)]
#![allow(unused_imports)]
use std::fs;
use std::result;
use std::str;

use crate::first_set::find_aes_encoding::find_most_repetetive_line;
use crate::first_set::find_aes_encoding::LineScore;
use crate::first_set::{base64::base64_to_bytes, decrypt_aes_128::decrypt_message};
use crate::second_set::decrypt_cbc_mode;
use crate::second_set::decrypt_cbc_mode::decrypt_in_cbc_mode;

mod first_set;
mod second_set;

fn main() {
    //let to_encrypt = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    //println!("{:?}", encrypt_sigle_line(to_encrypt, "HOTTER"))
    //let to_decrypt = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    //println!("{:#?}", single_bytes_decipher(to_decrypt))
    let path = "/home/fjk/Coding/Rust/cryptopals/src/recources/10.txt";
    //let bytes = hex::decode(&fs::read_to_string(path).expect("Error reading text from file"))
    //   .expect("Error decoding hex");
    let content = fs::read_to_string(path).expect("Error reading file");
    let result = decrypt_in_cbc_mode(
        &base64_to_bytes(&content),
        &vec![0; 16],
        &"YELLOW SUBMARINE".as_bytes().to_vec(),
    );
    print!("{}", result);
}
