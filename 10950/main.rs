use std::io;

fn main() {
    let count: u32;

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("stdin error");
    count = input.trim().parse().unwrap();

    let mut problems = Vec::<(u8, u8)>::new();
    
    for _i in 0..count {
        input.clear();
        io::stdin().read_line(&mut input).expect("stdin error");
        let mut nums: [u8; 2] = [0; 2];
        for (i_ab, num_string) in input.split(' ').enumerate() {
            nums[i_ab] = num_string.trim().parse().unwrap();
        }
        problems.push((nums[0], nums[1]));
    }

    for ab in problems {
        println!("{}", ab.0 + ab.1);
    }
}