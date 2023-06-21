use std::io;

fn main() {
    let score: u8;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin error");

    score = input.trim().parse().unwrap();

    match score {
        90..=100 => {
            println!("A");
        },
        80..=89 => {
            println!("B");
        },
        70..=79 => {
            println!("C");
        },
        60..=69 => {
            println!("D");
        },
        _ => {
            println!("F");
        }
    }
}
