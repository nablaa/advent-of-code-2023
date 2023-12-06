use rayon::prelude::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    solve1(&input);
    solve2(&input);
}

fn solve1(input: &str) {
    let almanac = parse_almanac_with_seeds(input);
    let lowest_location = almanac.lowest_location();
    println!("{}", lowest_location);
}

fn solve2(input: &str) {
    let almanac = parse_almanac_with_seed_range(input);
    let lowest_location = almanac.lowest_location();
    println!("{}", lowest_location);
}

#[derive(Debug, Clone)]
struct MapRange {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

#[derive(Debug, Clone)]
struct Map {
    name: String,
    ranges: Vec<MapRange>,
}

#[derive(Debug)]
struct Almanac {
    seed_ranges: Vec<(u64, u64)>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

fn parse_almanac_with_seed_range(input: &str) -> Almanac {
    parse_almanac(input, &parse_seeds_range)
}

fn parse_almanac_with_seeds(input: &str) -> Almanac {
    parse_almanac(input, &parse_seeds)
}

fn parse_almanac(input: &str, parse_seed_func: &dyn Fn(&str) -> Vec<(u64, u64)>) -> Almanac {
    let parts: Vec<_> = input.split("\n\n").collect();
    let seed_ranges = parse_seed_func(parts[0]);
    let maps: Vec<_> = parts[1..].iter().map(|p| parse_map(p)).collect();

    let seed_to_soil = maps
        .clone()
        .into_iter()
        .find(|m| m.name == "seed-to-soil")
        .unwrap();
    let soil_to_fertilizer = maps
        .clone()
        .into_iter()
        .find(|m| m.name == "soil-to-fertilizer")
        .unwrap();
    let fertilizer_to_water = maps
        .clone()
        .into_iter()
        .find(|m| m.name == "fertilizer-to-water")
        .unwrap();
    let water_to_light = maps
        .clone()
        .into_iter()
        .find(|m| m.name == "water-to-light")
        .unwrap();
    let light_to_temperature = maps
        .clone()
        .into_iter()
        .find(|m| m.name == "light-to-temperature")
        .unwrap();
    let temperature_to_humidity = maps
        .clone()
        .into_iter()
        .find(|m| m.name == "temperature-to-humidity")
        .unwrap();
    let humidity_to_location = maps
        .clone()
        .into_iter()
        .find(|m| m.name == "humidity-to-location")
        .unwrap();

    Almanac {
        seed_ranges,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn parse_seeds(line: &str) -> Vec<(u64, u64)> {
    let parts: Vec<_> = line.split(':').collect();
    assert_eq!(parts[0], "seeds");
    parts[1]
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .map(|s| (s, s))
        .collect()
}

fn parse_seeds_range(line: &str) -> Vec<(u64, u64)> {
    let parts: Vec<_> = line.split(':').collect();
    assert_eq!(parts[0], "seeds");
    let mut numbers = parts[1].trim().split(' ').map(|s| s.parse().unwrap());

    let mut seed_ranges: Vec<(u64, u64)> = Vec::new();
    while let Some(start) = numbers.next() {
        let length = numbers.next().unwrap();

        seed_ranges.push((start, start + length - 1));
    }
    seed_ranges
}

fn parse_map(input: &str) -> Map {
    let lines: Vec<_> = input.lines().collect();
    let name = lines[0].split(' ').collect::<Vec<_>>()[0].to_string();
    let ranges = lines[1..]
        .iter()
        .map(|line| match line.split(' ').collect::<Vec<_>>()[..] {
            [destination_range_start, source_range_start, range_length] => MapRange {
                destination_range_start: destination_range_start.parse().unwrap(),
                source_range_start: source_range_start.parse().unwrap(),
                range_length: range_length.parse().unwrap(),
            },
            _ => panic!(),
        })
        .collect();

    Map { name, ranges }
}

impl MapRange {
    fn is_within(&self, source: u64) -> bool {
        source >= self.source_range_start && source < self.source_range_start + self.range_length
    }

    fn map(&self, source: u64) -> u64 {
        let diff = source - self.source_range_start;
        self.destination_range_start + diff
    }
}

impl Map {
    fn source_to_destination(&self, source: u64) -> u64 {
        match self.matching_range(source) {
            None => source,
            Some(range) => range.map(source),
        }
    }

    fn matching_range(&self, source: u64) -> Option<&MapRange> {
        self.ranges.iter().find(|range| range.is_within(source))
    }
}

impl Almanac {
    fn lowest_location(&self) -> u64 {
        let mut lowest: u64 = std::u64::MAX;
        for (start, end) in self.seed_ranges.clone().into_iter() {
            let location: u64 = (start..=end)
                .into_par_iter()
                .map(|seed| self.seed_to_soil.source_to_destination(seed))
                .map(|soil| self.soil_to_fertilizer.source_to_destination(soil))
                .map(|fertilizer| self.fertilizer_to_water.source_to_destination(fertilizer))
                .map(|water| self.water_to_light.source_to_destination(water))
                .map(|light| self.light_to_temperature.source_to_destination(light))
                .map(|temperature| {
                    self.temperature_to_humidity
                        .source_to_destination(temperature)
                })
                .map(|humidity| self.humidity_to_location.source_to_destination(humidity))
                .min()
                .unwrap();
            if location < lowest {
                lowest = location;
            }
        }
        lowest
    }
}
