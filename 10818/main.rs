use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("input err");
    let count: usize = input.trim().parse().expect("parse err");

    input.clear();
    io::stdin().read_line(&mut input).expect("input err");
    let mut nums: Vec<i32> = Vec::new();
    for (i, num_str) in input.trim().split(' ').enumerate() {
        if i >= count {
            panic!("couny overflow!");
        }
        let num: i32 = num_str.trim().parse().expect("parse err");
        nums.push(num);
    }

    println!("{} {}", nums.iter().min().unwrap(), nums.iter().max().unwrap());
}
