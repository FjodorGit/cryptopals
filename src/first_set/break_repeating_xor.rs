use super::single_bytes_decipher::single_char_decipher;
use super::single_bytes_decipher::ScoreString;
struct KeyDistance {
    keylen: usize,
    distance: f32,
}

pub struct KeyScore {
    score: f32,
    key: String,
    value: String,
}

impl Default for KeyScore {
    fn default() -> Self {
        KeyScore {
            score: 0.0,
            key: "".to_string(),
            value: "".to_string(),
        }
    }
}

impl From<&ScoreString> for KeyScore {
    fn from(s: &ScoreString) -> Self {
        KeyScore {
            score: s.score,
            key: String::from(s.cipher),
            value: s.value.chars().map(|x| String::from(x)).collect(),
        }
    }
}

pub fn break_repeating_xor(to_break: Vec<u8>) -> KeyScore {
    let best_keys = find_best_keysize(&to_break);
    best_keys
        .iter()
        .map(|keylen| interpolate_decryptions(decipher_with_known_keysize(&to_break, *keylen)))
        .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .unwrap()
}

fn decipher_with_known_keysize(to_break: &Vec<u8>, keylen: usize) -> Vec<ScoreString> {
    let mut single_deciphers: Vec<ScoreString> = vec![];
    for key_index in 0..keylen {
        let block_to_deciper = to_break[key_index..]
            .iter()
            .step_by(keylen)
            .map(|a| *a)
            .collect();
        let deciper = single_char_decipher(block_to_deciper);
        single_deciphers.push(deciper);
    }
    single_deciphers
}

fn interpolate_decryptions(decipherd: Vec<ScoreString>) -> KeyScore {
    let value_len = decipherd[0].value.len();
    let values: Vec<&[u8]> = decipherd.iter().map(|e| e.value.as_bytes()).collect();
    let mut result = decipherd
        .iter()
        .fold(KeyScore::default(), |acc, x| KeyScore {
            score: acc.score + x.score,
            key: acc.key + &String::from(x.cipher),
            value: "".to_string(),
        });
    for index in 0..value_len {
        let a = values
            .iter()
            .map(|value| {
                let char_at = value.get(index);
                match char_at {
                    None => "".to_string(),
                    Some(c) => String::from(*c as char),
                }
            })
            .fold("".to_string(), |acc, x| acc + &x);
        result.value += &a;
    }
    result
}

fn find_best_keysize(to_break: &Vec<u8>) -> Vec<usize> {
    let mut best_values: Vec<KeyDistance> = vec![];
    for keysize in 2..40 {
        let distance = average_hamming_distance_of_first_n(&to_break, keysize, 10);
        if best_values.len() < 3 || best_values.iter().any(|elem| elem.distance > distance) {
            best_values.push(KeyDistance {
                keylen: keysize,
                distance,
            })
        }
    }
    best_values.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    best_values.iter().map(|k| k.keylen).take(3).collect()
}

fn average_hamming_distance_of_first_n(input: &Vec<u8>, bytes_size: usize, n: usize) -> f32 {
    if bytes_size * (n + 1) >= input.len() {
        return f32::MAX;
    }
    let mut summed_distance: f32 = 0.0;
    for pos in 0..(n - 1) {
        let a = input[pos * bytes_size..(pos + 1) * bytes_size].to_vec();
        let b = input[(pos + 1) * bytes_size..(pos + 2) * bytes_size].to_vec();
        summed_distance += hamming_distance(a, b) as f32 / bytes_size as f32;
    }
    summed_distance / n as f32
}

fn hamming_distance(s: Vec<u8>, t: Vec<u8>) -> usize {
    if s.len() != t.len() {
        panic!("Unequal sized of strings")
    }
    s.iter()
        .zip(t.iter())
        .map(|(a, b)| a ^ b)
        .fold(0, |acc, x| acc + usize::try_from(x.count_ones()).unwrap())
}

#[cfg(test)]
mod test {

    use super::{
        decipher_with_known_keysize, find_best_keysize, hamming_distance, interpolate_decryptions,
    };

    #[test]
    fn hemming_distance_test() {
        let a = "this is a test".as_bytes().to_vec();
        let b = "wokka wokka!!!".as_bytes().to_vec();
        assert_eq!(37, hamming_distance(a, b))
    }

    #[test]
    fn best_keysize_test() {
        let encrypted1 = hex::decode("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap();
        let encrypted2 = hex::decode("0c3c312b3b202e6362372365632c346e302c30722f202d62266e38362c312569222b366e272a2830222c490c7229266326202f333a6525262c2d651b6e212624206e2863262b232b2229").unwrap();
        let encrypted3 = hex::decode("0a3a263a2c3c2f6f7331287e682632743c3d3d6f353d2b753c6f25212c31236f353a2172262639362937420674332a722b3d352e3c723f27313a651b682731353772296f372d28302923").unwrap();
        assert!(find_best_keysize(&encrypted1).contains(&3));
        assert!(find_best_keysize(&encrypted2).contains(&5));
        assert!(find_best_keysize(&encrypted3).contains(&6));
    }

    #[test]
    fn find_right_key_test() {
        let encrypted1 = hex::decode("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap();
        let encrypted2 = hex::decode("0c3c312b3b202e6362372365632c346e302c30722f202d62266e38362c312569222b366e272a2830222c490c7229266326202f333a6525262c2d651b6e212624206e2863262b232b2229").unwrap();
        let encrypted3 = hex::decode("0a3a263a2c3c2f6f7331287e682632743c3d3d6f353d2b753c6f25212c31236f353a2172262639362937420674332a722b3d352e3c723f27313a651b682731353772296f372d28302923").unwrap();
        assert_eq!(
            "ICE",
            interpolate_decryptions(decipher_with_known_keysize(&encrypted1, 3)).key
        );
        assert_eq!(
            "NICER",
            interpolate_decryptions(decipher_with_known_keysize(&encrypted2, 5)).key
        );
        assert_eq!(
            "HOTTER",
            interpolate_decryptions(decipher_with_known_keysize(&encrypted3, 6)).key
        );
    }
}
