use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    solve1(&input);
    solve2(&input);
}

fn solve1(input: &str) {
    let games = parse_games(input);
    let sum: u32 = games.into_iter().filter(allowed_game).map(|game| game.id).sum();
    println!("{}", sum);
}

fn solve2(input: &str) {
    let games = parse_games(input);
    let sum: u32 = games.iter().map(power).sum();
    println!("{}", sum);
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

fn power(game: &Game) -> u32 {
    let (r, g, b) = fewest_balls(game);
    r * g * b
}

fn fewest_balls(game: &Game) -> (u32, u32, u32) {
    let red = game.sets.iter().map(|s| s.red).max().unwrap();
    let green = game.sets.iter().map(|s| s.green).max().unwrap();
    let blue = game.sets.iter().map(|s| s.blue).max().unwrap();
    (red, green, blue)
}

fn allowed_game(game: &Game) -> bool {
    game.sets.iter().all(allowed_set)
}

fn allowed_set(set: &Set) -> bool {
    set.red <= 12 && set.green <= 13 && set.blue <= 14
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(parse_game).collect()
}

fn parse_game(line: &str) -> Game {
    let parts: Vec<_> = line.split(":").collect();
    let g = parts[0].split(" ");
    let id: u32 = g.last().unwrap().parse().unwrap();
    let sets = parse_sets(parts[1]);

    Game { id, sets }
}

fn parse_sets(line: &str) -> Vec<Set> {
    line.split(";").map(parse_set).collect()
}

fn parse_set(line: &str) -> Set {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;

    line.split(",").for_each(|s| {
        let s = s.trim();
        let parts: Vec<_> = s.split(" ").collect();
        let count: u32 = parts.first().unwrap().parse().unwrap();
        let color = parts.last().unwrap().trim();
        if color == "red" {
            red = count;
        } else if color == "green" {
            green = count;
        } else {
            blue = count;
        }
    });
    Set { red, green, blue }
}
