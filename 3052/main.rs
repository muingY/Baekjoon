use std::io;
use std::collections::BTreeSet;

fn main() {
    let mut input = String::new();

    let mut els: BTreeSet<u16> = BTreeSet::new();
    let mut diff_count = 0_u8;
    for _ in 0..10 {
        io::stdin().read_line(&mut input).expect("input err");
        let num: u16 = input.trim().parse().expect("parse err");
        input.clear();

        let el = num % 42;
        if !els.contains(&el) {
            diff_count += 1;
            els.insert(el);
        }
    }

    println!("{}", diff_count);
}
