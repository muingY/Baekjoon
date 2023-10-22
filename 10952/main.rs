use std::io;

fn main() {
    let mut input = String::new();

    let mut results: Vec<u32> = Vec::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("stdin error");

        let mut nums: [u32; 2] = [0; 2];
        for (i, str_num) in input.split(" ").enumerate() {
            nums[i] = str_num.trim().parse().expect("parse error");
        }

        if nums[0] == 0 && nums[1] == 0 {
            break;
        }

        results.push(nums[0] + nums[1]);
    }

    for value in results {
        println!("{}", value);
    }
}
