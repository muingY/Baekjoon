use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("stdin error");
    let str_numbers = input.split(' ');

    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut c: u64 = 0;

    for (i, str_number) in str_numbers.enumerate() {
        match i {
            0 => { a = str_number.trim().parse().unwrap(); },
            1 => { b = str_number.trim().parse().unwrap(); },
            2 => { c = str_number.trim().parse().unwrap(); },
            _ => { () },
        }
    }

    println!("{}", a + b + c);
}