use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut line_iterator = buf_reader.lines().into_iter();

    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };

    let mut tail_positions_set: HashSet<Position> = HashSet::new();
    tail_positions_set.insert(tail_position.clone());

    while let Some(Ok(line)) = line_iterator.next() {
        let parameters = line.split(' ').collect::<Vec<&str>>();
        let direction = parameters[0];
        let direction = match direction {
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }
        .expect("Could not parse direction");
        let steps = parameters[1]
            .parse::<u64>()
            .expect("Could not parse number of step");

        for _ in 0..steps {
            (head_position, tail_position) = move_rope(head_position, tail_position, &direction);
            tail_positions_set.insert(tail_position.clone());
        }
    }
    println!("{}", tail_positions_set.len());
}

fn move_rope(
    head_position: Position,
    tail_position: Position,
    direction: &Direction,
) -> (Position, Position) {
    let new_head_position = move_head_rope(&head_position, direction);
    let new_tail_position = move_tail_rope(&new_head_position, head_position, tail_position);

    (new_head_position, new_tail_position)
}

fn move_head_rope(head_position: &Position, direction: &Direction) -> Position {
    let head_x = head_position.x;
    let head_y = head_position.y;

    match direction {
        Direction::Up => Position {
            x: head_x,
            y: head_y + 1,
        },
        Direction::Down => Position {
            x: head_x,
            y: head_y - 1,
        },
        Direction::Left => Position {
            x: head_x - 1,
            y: head_y,
        },
        Direction::Right => Position {
            x: head_x + 1,
            y: head_y,
        },
    }
}

fn move_tail_rope(
    new_head_position: &Position,
    head_position: Position,
    tail_position: Position,
) -> Position {
    let distance = get_tchebychev_distance(new_head_position, &tail_position);

    if distance <= 1 {
        tail_position
    } else {
        head_position
    }
}

fn get_tchebychev_distance(position_a: &Position, position_b: &Position) -> u64 {
    let distance_x = position_b.x.abs_diff(position_a.x);
    let distance_y = position_b.y.abs_diff(position_a.y);

    if distance_x > distance_y {
        distance_x
    } else {
        distance_y
    }
}
