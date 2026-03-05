use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut baskets: Vec<u32> = vec![0; numbers[0] as usize];
    for i in 0..baskets.len() {
        baskets[i] = i as u32 + 1;
    }

    for _ in 0..numbers[1] {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input_change_rule: Vec<u32> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let flip_data = &baskets.clone()[input_change_rule[0] as usize - 1..input_change_rule[1] as usize];
        for i in 0..flip_data.len() {
            baskets[input_change_rule[0] as usize - 1 + i] = flip_data[flip_data.len() - 1 - i];
        }
    }

    for basket_value in baskets {
        print!("{} ", basket_value);
    }
    println!();
}
