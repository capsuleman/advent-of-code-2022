use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use tqdm::tqdm;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    neighbor_valves: HashSet<String>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Possibility {
    opened_valves: Vec<String>,
    current_valve: String,
}

lazy_static! {
    static ref PARSER_REGEX: Regex =
        Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
            .unwrap();
}

const MAX_STEP: u32 = 30;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");
    let valves = parse_file(file_path);

    let mut possibilities: HashMap<Possibility, u32> = HashMap::from([(
        Possibility {
            opened_valves: Vec::new(),
            current_valve: String::from("AA"),
        },
        0,
    )]);

    for step_index in 0..MAX_STEP {
        println!("{step_index}\t{:} possibilities", possibilities.len());

        let mut next_possibilities: HashMap<Possibility, u32> = HashMap::new();
        for (possibility, total_flow) in tqdm(possibilities.into_iter()) {
            for (next_possibility, new_total_flow) in
                get_next_possibilities(&valves, possibility, total_flow).into_iter()
            {
                let max_total_flow = *next_possibilities.get(&next_possibility).unwrap_or(&0);
                if new_total_flow >= max_total_flow {
                    next_possibilities.insert(next_possibility, new_total_flow);
                }
            }
        }
        possibilities = next_possibilities;
    }

    let max_total_flow = *possibilities.values().max().unwrap();
    println!("Result: {max_total_flow}");
}

fn parse_file(file_path: &String) -> HashMap<String, Valve> {
    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut line_iterator = buf_reader.lines().into_iter();

    let mut valves: HashMap<String, Valve> = HashMap::new();

    while let Some(Ok(line)) = line_iterator.next() {
        let captures = PARSER_REGEX
            .captures(&line)
            .expect("Error parsing input line.");

        let valve_name = String::from(&captures[1]);
        let flow_rate = captures[2].parse::<u32>().unwrap();
        let mut neighbor_valves: HashSet<String> = HashSet::new();
        captures[3].split(", ").for_each(|neighbor_valve| {
            neighbor_valves.insert(String::from(neighbor_valve));
        });

        valves.insert(
            valve_name,
            Valve {
                flow_rate,
                neighbor_valves,
            },
        );
    }

    valves
}

fn get_next_possibilities(
    valves: &HashMap<String, Valve>,
    possibility: Possibility,
    total_flow: u32,
) -> HashMap<Possibility, u32> {
    let mut next_possibilities: HashMap<Possibility, u32> = HashMap::new();

    let current_flow: u32 = possibility
        .opened_valves
        .iter()
        .map(|open_valve| valves[open_valve].flow_rate)
        .sum();

    let is_current_valve_open = possibility
        .opened_valves
        .contains(&possibility.current_valve);

    let current_valve_flow = valves[&possibility.current_valve].flow_rate;

    if !is_current_valve_open && current_valve_flow > 0 {
        let mut opened_valves = possibility.opened_valves.clone();
        opened_valves.push(possibility.current_valve.clone());
        opened_valves.sort();
        next_possibilities.insert(
            Possibility {
                opened_valves,
                current_valve: possibility.current_valve.clone(),
            },
            total_flow + current_flow,
        );
    };

    valves
        .get(&possibility.current_valve)
        .unwrap()
        .neighbor_valves
        .iter()
        .for_each(|neighbor_valve| {
            next_possibilities.insert(
                Possibility {
                    current_valve: neighbor_valve.clone(),
                    opened_valves: possibility.opened_valves.clone(),
                },
                total_flow + current_flow,
            );
        });

    next_possibilities
}
