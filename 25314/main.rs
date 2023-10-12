use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin error");

    let byte: u32 = input.trim().parse().expect("parse error");

    for _i in 0..(byte / 4) {
        print!("long ");
    }
    println!("int");
}