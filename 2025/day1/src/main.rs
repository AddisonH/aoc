use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file_content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut dial: i32 = 50;
    const MAX_DIAL: i32 = 99;
    let mut zero_sum = 0;
    
    for line in file_content.lines() {
        let (rotation, value) = line.split_at(1);
        match rotation {
            "L" => {
                // Handle left rotation
                let value: i32 = value.parse().expect("Invalid number");
                dial = (dial + MAX_DIAL + 1 - value) % (MAX_DIAL + 1);
            }
            "R" => {
                // Handle right rotation
                let value: i32 = value.parse().expect("Invalid number");
                dial = (dial + value) % (MAX_DIAL + 1);
            }
            _ => {
                panic!("Invalid rotation");
            }
        }
        if dial == 0 {
            zero_sum += 1;
        }
    }
    println!("Number of times dial was zero: {}", zero_sum);
}