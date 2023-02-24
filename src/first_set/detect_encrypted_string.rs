use super::single_bytes_decipher::{single_char_decipher, ScoreString};

fn detect_encrypted_string(content: &str) -> ScoreString {
    content
        .lines()
        .map(|line| single_char_decipher(hex::decode(line).unwrap()))
        .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::detect_encrypted_string;

    #[test]
    fn test_detection() {
        let file_path = "/home/fjk/Coding/Rust/cryptopals/src/recources/4.txt";
        let content = fs::read_to_string(file_path).expect("Error reading message");
        assert_eq!('5', detect_encrypted_string(&content).cipher)
    }
}
