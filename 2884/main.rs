use std::{io};

fn main() {
    let mut hour: i32 = 0;
    let mut minute: i32 = 0;

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("stdin error");
    for (i, str_number) in input.split(' ').enumerate() {
        if i == 0 {
            hour = str_number.trim().parse().unwrap();
        } else if i == 1 {
            minute = str_number.trim().parse().unwrap();
        }
    }

    if minute < 45 {
        let nm = minute - 45;
        minute = 60 + nm;
        if hour == 0 {
            hour = 23;
        } else {
            hour -= 1;
        }
    } else {
        minute -= 45;
    }

    println!("{} {}", hour, minute);
}
