use aes::{
    cipher::{generic_array::GenericArray, KeyInit},
    Aes128,
};
use std::str;

use crate::first_set::{
    self,
    decrypt_aes_128::{self, decrypt_message, decrypt_using_aes, encrypt_using_aes},
    fixed_xor::fixed_xor,
};

use super::add_padding::add_padding_to;

pub fn encrypt_cbc_mode(
    input: &Vec<u8>,
    initialization_vector: &Vec<u8>,
    key: &Vec<u8>,
) -> Vec<u8> {
    if input.len() == 0 {
        return vec![];
    }
    let key_as_bytes: [u8; 16] = key.to_vec().try_into().unwrap();
    let current_block = input[..key.len()].to_vec();
    let remaining_blocks = input[key.len()..].to_vec();
    let to_encrypt = fixed_xor(current_block, initialization_vector.to_vec());
    let cipher = Aes128::new(&GenericArray::from(key_as_bytes));
    let mut encrypted = encrypt_using_aes(to_encrypt.try_into().unwrap(), &cipher);
    let mut remaining_encryption = encrypt_cbc_mode(&remaining_blocks, &encrypted, key);
    encrypted.append(&mut remaining_encryption);
    encrypted
}

pub fn decrypt_cbc_mode(input: &Vec<u8>, initialization_vector: &Vec<u8>, key: &Vec<u8>) -> String {
    if input.len() == 0 {
        return "".to_string();
    }
    let bytes: Vec<u8> = input.iter().take(key.len()).map(|c| *c).collect();
    let next = decrypt_cbc_mode(&input[key.len()..].to_vec(), &bytes, &key);
    let key_as_bytes: [u8; 16] = key.to_vec().try_into().unwrap();
    let cipher = Aes128::new(&GenericArray::from(key_as_bytes));
    let decrypted_bytes = decrypt_using_aes(bytes.try_into().unwrap(), &cipher);
    let xored_bytes = fixed_xor(decrypted_bytes, initialization_vector.to_vec());
    String::from_utf8(xored_bytes).expect("Cant read in xored string") + &next
}

#[cfg(test)]
mod tests {
    use crate::second_set::add_padding::add_padding_to;

    use super::{decrypt_cbc_mode, encrypt_cbc_mode};

    #[test]
    fn encrypt_decrypt_test() {
        let mut input =
            "A very smart senctence written at 10 at night, because of a lack of imagination"
                .as_bytes()
                .to_vec();
        let key = "YELLOW SUBMARINE".as_bytes().to_vec();
        let initialization_vector = vec![0; key.len()];
        let missing_len = input.len() % key.len();
        if missing_len != 0 {
            add_padding_to(&mut input, missing_len - 1);
        }
        let encrypted = encrypt_cbc_mode(&mut input, &initialization_vector, &key);
        let decrypted = decrypt_cbc_mode(&encrypted, &initialization_vector, &key);
        println!("{}", decrypted);
        assert_eq!(
            "A very smart senctence written at 10 at night, because of a lack of imagination"
                .to_string(),
            decrypted
        )
    }
}
