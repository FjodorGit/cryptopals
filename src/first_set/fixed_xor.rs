use hex;

pub fn fixed_xor(left_bytes: Vec<u8>, right_bytes: Vec<u8>) -> Vec<u8> {
    left_bytes
        .iter()
        .zip(right_bytes.iter())
        .map(|(l, r)| l ^ r)
        .collect()
}

#[cfg(test)]
mod test {
    use super::fixed_xor;

    #[test]
    fn xor_test() {
        let a = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let b = hex::decode("686974207468652062756c6c277320657965").unwrap();
        let xored = hex::encode(fixed_xor(a, b));
        assert_eq!("746865206b696420646f6e277420706c6179", xored)
    }
}
