use std::io;

fn main() {
    let mut input: String = String::new();
    
    io::stdin().read_line(&mut input).expect("inout err");
    let count: u8 = input.trim().parse().expect("parse err");

    let mut nums: Vec<i8> = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).expect("input err");
    for (i, num_str) in input.trim().split(' ').enumerate() {
        if i >= count.into() {
            panic!("split count overflow");
        }
        let num: i8 = num_str.trim().parse().expect("parse err");
        nums.push(num);
    }

    input.clear();
    io::stdin().read_line(&mut input).expect("input err");
    let find_target: i8 = input.trim().parse().expect("parse err");

    let mut result: u8 = 0_u8;
    for num in nums {
        if num == find_target {
            result += 1_u8;
        }
    }

    println!("{}", result);
}
