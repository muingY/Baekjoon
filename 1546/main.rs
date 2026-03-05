use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let exam_count = input.trim().parse::<u32>().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let scores: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let max_score = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mut total_score = 0.0;
    for score in scores {
        total_score += score / max_score * 100.0;
    }

    println!("{}", total_score / exam_count as f64);
}
