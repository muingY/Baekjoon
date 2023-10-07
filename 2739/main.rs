use std::io;

fn main() {
    let num: u8;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin error");

    num = input.trim().parse().unwrap();

    for i in 1..=9 {
        println!("{} * {} = {}", num, i, num * i);
    }
}
