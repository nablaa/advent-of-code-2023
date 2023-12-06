use std::convert::TryInto;
use std::{fs, iter::zip};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    solve1(&input);
    solve2(&input);
}

fn solve1(input: &str) {
    let races = parse_races(input);
    let winning_counts: Vec<_> = races.into_iter().map(winning_count).collect();
    let multiplied: u64 = winning_counts.iter().product();
    println!("{multiplied}");
}

fn solve2(input: &str) {
    let race = parse_single_race(input);
    let count = winning_count(race);
    println!("{count}");
}

fn parse_single_race(input: &str) -> (u64, u64) {
    let lines: Vec<_> = input.lines().collect();
    let time = parse_single_number(lines[0]);
    let distance = parse_single_number(lines[1]);
    (time, distance)
}

fn parse_races(input: &str) -> Vec<(u64, u64)> {
    let lines: Vec<_> = input.lines().collect();
    let times = parse_numbers(lines[0]);
    let distances = parse_numbers(lines[1]);
    zip(times, distances).collect()
}

fn parse_single_number(line: &str) -> u64 {
    let string = line.split(':').last().unwrap();
    string.replace(' ', "").parse().unwrap()
}

fn parse_numbers(line: &str) -> Vec<u64> {
    line.split(':')
        .last()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn distance_travelled(race_length: u64, hold_time: u64) -> u64 {
    assert!(hold_time <= race_length);
    let speed = hold_time;
    let race_time = race_length - hold_time;
    speed * race_time
}

fn possible_distances(race_length: u64) -> Vec<u64> {
    let mut distances = Vec::new();
    for i in 1..race_length {
        distances.push(distance_travelled(race_length, i));
    }
    distances
}

fn winning_count(race: (u64, u64)) -> u64 {
    let (race_length, record_time) = race;
    possible_distances(race_length)
        .into_iter()
        .filter(|d| d > &record_time)
        .count()
        .try_into()
        .unwrap()
}
