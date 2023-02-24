use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit},
    Aes128,
};

pub fn decrypt_message(content: Vec<u8>, key: [u8; 16]) -> Vec<u8> {
    let gkey = GenericArray::from(key);
    let cipher = Aes128::new(&gkey);
    let mut result: Vec<u8> = vec![];
    if content.len() % 16 != 0 {
        panic!("Content is not cleanly devisable by 16")
    }
    for pos in 0..content.len() / 16 {
        let block = &content[16 * pos..16 * (pos + 1)].try_into().unwrap();
        let mut encrypted_block = decrypt_using_aes(*block, &cipher);
        result.append(&mut encrypted_block);
    }
    result
}

fn decrypt_using_aes(content: [u8; 16], cipher: &Aes128) -> Vec<u8> {
    let mut block = GenericArray::from(content);
    cipher.decrypt_block(&mut block);
    block.to_vec()
}
