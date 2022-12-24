use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    line_number: usize,
    column_number: usize,
}

#[derive(Debug)]
struct Blizzard {
    position: Position,
    direction: Direction,
}

struct InitialInput {
    blizzards: Vec<Blizzard>,
    line_number_max: usize,
    column_number_max: usize,
}

fn main() -> std::io::Result<()> {
    let file_path = "real.txt";

    let initial_input = parse_initial_blizzard(file_path);

    let mut blizzards = initial_input.blizzards;
    let line_number_max = initial_input.line_number_max;
    let column_number_max = initial_input.column_number_max;

    println!("{line_number_max} / {column_number_max}");

    let mut count: usize = 0;
    let mut positions = vec![Position {
        line_number: 0,
        column_number: 1,
    }];

    loop {
        blizzards = get_next_blizzards(&blizzards, line_number_max, column_number_max);

        positions = get_all_neighbors(&positions, line_number_max, column_number_max)
            .into_iter()
            .filter(|position| !is_position_taken(&blizzards, position))
            .collect();

        count += 1;
        println!("{count}\t{}", positions.len());

        if positions.iter().any(|position| {
            position.line_number == line_number_max
                && position.column_number == column_number_max - 1
        }) {
            break;
        }
    }

    println!("{count}");

    Ok(())
}

fn parse_initial_blizzard(filename: &str) -> InitialInput {
    let file = File::open(filename).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut blizzards: Vec<Blizzard> = Vec::new();
    let mut line_number_max = 0;
    let mut column_number_max = 0;

    for (line_number, line) in buf_reader.lines().enumerate() {
        let line_value = line.unwrap();
        line_number_max = line_number;

        for (column_number, char_value) in line_value.chars().enumerate() {
            column_number_max = column_number;
            let direction: Option<Direction> = match char_value {
                '>' => Some(Direction::RIGHT),
                '<' => Some(Direction::LEFT),
                '^' => Some(Direction::UP),
                'v' => Some(Direction::DOWN),
                _ => None,
            };

            if direction.is_none() {
                continue;
            }

            let blizzard = Blizzard {
                position: Position {
                    line_number,
                    column_number,
                },
                direction: direction.unwrap(),
            };

            blizzards.push(blizzard);
        }
    }

    InitialInput {
        blizzards,
        line_number_max,
        column_number_max,
    }
}

fn get_next_blizzards(
    blizzards: &Vec<Blizzard>,
    line_number_max: usize,
    column_number_max: usize,
) -> Vec<Blizzard> {
    blizzards
        .iter()
        .map(|blizzard| {
            let line_number = blizzard.position.line_number;
            let column_number = blizzard.position.column_number;

            let position = match blizzard.direction {
                Direction::UP => Position {
                    line_number: 1
                        + (line_number_max - 1 + line_number - 1 - 1) % (line_number_max - 1),
                    column_number,
                },
                Direction::DOWN => Position {
                    line_number: 1
                        + (line_number_max - 1 + line_number - 1 + 1) % (line_number_max - 1),
                    column_number,
                },
                Direction::LEFT => Position {
                    line_number,
                    column_number: 1
                        + (column_number_max - 1 + column_number - 1 - 1) % (column_number_max - 1),
                },
                Direction::RIGHT => Position {
                    line_number,
                    column_number: 1
                        + (column_number_max - 1 + column_number - 1 + 1) % (column_number_max - 1),
                },
            };

            Blizzard {
                position,
                direction: blizzard.direction,
            }
        })
        .collect()
}

fn is_position_taken(blizzards: &Vec<Blizzard>, position: &Position) -> bool {
    blizzards.iter().any(|blizzard| {
        blizzard.position.line_number == position.line_number
            && blizzard.position.column_number == position.column_number
    })
}

fn get_neighbors(
    position: Position,
    line_number_max: usize,
    column_number_max: usize,
) -> Vec<Position> {
    let line_number = position.line_number;
    let column_number = position.column_number;

    if line_number == 0 && column_number == 1 {
        return vec![
            Position {
                line_number: 0,
                column_number: 1,
            },
            Position {
                line_number: 1,
                column_number: 1,
            },
        ];
    }

    vec![
        Position {
            line_number,
            column_number,
        },
        Position {
            line_number: line_number + 1,
            column_number,
        },
        Position {
            line_number,
            column_number: column_number + 1,
        },
        Position {
            line_number: line_number - 1,
            column_number,
        },
        Position {
            line_number,
            column_number: column_number - 1,
        },
    ]
    .into_iter()
    .filter(|position| {
        // Start + End
        if (position.line_number == 0 && position.column_number == 1)
            || (position.line_number == line_number_max
                && position.column_number == column_number_max - 1)
        {
            return true;
        }

        1 <= position.line_number
            && position.line_number < line_number_max
            && 1 <= position.column_number
            && position.column_number < column_number_max
    })
    .collect()
}

fn get_all_neighbors(
    positions: &Vec<Position>,
    line_number_max: usize,
    column_number_max: usize,
) -> Vec<Position> {
    positions
        .into_iter()
        .flat_map(|&position| get_neighbors(position, line_number_max, column_number_max))
        .unique()
        .collect()
}
