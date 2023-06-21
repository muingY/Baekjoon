use std::io;

fn main() {
    let mut input = String::new();
    let mut input2 = String::new();

    let a: u32;
    let b: u32;
    
    io::stdin().read_line(&mut input).expect("stdin error");
    a = input.trim().parse().unwrap();
    drop(input);

    io::stdin().read_line(&mut input2).expect("stdin error");
    b = input2.trim().parse().unwrap();

    for (_, char_digit) in input2.trim().chars().rev().enumerate() {
        println!("{}", match char_digit.to_digit(10) {
            Some(n) => n * a,
            None => 0,
        });
    }

    println!("{}", a * b);
}