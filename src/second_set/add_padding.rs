use std::cmp::{max, min};

pub fn add_padding_to(bytes: &mut Vec<u8>, length: usize) {
    let missing_len = (max(bytes.len(), length) - min(bytes.len(), length)) as u8;
    for _ in 0..missing_len {
        bytes.push(missing_len);
    }
}

#[cfg(test)]
mod tests {
    use super::add_padding_to;

    #[test]
    fn padding_test() {
        let mut input = "YELLOW SUBMARINE".as_bytes().to_vec();
        add_padding_to(&mut input, 20);
        let result = String::from_utf8(input).expect("Error converting to string");
        let expected = "YELLOW SUBMARINE\x04\x04\x04\x04";
        assert_eq!(result, expected)
    }
}
