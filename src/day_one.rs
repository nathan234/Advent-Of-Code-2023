use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//--- Day 1: Trebuchet?! ---
// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
//
// You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
//
// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//
// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
//
// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
//
// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
//
// For example:
//
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
//
// Consider your entire calibration document. What is the sum of all of the calibration values?

/**
 * Combine first and last digits
 **/
fn extract_calibration_value(line: &str) -> Option<i32> {
    let first_digit = line.chars().find(|c| c.is_digit(10));
    let last_digit = line.chars().rev().find(|c| c.is_digit(10));

    match (first_digit, last_digit) {
        (Some(f), Some(l)) => {
            let first_digit_val = f.to_digit(10)? as i32;
            let last_digit_val = l.to_digit(10)? as i32;
            Some(first_digit_val * 10 + last_digit_val)
        }
        _ => None,
    }
}

/**
 * Read the text file and loop over each line, summing the result
 **/
pub fn read_lines_from_file<P>(filename: P) -> io::Result<i32>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut sum: i32 = 0;

    for line in reader.lines() {
        match line {
            Ok(ln) => {
                if let Some(calibration_value) = extract_calibration_value(&ln) {
                    sum += calibration_value;
                }
            }
            Err(e) => println!("Error reading line: {}", e),
        }
    }

    Ok(sum)
}
