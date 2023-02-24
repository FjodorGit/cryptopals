use hex;

pub fn fixed_xor(left_bytes: Vec<u8>, right_bytes: Vec<u8>) -> String {
    let xored: Vec<u8> = left_bytes
        .iter()
        .zip(right_bytes.iter())
        .map(|(l, r)| l ^ r)
        .collect();
    hex::encode(xored)
}

#[cfg(test)]
mod test {
    use super::fixed_xor;

    #[test]
    fn xor_test() {
        let a = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let b = hex::decode("686974207468652062756c6c277320657965").unwrap();
        assert_eq!("746865206b696420646f6e277420706c6179", fixed_xor(a, b))
    }
}
