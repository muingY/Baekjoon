use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin error");

    let bill: u32 = input.trim().parse().expect("parse error");

    input.clear();
    io::stdin().read_line(&mut input).expect("stdin error");

    let count = input.trim().parse().expect("parse error");

    let mut product_values: Vec<u32> = Vec::new();

    for _i in 0..count {
        input.clear();
        io::stdin().read_line(&mut input).expect("stdin error");

        let mut nums: [u32; 2] = [0; 2];
        for (ii, num_string) in input.split(' ').enumerate() {
            nums[ii] = num_string.trim().parse().unwrap();
        }

        product_values.push(nums[0] * nums[1]);
    }

    if product_values.iter().sum::<u32>() == bill {
        println!("Yes");
    } else {
        println!("No");
    }
}