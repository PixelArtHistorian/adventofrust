use std::{fs, path::Path};

fn main() {
    match read_input(Path::new("src/input")) {
        Ok(lines) => {
            println!(
                "Solution for part one: {}",
                solve_day_three_part_one(&lines)
            );
            println!(
                "Solution for part two: {}",
                solve_day_three_part_two(&lines)
            );
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

fn solve_day_three_part_one(input_lines: &Vec<String>) -> u32 {
    parse_engine_specs(input_lines)
        .into_iter()
        .for_each(|number| println!("{}", number));
    parse_engine_specs(input_lines).into_iter().sum()
}

fn solve_day_three_part_two(input_lines: &Vec<String>) -> u32 {
    0
}

fn parse_engine_specs(engine_specs: &Vec<String>) -> Vec<u32> {
    let mut part_numbers = Vec::<u32>::new();
    for index_and_line in engine_specs.iter().enumerate() {
        let line_index = index_and_line.0;
        let line = index_and_line.1;
        let mut digits_buffer = DigitsBuffer::new();

        for index_and_char in line.char_indices() {
            let char_index = index_and_char.0;
            let char = index_and_char.1;

            if is_adjacent(line_index, char_index, engine_specs)
                && !digits_buffer.is_adjacent
                && char != '.'
            {
                digits_buffer.is_adjacent = true;
            }
            if char.is_digit(10) {
                digits_buffer.digits.push(char);
            } else {
                if digits_buffer.can_be_captured() {
                    part_numbers.push(digits_buffer.digits.parse::<u32>().unwrap())
                }
                digits_buffer = DigitsBuffer::new();
            }
            if line.len() - 1 == char_index && digits_buffer.can_be_captured() {
                part_numbers.push(digits_buffer.digits.parse::<u32>().unwrap())
            }
        }
    }
    return part_numbers;
}

fn is_adjacent(line: usize, column: usize, vector: &Vec<String>) -> bool {
    let start_line = if line == 0 { 0 } else { line - 1 };
    let end_line = if line >= vector.len() - 1 {
        vector.len()
    } else {
        line + 2
    };

    let start_column = if column == 0 { 0 } else { column - 1 };
    let end_column = if column >= vector[0].len() - 1 {
        vector[0].len()
    } else {
        column + 2
    };

    vector[start_line..end_line]
        .into_iter()
        .enumerate()
        .map(|y| &y.1[start_column..end_column])
        .any(|slice| slice.chars().any(|x| !x.is_digit(10) && x != '.'))
}

struct DigitsBuffer {
    is_adjacent: bool,
    digits: String,
}
impl DigitsBuffer {
    fn new() -> DigitsBuffer {
        DigitsBuffer {
            is_adjacent: false,
            digits: String::new(),
        }
    }
    fn can_be_captured(&self) -> bool {
        self.is_adjacent && self.digits.len() > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_solution_matches_example() {
        let specs = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        let solution = solve_day_three_part_one(&specs);
        assert_eq!(4361, solution);
    }
    #[test]
    fn parse_engine_specs_one_liner_no_parts_numer() {
        let specs = vec!["467..114..".to_string()];
        let expected = Vec::<u32>::new();
        let parsed_specs = parse_engine_specs(&specs);
        assert_eq!(expected, parsed_specs);
    }
    #[test]
    fn parse_engine_specs_one_liner_before() {
        let specs = vec!["467.114*.".to_string()];
        let expected = vec![114];
        let parsed_specs = parse_engine_specs(&specs);
        assert_eq!(expected, parsed_specs);
    }
    #[test]
    fn parse_engine_specs_one_liner_before_ends_with_number() {
        let specs = vec!["467..*114".to_string()];
        let expected = vec![114];
        let parsed_specs = parse_engine_specs(&specs);
        assert_eq!(expected, parsed_specs);
    }
    #[test]
    fn parse_engine_specs_one_liner_after() {
        let specs = vec!["467.114*.".to_string()];
        let expected = vec![114];
        let parsed_specs = parse_engine_specs(&specs);
        assert_eq!(expected, parsed_specs);
    }
    #[test]
    fn parse_engine_specs_one_liner_two_captures() {
        let specs = vec!["467*114.".to_string()];
        let expected = vec![467, 114];
        let parsed_specs = parse_engine_specs(&specs);
        assert_eq!(expected, parsed_specs);
    }
    #[test]
    fn parse_engine_specs_two_liner() {
        let specs = vec!["467..114..".to_string(), "...*......".to_string()];
        let expected = vec![467];
        let parsed_specs = parse_engine_specs(&specs);
        assert_eq!(expected, parsed_specs);
    }
    #[test]
    fn parse_engine_specs_two_liner_below() {
        let specs = vec!["..35.*633.".to_string(), "......#...".to_string()];
        let expected = vec![633];
        let parsed_specs = parse_engine_specs(&specs);
        assert_eq!(expected, parsed_specs);
    }
    #[test]
    fn parse_engine_specs_three_liner() {
        let specs = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
        ];
        let expected = vec![467, 35];
        let parsed_specs = parse_engine_specs(&specs);
        assert_eq!(expected, parsed_specs);
    }
}
