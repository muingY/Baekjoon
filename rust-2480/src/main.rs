use std::io;

fn main() {
    let mut dicePool: u16 = 0x000000000000;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin error");
    for (i, str_number) in input.split(' ').enumerate() {
        
    }
}
