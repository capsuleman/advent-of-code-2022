use crate::packet::Packet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub mod packet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut line_iterator = buf_reader.lines().into_iter();
    let mut index: usize = 1;
    let mut result = 0;

    while let (Some(Ok(left_line)), Some(Ok(right_line)), Some(Ok(_line))) = (
        line_iterator.next(),
        line_iterator.next(),
        line_iterator.next(),
    ) {
        let left_packet = Packet::from(left_line);
        let right_packet = Packet::from(right_line);
        let is_pair_ordered = left_packet < right_packet;

        println!(
            "{:?}\nVS\n{:?}\n{:?}\n",
            left_packet, right_packet, is_pair_ordered
        );

        if is_pair_ordered {
            result += index;
        }

        index += 1;
    }
    println!("Result: {result}");
}
