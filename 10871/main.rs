use std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("input err");
    let mut count: u16 = 0_u16;
    let mut x: u16 = 0_u16;
    for (i, num_str) in input.trim().split(' ').enumerate() {
        if i == 0 {
            count = num_str.trim().parse().expect("parse err");
        } else if i == 1 {
            x = num_str.trim().parse().expect("parse err");
        }
    }
    
    input.clear();
    io::stdin().read_line(&mut input).expect("parse err");
    let mut nums: Vec<u16> = Vec::new();
    for (i, num_str) in input.trim().split(' ').enumerate() {
        if i >= count.into() {
            panic!("count overflow!");
        }
        let num: u16 = num_str.parse().expect("parse err");
        nums.push(num);
    }

    let mut prev_space = false;
    for num in nums {
        if num < x {
            if prev_space {
                print!(" ");
            }
            print!("{}", num);
            prev_space = true;
        }
    }
    println!("");
}
