use std::io;

fn main() {
    let mut dice_log: [u8;6] = [0;6];

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin error");
    for (_, str_number) in input.split(' ').enumerate() {
        let dice_num: usize = str_number.trim().parse().unwrap();
        dice_log[dice_num - 1] += 1;
    }

    for i in 0..6 {
        if dice_log[i] == 3 {
            println!("{}", 10000 + (i + 1) * 1000);
            return;
        } else if dice_log[i] == 2 {
            println!("{}", 1000 + (i + 1) * 100);
            return;
        }
    }
    for i in (0..6).rev() {
        if dice_log[i] == 1 {
            println!("{}", (i + 1) * 100);
            return;
        }
    }
}
