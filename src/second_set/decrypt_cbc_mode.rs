use aes::{
    cipher::{generic_array::GenericArray, KeyInit},
    Aes128,
};
use std::str;

use crate::first_set::{
    self,
    decrypt_aes_128::{self, decrypt_message, decrypt_using_aes},
    fixed_xor::fixed_xor,
};

use super::add_padding::add_padding_to;

pub fn decrypt_in_cbc_mode(
    input: &Vec<u8>,
    initialization_vector: &Vec<u8>,
    key: &Vec<u8>,
) -> String {
    if input.len() == 0 {
        return "".to_string();
    }
    let bytes: Vec<u8> = input.iter().take(key.len()).map(|c| *c).collect();
    let next = decrypt_in_cbc_mode(&input[key.len()..].to_vec(), &bytes, &key);
    let key_as_bytes: [u8; 16] = key.to_vec().try_into().unwrap();
    let cipher = Aes128::new(&GenericArray::from(key_as_bytes));
    let decrypted_bytes = decrypt_using_aes(bytes.try_into().unwrap(), &cipher);
    let xored_bytes = fixed_xor(decrypted_bytes, initialization_vector.to_vec());
    String::from_utf8(xored_bytes).expect("Cant read in xored string") + &next
}
