use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file_path = "real.txt";

    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut x_history = vec![1];

    for line in buf_reader.lines() {
        let line_value = line?;

        if line_value == "noop" {
            x_history.push(*x_history.last().unwrap());
        } else {
            let added_value = &line_value[5..].parse::<i32>().unwrap();
            x_history.push(*x_history.last().unwrap());
            x_history.push(*x_history.last().unwrap() + added_value);
        }
    }

    let mut response = 0;
    for i in vec![20, 60, 100, 140, 180, 220] {
        println!("{} {}", i, x_history[i - 1]);
        response += x_history[i - 1] * (i as i32);
    }

    println!("{response}");

    Ok(())
}
