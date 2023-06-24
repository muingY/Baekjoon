use std::io;

fn main() {
    let year: u32;
    let yun_year: bool;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin error");

    year = input.trim().parse().unwrap();

    yun_year = ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0);
    println!("{}", if yun_year {
        1
    } else {
        0
    });
}
