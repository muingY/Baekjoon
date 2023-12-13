use std::io;

fn main() {
    let mut input = String::new();

    let mut good_students: Vec<u8> = Vec::new();
    for _ in 0..28 {
        input.clear();
        io::stdin().read_line(&mut input).expect("input err");
        
        let student_num: u8 = input.trim().parse().expect("parse err");
        good_students.push(student_num);
    }

    good_students.sort();

    let mut current_index = 0;
    for i in 1..=30 {
        if current_index < good_students.len() && good_students[current_index] == i {
            current_index += 1;
        } else {
            println!("{}", i);
        }
    }
}
