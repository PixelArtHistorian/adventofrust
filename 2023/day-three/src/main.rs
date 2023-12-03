use std::{fs, path::Path};

fn main() {
    match read_input(Path::new("src/input")) {
        Ok(lines) => {
            println!("Solution for part one: {}", solve_day_three_part_one(&lines));
            println!("Solution for part two: {}", solve_day_three_part_two(&lines));
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
    0
}

fn solve_day_three_part_two(input_lines: &Vec<String>) -> u32 {
    0
}

fn parse_engine_specs(engine_specs:&Vec<String>) -> Vec<u32>{
    let mut part_numbers = Vec::<u32>::new();
    
    return part_numbers;
}

#[cfg(test)]
mod tests{
    use std::f32::consts::E;

    use super::*;

    #[test]
    fn part_one_solution_matches_example() {
        let game = vec![
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
        let solution = solve_day_three_part_one(&game);
        assert_eq!(4361, solution);
    }
    #[test]
    fn parse_engine_specs_one_liner_no_parts_numer() {
        let game = vec![
            "467..114..".to_string(),
        ];
        let expected= Vec::<u32>::new();
        let parsed_specs = parse_engine_specs(&game);
        assert_eq!(expected, parsed_specs);
    }
    #[test]
    fn parse_engine_specs_one_liner() {
        let game = vec![
            "467.*114..".to_string(),
        ];
        let expected=vec![114];
        let parsed_specs = parse_engine_specs(&game);
        assert_eq!(expected, parsed_specs);
    }
    #[test]
    fn parse_engine_specs_two_liner() {
        let game = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
        ];
        let expected=vec![467];
        let parsed_specs = parse_engine_specs(&game);
        assert_eq!(expected, parsed_specs);
    }
    #[test]
    fn parse_engine_specs_three_liner() {
        let game = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
        ];
        let expected=vec![467,35];
        let parsed_specs = parse_engine_specs(&game);
        assert_eq!(vec![467,35], parsed_specs);
    }
}