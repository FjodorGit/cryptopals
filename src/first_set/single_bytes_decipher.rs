#[derive(Debug)]
pub struct ScoreString {
    pub score: f32,
    pub cipher: char,
    pub value: String,
}

impl Default for ScoreString {
    fn default() -> Self {
        ScoreString {
            score: 0.0,
            cipher: 0 as char,
            value: " ".to_string(),
        }
    }
}

pub fn single_char_decipher(bytes: Vec<u8>) -> ScoreString {
    let mut sentence: Vec<u8>;
    let mut best_score_string: ScoreString = ScoreString::default();
    for letter in 0..255 {
        sentence = bytes.iter().map(|elem| elem ^ letter).collect();
        let score = sentence_score(&sentence);
        if score > best_score_string.score {
            best_score_string.cipher = letter as char;
            best_score_string.score = score;
            best_score_string.value = sentence.iter().map(|c| *c as char).collect();
        }
    }
    best_score_string
}

fn sentence_score(bytes: &Vec<u8>) -> f32 {
    bytes
        .iter()
        .map(|x| {
            if x.is_ascii_alphabetic() || *x == 32 {
                1
            } else {
                0
            }
        })
        .sum::<i32>() as f32
        / bytes.len() as f32
}

#[cfg(test)]
mod test {
    use super::single_char_decipher;

    #[test]
    fn single_decipher_test() {
        let encrypted =
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();
        assert_eq!('X', single_char_decipher(encrypted).cipher)
    }
}
