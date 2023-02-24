use base64::{engine::general_purpose, Engine as _};
use hex;

fn to_base64(input: &str) -> String {
    let bytes = &hex::decode(input).unwrap()[..];
    general_purpose::STANDARD.encode(bytes)
}

pub fn base64_to_bytes(input: &str) -> Vec<u8> {
    let content: Vec<u8> = input
        .as_bytes()
        .iter()
        .map(|a| *a)
        .filter(|c| !c.is_ascii_whitespace())
        .collect();
    general_purpose::STANDARD
        .decode(content)
        .expect("Error decoding base64")
}

#[cfg(test)]
mod tests {
    use super::to_base64;

    #[test]
    fn small_test() {
        let input = "64";
        let result = to_base64(input);
        assert_eq!(result, "ZA==")
    }

    #[test]
    fn big_test() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let result = to_base64(input);
        assert_eq!(
            result,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        )
    }
}
