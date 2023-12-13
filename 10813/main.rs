use std::io;

fn main() {
    let mut input = String::new();

    let mut buckets: Vec<u8> = Vec::new();
    let mut change_count: u8 = 0;
    
    io::stdin().read_line(&mut input).expect("input err");
    for (i, num_str) in input.trim().split(' ').enumerate() {
        if i == 0 {
            let bucket_count: u8 = num_str.trim().parse().expect("parse err");
            for ii in 0..bucket_count {
                buckets.push((ii + 1) as u8);
            }
        } else if i == 1 {
            change_count = num_str.trim().parse().expect("parse err");
        }
    }

    for _ in 0..change_count {
        let mut a: usize = 0;
        let mut b: usize = 0;

        input.clear();
        io::stdin().read_line(&mut input).expect("input err");
        for (i, num_str) in input.trim().split(' ').enumerate() {
            if i == 0 {
                a = num_str.trim().parse().expect("parse err");
            } else if i == 1 {
                b = num_str.trim().parse().expect("parse err");
            }
        }

        a -= 1;
        b -= 1;
        let temp = buckets[a];
        buckets[a] = buckets[b];
        buckets[b] = temp;
    }

    let mut is_space = false;
    for ball in buckets {
        if is_space {
            print!(" ");
        }
        print!("{}", ball);
        is_space = true;
    }
    println!("");
}
