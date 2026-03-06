use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let str = input;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let i = input.trim().parse::<usize>().unwrap();

    println!("{}", str.chars().nth(i - 1).unwrap());
}