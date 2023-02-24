#[derive(Debug)]
pub struct LineScore {
    pub score: usize,
    pub line: String,
}

fn byte_times_in_line(line: &[u8]) -> LineScore {
    if line.len() % 16 != 0 {
        panic!("Line length is not devisible by 16")
    }
    let mut line_score = 0;
    for pos in 0..line.len() / 16 {
        let pattern = &line[pos * 16..(pos + 1) * 16];
        for i in pos + 1..line.len() / 16 {
            if pattern.eq(&line[i * 16..(i + 1) * 16]) {
                line_score += 1;
            }
        }
    }
    LineScore {
        score: line_score,
        line: String::from_utf8(line.to_vec()).unwrap(),
    }
}

pub fn find_most_repetetive_line(input: &str) -> LineScore {
    input
        .lines()
        .map(|line| byte_times_in_line(line.as_bytes()))
        .max_by(|a, b| a.score.cmp(&b.score))
        .unwrap()
}
