use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file_path = "real.txt";

    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut maximum_calories = 0;
    let mut current_calories = 0;

    for line in buf_reader.lines() {
        let line_value = line?;
        if line_value != "" {
            current_calories += line_value.parse::<u32>().unwrap();
        } else {
            if current_calories > maximum_calories {
                maximum_calories = current_calories;
            }
            current_calories = 0;
        }
    }
    println!("{}", maximum_calories);

    Ok(())
}
