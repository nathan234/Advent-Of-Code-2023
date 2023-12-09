use crate::day_one::read_lines_from_file;

mod day_one;

fn main() {
    match read_lines_from_file("src/day_one_input.txt") {
        Ok(value) => {
            println!("sum: {}", value)
        }
        Err(_) => {
            println!("Error")
        }
    }
}
