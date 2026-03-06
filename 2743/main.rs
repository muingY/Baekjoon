use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let str = input;

    println!("{}", str.len() - 1);
}
