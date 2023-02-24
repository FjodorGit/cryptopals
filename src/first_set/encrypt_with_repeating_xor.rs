use hex;

pub fn encrypt_sigle_line(to_encrypt: &str, key: &str) -> String {
    let big_key = key.repeat(100000);
    let encrypted: Vec<u8> = to_encrypt
        .chars()
        .zip(big_key.chars())
        .map(|(a, b)| ((a as u8) ^ (b as u8)))
        .collect();
    hex::encode(encrypted)
}

#[cfg(test)]
mod test {
    use super::encrypt_sigle_line;

    #[test]
    fn test_encryption() {
        let to_encrypt =
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected_result =
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        assert_eq!(expected_result, encrypt_sigle_line(to_encrypt, "ICE"))
    }
}
