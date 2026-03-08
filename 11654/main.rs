use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let c = input.trim().chars().next().unwrap() as u8;
    println!("{}", c);
}
