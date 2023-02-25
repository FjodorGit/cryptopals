use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit},
    Aes128,
};

pub fn encrypt_message(content: &str, key_string: &str) -> Vec<u8> {
    let bytes = content.as_bytes().to_vec();
    let key = key_string.as_bytes().try_into().unwrap();
    let result_bytes = process_message(bytes, key, encrypt_using_aes);
    result_bytes
}

pub fn decrypt_message(content_bytes: Vec<u8>, key_string: &str) -> String {
    let key = key_string.as_bytes().try_into().unwrap();
    let result_bytes = process_message(content_bytes, key, decrypt_using_aes);
    String::from_utf8(result_bytes).unwrap()
}

fn process_message(
    content: Vec<u8>,
    key: [u8; 16],
    processing: fn([u8; 16], &Aes128) -> Vec<u8>,
) -> Vec<u8> {
    let gkey = GenericArray::from(key);
    let cipher = Aes128::new(&gkey);
    let mut result: Vec<u8> = vec![];
    if content.len() % 16 != 0 {
        panic!("Content is not cleanly devisable by 16")
    }
    for pos in 0..content.len() / 16 {
        let block = &content[16 * pos..16 * (pos + 1)].try_into().unwrap();
        let mut encrypted_block = processing(*block, &cipher);
        result.append(&mut encrypted_block);
    }
    result
}

pub fn decrypt_using_aes(content: [u8; 16], cipher: &Aes128) -> Vec<u8> {
    let mut block = GenericArray::from(content);
    cipher.decrypt_block(&mut block);
    block.to_vec()
}

pub fn encrypt_using_aes(content: [u8; 16], cipher: &Aes128) -> Vec<u8> {
    let mut block = GenericArray::from(content);
    cipher.encrypt_block(&mut block);
    block.to_vec()
}

#[cfg(test)]
mod tests {
    use crate::first_set::decrypt_aes_128::{decrypt_message, encrypt_message};

    #[test]
    fn encryption_decryption_test() {
        let key_string = "YELLOW SUBMARINE";
        let message = "The quick brown fox jumps over the lazy dog eat!";
        assert!(message.len() % 16 == 0);
        assert_eq!(
            message,
            decrypt_message(encrypt_message(message, key_string), key_string)
        );
    }
}
