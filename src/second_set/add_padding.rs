pub fn add_padding_to(bytes: &mut Vec<u8>, length: usize) -> &Vec<u8> {
    let missing_len = (length - bytes.len()) as u8;
    for _ in 0..missing_len {
        bytes.push(missing_len);
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::add_padding_to;

    #[test]
    fn padding_test() {
        let mut input = "YELLOW SUBMARINE".as_bytes().to_vec();
        let result = String::from_utf8(add_padding_to(&mut input, 20).to_vec())
            .expect("Error converting to string");
        let expected = "YELLOW SUBMARINE\x04\x04\x04\x04";
        assert_eq!(result, expected)
    }
}
