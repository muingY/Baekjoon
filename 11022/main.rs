use std::io;

fn main() {
        let mut input = String::new();
            io::stdin().read_line(&mut input).expect("input error");

                let count: u32 = input.trim().parse().expect("parse error");

                    let mut result: Vec<(u32, u32)> = Vec::new();

                        for _i in 0..count {
                                    input.clear();
                                            io::stdin().read_line(&mut input).expect("input error");

    let mut nums: [u32; 2] = [0; 2];

                                                    for (ii, num_str) in input.split(' ').enumerate() {
                                                                            nums[ii] = num_str.trim().parse().expect("parse error");
                                                                                    }

                                                                    result.push((nums[0], nums[1]));
                                                                        }

                            for (i, value) in result.iter().enumerate() {
                                        println!("Case #{}: {} + {} = {}", i + 1, value.0, value.1, value.0 + value.1);
                                            }
}
