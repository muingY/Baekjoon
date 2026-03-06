use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let t = input.trim().parse::<usize>().unwrap();

    let mut results = Vec::<String>::new();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();

        let str = format!(
            "{}{}",
            input.chars().next().unwrap(),
            input.chars().last().unwrap()
        );
        results.push(str);
    }

    for result in results {
        println!("{}", result);
    }
}
