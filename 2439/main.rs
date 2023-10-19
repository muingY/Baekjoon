use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");

    let height: u8 = input.trim().parse().expect("parse error");
    let mut width: u8 = 1;
    
    for _i in 0..height {
        for _ii in 0..(height - width) {
            print!(" ");
        }
        for _ii in 0..width {                           print!("*");
        }                                           width += 1;
        println!();
    }
}
