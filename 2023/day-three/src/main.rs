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

#[cfg(test)]
mod tests{
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
}