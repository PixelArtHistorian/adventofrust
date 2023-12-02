use path::Path;
use std::{fs, path};

fn main() {
    let input_path = Path::new("./src/input/day-one");
    let snacks = read_input(input_path);
    let top_three = sort_packs(snacks);
    match top_three.iter().max() {
        Some(num) => println!("{}", num),
        None => println!("There's nothing here"),
    };
    let total: u64 = top_three.iter().take(3).sum();
    println!("{}", total)
}

fn read_input(file_path: &Path) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn sort_packs(snacks: Vec<String>) -> Vec<u64> {
    let mut packs: Vec<u64> = Vec::new();
    let mut current = 0;

    for snack in snacks {
        match snack.parse::<u64>() {
            Ok(num) => current += num,
            Err(_) => {
                packs.push(current);
                current = 0;
            },
        }
    }
    packs.sort();
    packs.reverse();
    packs
}
