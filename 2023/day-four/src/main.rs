use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    match read_input(Path::new("src/input")) {
        Ok(lines) => {
            println!("Solution for part one: {}", solve_day_four_part_one(&lines));
            println!("Solution for part two: {}", solve_day_four_part_two(&lines));
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

fn solve_day_four_part_one(input_lines: &Vec<String>) -> u32 {
    input_lines
        .iter()
        .filter_map(|c| compute_card_points(c))
        .map(|card| {
            if card.matches == 0 {
                0
            } else {
                2u32.pow(u32::try_from(card.matches - 1).unwrap())
            }
        })
        .sum()
}

fn solve_day_four_part_two(input_lines: &Vec<String>) -> u32 {
    0
}

fn compute_card_points(card: &String) -> Option<Card> {
    let points_pattern =
        Regex::new(r"Card\s+(?P<card_number>\d+):\s*(?P<before>[\d\s]+)\s*\|\s*(?P<after>[\d\s]+)")
            .unwrap();

    let caps = points_pattern.captures(&card).unwrap();

    return match caps.name("card_number") {
        Some(x) => {
            let card_number = x.as_str().to_string().parse::<u32>().unwrap();
            let card_numbers: HashMap<&str, Vec<u32>> = points_pattern
                .capture_names()
                .flatten()
                .filter_map(|name| {
                    Some((
                        name,
                        caps.name(name)?
                            .as_str()
                            .to_string()
                            .split_whitespace()
                            .map(|number| number.parse::<u32>().unwrap())
                            .collect(),
                    ))
                })
                .collect();

            let winning_numbers = card_numbers.get("before").unwrap();
            let numbers = card_numbers.get("after").unwrap();

            let matches = numbers
                .into_iter()
                .filter(|num| winning_numbers.contains(num))
                .count();

            Some(Card {
                number: (card_number),
                matches: (u32::try_from(matches).unwrap()),
            })
        }
        _ => None,
    };
}

struct Card {
    number: u32,
    matches: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_solution_matches_example() {
        let cards = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];
        let solution = solve_day_four_part_one(&cards);
        assert_eq!(13, solution);
    }

    #[test]
    fn part_two_solution_matches_example() {
        let cards = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];
        let solution = solve_day_four_part_one(&cards);
        assert_eq!(30, solution);
    }

    #[test]
    fn card_1_has_8_points() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string();
        let card = compute_card_points(&line);
        assert_eq!(4, card.unwrap().matches);
    }
    #[test]
    fn card_2_has_2_points() {
        let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string();
        let card = compute_card_points(&line);
        assert_eq!(2, card.unwrap().matches);
    }
    #[test]
    fn card_3_has_2_points() {
        let line = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string();
        let card = compute_card_points(&line);
        assert_eq!(2, card.unwrap().matches);
    }
    #[test]
    fn card_4_has_1_points() {
        let line = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string();
        let card = compute_card_points(&line);
        assert_eq!(1, card.unwrap().matches);
    }
    #[test]
    fn card_5_has_0_points() {
        let line = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string();
        let card = compute_card_points(&line);
        assert_eq!(0, card.unwrap().matches);
    }
    #[test]
    fn card_6_has_0_points() {
        let line = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();
        let card = compute_card_points(&line);
        assert_eq!(0, card.unwrap().matches);
    }
}
