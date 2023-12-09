use crate::day_one::{read_lines_from_file, read_lines_from_file_part_two};

mod day_one;

fn main() {
    match read_lines_from_file("src/day_one_input.txt") {
        Ok(value) => println!("Total sum part one: {}", value),
        Err(e) => println!("Error opening file: {}", e)
    }
    match read_lines_from_file_part_two("src/day_one_input.txt") {
        Ok(sum) => println!("Total sum part two: {}", sum),
        Err(e) => println!("Error opening file: {}", e),
    }
}
