use day_02::process_part_2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Result: {}", process_part_2(&file));
}