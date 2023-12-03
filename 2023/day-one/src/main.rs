use std::{collections::HashMap, fs, path::Path};

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
    input_lines
        .iter()
        .map(|line| {
            let reverse_line: String = line.chars().rev().collect();
            let mut digits: String = find_digit(line, None);
            digits.push_str(&find_digit(&reverse_line, None));
            digits.parse::<u32>().unwrap()
        })
        .sum()
}

fn solve_day_one_part_two(input_lines: &Vec<String>) -> u32 {
    let digit_mappings: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    input_lines
        .iter()
        .map(|line| {
            let reverse_line = line.chars().rev().collect::<String>();
            let mut digits = find_digit(line, Some(digit_mappings.clone()));
            digits.push_str(&find_digit(&reverse_line, Some(digit_mappings.clone())));
            digits.parse::<u32>().unwrap()
        })
        .sum()
}

fn find_digit(characters: &str, digit_map: Option<HashMap<&str, u32>>) -> String {
    let mut digit_string = "".to_string();
    let mut buffer = "".to_string();
    let map = match digit_map {
        Some(map) => map,
        None => HashMap::new(),
    };
    for char in characters.chars() {
        match char.to_digit(10) {
            Some(_digit) => {
                digit_string.push(char);
                break;
            }
            _ => {
                buffer.push(char);
            }
        }
        let rev_buffer: String = buffer.chars().rev().collect();
        match map
            .keys()
            .copied()
            .find(|k| buffer.contains(k) || rev_buffer.contains(k))
        {
            Some(key) => {
                digit_string.push_str(&map.get(key).unwrap().to_string());
                break;
            }
            None => (),
        }
    }
    return digit_string;
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
        let input = vec!["1abc2".to_string()];
        assert_eq!(12, solve_day_one_part_one(&input));
    }
    #[test]
    fn calibration_line_contains_even_number_of_numbers() {
        let input = vec!["pqr3stu8vwx".to_string()];
        assert_eq!(38, solve_day_one_part_one(&input));
    }
    #[test]
    fn calibration_line_contains_uneven_number_of_numbers() {
        let input = vec!["a1b2c3d4e5f".to_string()];
        assert_eq!(15, solve_day_one_part_one(&input));
    }
    #[test]
    fn calibration_line_contains_one_number() {
        let input = vec!["treb7uchet".to_string()];
        assert_eq!(77, solve_day_one_part_one(&input));
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
        assert_eq!(281, solve_day_one_part_two(&input));
    }
    #[test]
    fn calibration_line_starts_and_ends_with_number_as_chars() {
        let input = vec!["two1nine".to_string()];
        assert_eq!(29, solve_day_one_part_two(&input));
    }
    #[test]
    fn calibration_line_starts_and_ends_with_number_as_chars_no_digits() {
        let input = vec!["eightwothree".to_string()];
        assert_eq!(83, solve_day_one_part_two(&input));
    }
    #[test]
    fn calibration_line_contains_even_number_of_numbers_as_chars_and_digits() {
        let input = vec!["abcone2threexyz".to_string()];
        assert_eq!(13, solve_day_one_part_two(&input));
    }
    #[test]
    fn calibration_line_ends_with_even_number_of_numbers_as_chars_and_digits() {
        let input = vec!["xtwone3four".to_string()];
        assert_eq!(24, solve_day_one_part_two(&input));
    }
    #[test]
    fn calibration_line_contains_uneven_number_of_numbers_as_chars_and_digits() {
        let input = vec!["4nineeightseven2".to_string()];
        assert_eq!(42, solve_day_one_part_two(&input));
    }
    #[test]
    fn calibration_line_contains_one_number_as_chars() {
        let input = vec!["zoneight234".to_string()];
        assert_eq!(14, solve_day_one_part_two(&input));
    }
    #[test]
    fn calibration_line_ends_with_one_number_as_chars() {
        let input = vec!["7pqrstsixteen".to_string()];
        assert_eq!(76, solve_day_one_part_two(&input));
    }
}
