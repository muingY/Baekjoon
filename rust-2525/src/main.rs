use std::io;

fn main() {
    let mut hour: u32 = 0;
    let mut minute: u32 = 0;
    let mut cook_time: u32 = 0;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin error");
    for (i, input) in input.split(' ').enumerate() {
        if i == 0 {
            hour = input.trim().parse().unwrap();
        } else if i == 1 {
            minute = input.trim().parse().unwrap();
        } else {
            break;
        }
    }
    input.clear();

    io::stdin().read_line(&mut input).expect("stdin error");
    cook_time = input.trim().parse().unwrap();
    drop(input);

    minute += cook_time;

    if 60 <= minute {
        hour += minute / 60;
        minute %= 60;
    }
    if hour >= 24 {
        hour %= 24;
    }

    println!("{} {}", hour, minute);
}
