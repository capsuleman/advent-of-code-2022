use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() -> std::io::Result<()> {
    let file_path = "real.txt";

    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut total_priority = 0;

    for line in buf_reader.lines() {
        let bag = line?;
        let (first_part, second_part) = split_bag(&bag);
        let duplicate = find_duplicate(first_part, second_part);
        total_priority += get_letter_score(duplicate);
    }

    println!("{}", total_priority);
    Ok(())
}

fn split_bag(bag: &str) -> (&str, &str) {
    let bag_size = bag.len();
    let first_part = &bag[..(bag_size / 2)];
    let second_part = &bag[(bag_size / 2)..];

    (first_part, second_part)
}

fn find_duplicate(first_part: &str, second_part: &str) -> char {
    for element in first_part.chars().into_iter() {
        if second_part.contains(element) {
            return element;
        }
    }

    '0'
}

fn get_letter_score(letter: char) -> usize {
    ALPHABET
        .chars()
        .into_iter()
        .position(|r| r == letter)
        .unwrap()
        + 1
}
