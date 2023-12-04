use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    solve1(&input);
    solve2(&input);
}

fn solve1(input: &str) {
    let cards = parse_cards(input);
    let total_worth: usize = cards.into_values().map(|card| card.worth()).sum();
    println!("{total_worth}");
}

fn solve2(input: &str) {
    let mut cards = parse_cards(input);
    process_winnings(&mut cards);
    let card_count: u32 = cards.into_values().map(|card| card.count).sum();
    println!("{card_count}");
}

fn process_winnings(cards: &mut HashMap<u32, Card>) {
    for i in 1..=(cards.len() as u32) {
        let to_copy = cards_to_copy(cards.get(&i).unwrap());
        let copy_count = cards.get(&i).unwrap().count;
        to_copy.into_iter().for_each(|id| {
            if let Some(card) = cards.get_mut(&id) {
                card.count += copy_count;
            }
        });
    }
}

fn cards_to_copy(card: &Card) -> Vec<u32> {
    let mut cards = Vec::new();
    for i in 1..=card.matching_number_count {
        cards.push(card.id + i as u32)
    }
    cards
}

#[derive(Debug)]
struct Card {
    id: u32,
    count: u32,
    _winning_numbers: HashSet<u32>,
    _numbers: HashSet<u32>,
    matching_number_count: usize,
}

fn parse_cards(input: &str) -> HashMap<u32, Card> {
    let mut cards = HashMap::new();
    input.lines().map(parse_card).for_each(|card| {
        cards.insert(card.id, card);
    });
    cards
}

fn parse_card(line: &str) -> Card {
    let parts: Vec<_> = line.split(':').collect();
    let header = parts[0];
    let id: u32 = header.split(' ').last().unwrap().parse().unwrap();
    let number_parts: Vec<_> = parts[1].split('|').collect();
    let winning_numbers = parse_numbers(number_parts[0]);
    let numbers = parse_numbers(number_parts[1]);
    let matching_number_count = count_matching_numbers(&numbers, &winning_numbers);

    Card {
        id,
        count: 1,
        _winning_numbers: winning_numbers,
        _numbers: numbers,
        matching_number_count,
    }
}

fn parse_numbers(line: &str) -> HashSet<u32> {
    line.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn count_matching_numbers(numbers: &HashSet<u32>, winning_numbers: &HashSet<u32>) -> usize {
    numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count()
}

impl Card {
    fn worth(&self) -> usize {
        if self.matching_number_count == 0 {
            0
        } else {
            2_u64
                .pow((self.matching_number_count - 1).try_into().unwrap())
                .try_into()
                .unwrap()
        }
    }
}
