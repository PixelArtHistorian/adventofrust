use regex::Regex;
use std::{fs, path::Path};

fn main() {
    match read_input(Path::new("src/input")) {
        Ok(lines) => {
            println!("Solution for part one: {}", solve_day_two_part_one(&lines));
            println!("Solution for part two: {}", solve_day_two_part_two(&lines));
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

fn solve_day_two_part_one(input_lines: &Vec<String>) -> u32 {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    input_lines
        .into_iter()
        .map(|line| {
            let current_game = parse_game(line);
            if !current_game.is_game_possible(&bag) {
                return 0;
            }
            return current_game.id;
        })
        .sum()
}

fn solve_day_two_part_two(input_lines: &Vec<String>) -> u32 {
    input_lines
        .into_iter()
        .map(|line| parse_game(line).calc_power_of_set())
        .sum()
}

fn parse_game(game: &str) -> Game {
    let id_pattern = Regex::new(r"Game (\d+)").unwrap();
    let red_pattern = Regex::new(r"(\d+) red").unwrap();
    let green_pattern = Regex::new(r"(\d+) green").unwrap();
    let blue_pattern = Regex::new(r"(\d+) blue").unwrap();

    Game {
        id: parse_game_element(id_pattern, game),
        red: parse_game_element(red_pattern, game),
        green: parse_game_element(green_pattern, game),
        blue: parse_game_element(blue_pattern, game),
    }
}

fn parse_game_element(regex: Regex, game: &str) -> u32 {
    regex
        .captures_iter(game)
        .map(|c| c.extract::<1>())
        .map(|e| {
            e.0.split(" ")
                .filter_map(|s| s.parse::<u32>().ok())
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}
impl Game {
    fn is_game_possible(&self, bag: &Bag) -> bool {
        bag.red >= self.red && bag.green >= self.green && bag.blue >= self.blue
    }
    fn calc_power_of_set(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn game_is_parsed_from_input() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string();
        let parsed_game = parse_game(&game);
        assert_eq!(1, parsed_game.id);
        assert_eq!(4, parsed_game.red);
        assert_eq!(2, parsed_game.green);
        assert_eq!(6, parsed_game.blue);
    }
    #[test]
    fn game_is_possible_when_bag_is_greater() {
        let game = Game {
            id: 1,
            red: 4,
            green: 4,
            blue: 4,
        };
        let bag = Bag {
            red: 5,
            green: 5,
            blue: 5,
        };
        let is_possible = game.is_game_possible(&bag);
        assert_eq!(true, is_possible);
    }
    #[test]
    fn game_is_not_possible_when_bag_is_lesser() {
        let game = Game {
            id: 1,
            red: 4,
            green: 4,
            blue: 4,
        };
        let bag = Bag {
            red: 3,
            green: 3,
            blue: 3,
        };
        let is_possible = game.is_game_possible(&bag);
        assert_eq!(false, is_possible);
    }
    #[test]
    fn game_is_not_possible_when_bag_red_is_lesser() {
        let game = Game {
            id: 1,
            red: 4,
            green: 4,
            blue: 4,
        };
        let bag = Bag {
            red: 3,
            green: 5,
            blue: 5,
        };
        let is_possible = game.is_game_possible(&bag);
        assert_eq!(false, is_possible);
    }
    #[test]
    fn game_is_not_possible_when_bag_green_is_lesser() {
        let game = Game {
            id: 1,
            red: 4,
            green: 4,
            blue: 4,
        };
        let bag = Bag {
            red: 5,
            green: 3,
            blue: 5,
        };
        let is_possible = game.is_game_possible(&bag);
        assert_eq!(false, is_possible);
    }
    #[test]
    fn game_is_not_possible_when_bag_blue_is_lesser() {
        let game = Game {
            id: 1,
            red: 4,
            green: 4,
            blue: 4,
        };
        let bag = Bag {
            red: 5,
            green: 5,
            blue: 3,
        };
        let is_possible = game.is_game_possible(&bag);
        assert_eq!(false, is_possible);
    }
    #[test]
    fn part_one_solution_matches_example() {
        let game = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        let solution = solve_day_two_part_one(&game);
        assert_eq!(8, solution);
    }
    #[test]
    fn part_two_solution_matches_example() {
        let game = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        let solution = solve_day_two_part_two(&game);
        assert_eq!(2286, solution);
    }
}
