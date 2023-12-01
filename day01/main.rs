use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    solve1(&input);
    solve2(&input);
}

fn solve1(input: &str) {
    println!("{}", sum_of_calibration_values(input, &str_to_digits));
}

fn str_to_digits(line: &str) -> Vec<u32> {
    line.chars().into_iter().filter_map(|c| c.to_digit(10)).collect()
}

fn sum_of_calibration_values(input: &str, to_digits: &dyn Fn(&str) -> Vec<u32>) -> u32 {
    input.lines().map(|line| calibration_value(line, to_digits)).sum()
}

fn calibration_value(line: &str, to_digits: &dyn Fn(&str) -> Vec<u32>) -> u32 {
    let digits: Vec<_> = to_digits(line);
    let first = digits.first().unwrap();
    let last = digits.last().unwrap();
    first * 10 + last
}

fn solve2(input: &str) {
    println!("{}", sum_of_calibration_values(input, &digits_on_line));
}

fn digits_on_line(line: &str) -> Vec<u32> {
    if line.is_empty() {
        return Vec::new();
    }

    for digit_letter in DIGIT_LETTERS {
        if line.starts_with(digit_letter.0) {
            let digit = digit_letter.1;
            let mut v = digits_on_line(&line[1..]);
            v.insert(0, digit);
            return v;
        }
    }
    match line.chars().next().unwrap().to_digit(10) {
        None => digits_on_line(&line[1..]),
        Some(digit) => {
            let mut v = digits_on_line(&line[1..]);
            v.insert(0, digit);
            return v;
        }
    }
}

const DIGIT_LETTERS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];
