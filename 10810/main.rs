use std::io;

fn main() {
    let mut input = String::new();

    let mut buckets: Vec<u8> = Vec::new();
    let mut count: usize = 0;
    io::stdin().read_line(&mut input).expect("input err");
    for (i, num_str) in input.trim().split(' ').enumerate() {
        if i == 0 {
            let bucket_count: usize = num_str.trim().parse().expect("parse err");
            buckets = vec![0; bucket_count];
        } else if i == 1 {
            count = num_str.trim().parse().expect("parse err");
        }
    }

    for _i in 0..count {
        let mut ball_info: [u8; 3] = [0, 0, 0];
        input.clear();
        io::stdin().read_line(&mut input).expect("input err");
        for (ii, num_str) in input.trim().split(' ').enumerate() {
            ball_info[ii] = num_str.trim().parse().expect("parse err");
        }

        for ii in (ball_info[0] - 1)..ball_info[1] {
            buckets[ii as usize] = ball_info[2];
        }
    }

    let mut is_space = false;
    for ball_num in buckets {
        if is_space {
            print!(" ");
        }
        print!("{}", ball_num);
        is_space = true;
    }
    println!("");
}
