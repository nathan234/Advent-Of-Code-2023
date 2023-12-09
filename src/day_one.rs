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

//--- Part Two ---
// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
//
// Equipped with this new information, you now need to find the real first and last digit on each line. For example:
//
// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
//
// What is the sum of all of the calibration values?

pub fn read_lines_from_file_part_two<P>(filename: P) -> io::Result<i32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let nums = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];
    let mut total = 0;
    for line in reader.lines() {
        let mut first = None;
        match line {
            Ok(line) => {
                'out: for i in 0..line.len() {
                    for (index, num) in nums.iter().enumerate() {
                        if i + num.len() > line.len() {
                            continue;
                        }
                        if line[i..i + num.len()] == **num {
                            first = Some(1 + index / 2);
                            break 'out;
                        }
                    }
                }
                let Some(first) = first else { panic!("invalid input") };

                let mut last = None;
                'out: for i in (0..line.len()).rev() {
                    for (index, num) in nums.iter().enumerate() {
                        if i + num.len() > line.len() {
                            continue;
                        }
                        if line[i..i + num.len()] == **num {
                            last = Some(1 + index / 2);
                            break 'out;
                        }
                    }
                }
                let Some(last) = last else { panic!("invalid input") };
                total += 10 * first as i32 + last as i32;
            }
            Err(_) => {}
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use crate::day_one::read_lines_from_file_part_two;
    #[test]
    fn test_sum() {
        let actual_output = match read_lines_from_file_part_two("src/day_one_test_input.txt") {
            Ok(sum) => sum,
            Err(_) => 0,
        };

        assert_eq!(actual_output, 281, "Failed");
    }

    #[test]
    fn test_sum_actual() {
        let actual_output = match read_lines_from_file_part_two("src/day_one_input.txt") {
            Ok(sum) => sum,
            Err(_) => 0,
        };

        assert_eq!(actual_output, 53389, "Failed");
    }
}
