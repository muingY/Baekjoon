use std::io;

fn main() {
    let x: i32;
    let y: i32;

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("stdin error");
    x = input.trim().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("stdin error");
    y = input.trim().parse().unwrap();
    drop(input);

    let quadrant 
        = if x > 0 {
            if y > 0 {
                1
            } else {
                4
            }
        } else {
            if y > 0 {
                2
            } else {
                3
            }
        };

    println!("{}", quadrant);
}
