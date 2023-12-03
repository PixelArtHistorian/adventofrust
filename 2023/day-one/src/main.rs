use std::{fs, path::Path};

fn main() {
    match read_input(Path::new("src/input")) {
        Ok(lines) => {
            println!("Solution for part one: {}", solve_day_one_part_one(&lines));
            println!("Solution for part two: {}", solve_day_one_part_two(&lines));
        }
        Err(message) => println!("{}", message),
    }
}

fn read_input(path: &Path) -> Result<Vec<String>, String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content.lines().map(String::from).collect()),
        _ => Err(format!("Could not read file {}.", path.display())),
    }
}

fn solve_day_one_part_one(input_lines: &Vec<String>) -> u32 {
    input_lines.iter().map(|line| parse_calibration(line)).sum()
}

fn solve_day_one_part_two(input_lines: &Vec<String>) -> u32 {
    input_lines.iter().map(|line| parse_calibration(line)).sum()
}

fn parse_calibration(line: &str) -> u32 {
    let mut digits_string = "".to_string();

    for char in line.chars() {
        match char.to_digit(10) {
            Some(_digit) => {
                digits_string.push(char);
                break;
            }
            _ => continue,
        }
    }
    for char in line.chars().rev() {
        match char.to_digit(10) {
            Some(_digit) => {
                digits_string.push(char);
                break;
            }
            _ => continue,
        }
    }

    digits_string.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_matches_example() {
        let input = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];
        assert_eq!(142, solve_day_one_part_one(&input));
    }
    #[test]
    fn calibration_line_starts_and_ends_with_number() {
        let input = "1abc2";
        assert_eq!(12, parse_calibration(input));
    }
    #[test]
    fn calibration_line_contains_even_number_of_numbers() {
        let input = "pqr3stu8vwx";
        assert_eq!(38, parse_calibration(input));
    }
    #[test]
    fn calibration_line_contains_uneven_number_of_numbers() {
        let input = "a1b2c3d4e5f";
        assert_eq!(15, parse_calibration(input));
    }
    #[test]
    fn calibration_line_contains_one_number() {
        let input = "treb7uchet";
        assert_eq!(77, parse_calibration(input));
    }
    #[test]
    fn solution_part_two_matches_example() {
        let input = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];
        assert_eq!(142, solve_day_one_part_two(&input));
    }
    #[test]
    fn calibration_line_starts_and_ends_with_number_as_chars_and_digits() {
        let input = "two1nine";
        assert_eq!(29, parse_calibration(input));
    }
    #[test]
    fn calibration_line_starts_and_ends_with_number_as_chars() {
        let input = "eightwothree";
        assert_eq!(83, parse_calibration(input));
    }
    #[test]
    fn calibration_line_contains_even_number_of_numbers_as_chars_and_digits() {
        let input = "abcone2threexyz";
        assert_eq!(13, parse_calibration(input));
    }
    #[test]
    fn calibration_line_ends_with_even_number_of_numbers_as_chars_and_digits() {
        let input = "xtwone3four";
        assert_eq!(24, parse_calibration(input));
    }
    #[test]
    fn calibration_line_contains_uneven_number_of_numbers_as_chars_and_digits() {
        let input = "4nineeightseven2";
        assert_eq!(42, parse_calibration(input));
    }
    #[test]
    fn calibration_line_contains_one_number_as_chars() {
        let input = "zoneight234";
        assert_eq!(14, parse_calibration(input));
    }
    #[test]
    fn calibration_line_ends_with_one_number_as_chars() {
        let input = "7pqrstsixteen";
        assert_eq!(76, parse_calibration(input));
    }
}
