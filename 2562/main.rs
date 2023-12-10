use std::io;

fn find_max(nums: [u8; 9]) -> (usize, u8) {
    let mut max_index: usize = 0;
    let mut max_value: u8 = 0;

    for i in 0..9 {
        if max_value < nums[i] {
            max_index = i;
            max_value = nums[i];
        }
    }

    return (max_index, max_value);
}

fn main() {
    let mut input = String::new();

    let mut nums: [u8; 9] = [0; 9];
    for i in 0..9 {
        input.clear();
        io::stdin().read_line(&mut input).expect("input err");
        nums[i] = input.trim().parse().expect("parse err");
    }

    let (max_index, max_value) = find_max(nums);
    println!("{}\n{}", max_value, max_index + 1);
}
