use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const ALLY: [char; 3] = ['X', 'Y', 'Z'];
const ENEMY: [char; 3] = ['A', 'B', 'C'];
// Rock, Paper, Scissors

fn main() -> std::io::Result<()> {
    let file_path = "real.txt";

    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut total_score: usize = 0;

    for line in buf_reader.lines() {
        let line_value = line?;
        let enemy = line_value.chars().nth(0).unwrap();
        let ally = line_value.chars().nth(2).unwrap();

        total_score += get_score(ally, enemy);
    }

    println!("{}", total_score);
    Ok(())
}

fn get_score(ally: char, enemy: char) -> usize {
    let ally_index = index_of(&ALLY, ally);
    let enemy_index = index_of(&ENEMY, enemy);

    let shape_score = ally_index + 1;
    let diff_index: i8 = i8::try_from(3 + ally_index - enemy_index).unwrap() % 3;

    let outcome_score = match diff_index {
        0 => 3,
        1 => 6,
        _ => 0,
    };

    shape_score + outcome_score
}

fn index_of<T: std::cmp::PartialEq + Copy>(array: &[T], value: T) -> usize {
    array.iter().position(|&r| r == value).unwrap()
}
