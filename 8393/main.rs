use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin fail!");

    let n: u32 = input.trim().parse().expect("parse fail!");
    let mut result: u32 = 0;

    for i in 1..=n {
        result += i;
    }
    
    println!("{}", result);
}