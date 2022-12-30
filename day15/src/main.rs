use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    pub fn intersect(&self, other: &Self) -> bool {
        let max_start = std::cmp::max(self.start, other.start);
        let min_end = std::cmp::min(self.end, other.end);

        max_start <= min_end
    }

    pub fn exclusive_union(&self, other: &Self) -> Vec<Interval> {
        if !self.intersect(other) {
            return Vec::from([other.clone()]);
        }

        let mut exclusive_union_vec: Vec<Interval> = Vec::new();
        if other.end >= self.start && self.start > other.start {
            exclusive_union_vec.push(Interval {
                start: other.start,
                end: self.start - 1,
            })
        }
        if other.start <= self.end && self.end < other.end {
            exclusive_union_vec.push(Interval {
                start: self.end + 1,
                end: other.end,
            })
        }

        exclusive_union_vec
    }

    pub fn len(&self) -> i32 {
        self.end - self.start + 1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");
    let depth_search: i32 = args.get(2).expect("No deep given.").parse::<i32>().unwrap();

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut line_iterator = buf_reader.lines().into_iter();

    let parser_regex = Regex::new(
        r"^Sensor at x=(\-?\d+), y=(\-?\d+): closest beacon is at x=(\-?\d+), y=(\-?\d+)$",
    )
    .unwrap();

    let mut impossible_beacon_intervals: Vec<Interval> = Vec::new();
    let mut beacon_at_depth_search: HashSet<i32> = HashSet::new();

    while let Some(Ok(line)) = line_iterator.next() {
        let regex_captures = parser_regex
            .captures(&line)
            .expect("Issue capturing input.");

        let sensor_position = Position {
            x: regex_captures[1].parse::<i32>().unwrap(),
            y: regex_captures[2].parse::<i32>().unwrap(),
        };
        let beacon_position = Position {
            x: regex_captures[3].parse::<i32>().unwrap(),
            y: regex_captures[4].parse::<i32>().unwrap(),
        };

        if beacon_position.y == depth_search {
            beacon_at_depth_search.insert(beacon_position.x);
        }

        let distance_sensor_beacon = get_manhattan_distance(&sensor_position, &beacon_position);
        let distance_sensor_depth = depth_search.abs_diff(sensor_position.y);

        if distance_sensor_depth > distance_sensor_beacon {
            continue;
        }

        let diff_distances = (distance_sensor_beacon - distance_sensor_depth) as i32;

        let mut intervals_to_insert = Vec::from([Interval {
            start: sensor_position.x - diff_distances,
            end: sensor_position.x + diff_distances,
        }]);

        for impossible_beacon_interval in impossible_beacon_intervals.iter() {
            let mut new_intervals_to_insert: Vec<Interval> = Vec::new();
            for interval_to_insert in intervals_to_insert.into_iter() {
                let exclusive_union =
                    impossible_beacon_interval.exclusive_union(&interval_to_insert);

                new_intervals_to_insert.extend(exclusive_union);
            }
            intervals_to_insert = new_intervals_to_insert;
        }

        impossible_beacon_intervals.extend(intervals_to_insert);
    }

    let impossible_beacon_count = impossible_beacon_intervals
        .iter()
        .map(|interval| interval.len())
        .sum::<i32>() as usize
        - beacon_at_depth_search.len();

    println!("{impossible_beacon_count}")
}

fn get_manhattan_distance(position_a: &Position, position_b: &Position) -> u32 {
    position_b.x.abs_diff(position_a.x) + position_b.y.abs_diff(position_a.y)
}
