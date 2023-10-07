use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin error");

    let year: u32 = input.trim().parse().unwrap();

    println!("{}", year - 543);
}