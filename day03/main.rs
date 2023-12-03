use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    solve1(&input);
    solve2(&input);
}

fn solve1(input: &str) {
    let grid = parse_grid(input);
    let (_symbols, symbol_neighbor_cells) = scan_symbols(&grid);
    let all_numbers = find_all_numbers(input);
    let numbers_without_parts = find_numbers_without_parts(grid, symbol_neighbor_cells);
    let sum_all_numbers: u32 = all_numbers.into_iter().sum();
    let sum_numbers_without_parts: u32 = numbers_without_parts.into_iter().sum();
    println!("{}", sum_all_numbers - sum_numbers_without_parts);
}

fn solve2(input: &str) {
    let grid = preprocess(input);
    let gears = find_gears(&grid);
    let sum_gear_ratios: u32 = gears.into_iter().map(gear_ratio).sum();
    println!("{}", sum_gear_ratios);

}

fn gear_ratio(gear: (u32, u32)) -> u32 {
    gear.0 * gear.1
}

fn preprocess(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().map(|c| {
        if !c.is_digit(10) && c != '.' && c != '*' {
            '$'
        } else {
            c
        }
    }).collect()).collect()
}

fn find_gears(grid: &Vec<Vec<char>>) -> Vec<(u32, u32)> {
    let mut gears: Vec<_> = Vec::new();


    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let c = grid[row][col];
            if c == '*' {
                let touching_cells = find_all_touching_cells(row, col, c);
                let mut number_cells: Vec<(usize, usize)> = Vec::new();

                for (r1, c1) in touching_cells.keys() {
                    if grid[*r1][*c1].is_digit(10) {
                        number_cells.push((*r1, *c1));
                    }
                }

                let mut numbers: Vec<_> = number_cells.into_iter().map(|cell| number_cells_from_cell(grid, cell)).collect();
                numbers.sort();
                numbers.dedup();

                if numbers.len() == 2 {
                    gears.push(numbers);
                }
            }
        }
    }

    let gears: Vec<_> = gears.into_iter().map(|g| coords_to_gear(grid, g)).collect();
    gears.into_iter().map(|g| (g[0], g[1])).collect()
}

fn coords_to_gear(grid: &Vec<Vec<char>>, coords: Vec<Vec<(usize, usize)>>) -> Vec<u32> {
    coords.into_iter().map(|c| coords_to_numbers(grid, c)).collect()
}

fn coords_to_numbers(grid: &Vec<Vec<char>>, coords: Vec<(usize, usize)>) -> u32 {
    let mut coords = coords.clone();
    coords.sort();
    let mut number = 0;
    for (row, col) in coords {
        let d: u32 = grid[row][col].to_digit(10).unwrap();
        number = number * 10 + d;
    }
    number
}

fn number_cells_from_cell(grid: &Vec<Vec<char>>, cell: (usize, usize)) -> Vec<(usize, usize)> {
    let mut number_cells: Vec<(usize, usize)> = Vec::new();
    let row = &grid[cell.0];

    let mut i = cell.1;
    while i > 0 && row[i - 1].is_digit(10) {
        i -= 1;
    }
    while i < row.len() {
        if !row[i].is_digit(10) {
            break;
        }
        number_cells.push((cell.0, i));
        i += 1;
    }

    number_cells
}


fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_all_numbers(input: &str) -> Vec<u32> {
    input.lines().into_iter().map(find_line_numbers).flatten().collect()
}

fn find_line_numbers(line: &str) -> Vec<u32> {
    let line: String = line.chars().into_iter().map(|c| {
        if c.is_digit(10) {
            c
        } else {
            ' '
        }
    }).collect();
    line.split(' ').filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect()
}

fn scan_symbols(grid: &Vec<Vec<char>>) -> (HashMap<(usize, usize), char>, HashMap<(usize, usize), char>) {
    let mut symbols = HashMap::new();
    let mut touching_cells = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let c = grid[row][col];
            if c == '.' || c.is_digit(10) {
                continue;
            }
            symbols.insert((row, col), c);
            let cells = find_all_touching_cells(row, col, c);
            for (coord, symbol) in cells.into_iter() {
                touching_cells.insert(coord, symbol);
            }
        }
    }
    (symbols, touching_cells)
}

fn find_all_touching_cells(row: usize, col: usize, c: char) -> HashMap<(usize, usize), char> {
    let mut touching_cells = HashMap::new();
    for i in -1..=1 {
        if row as i32 + i < 0 {
            continue;
        }
        for j in -1..=1 {
            if col as i32 + j < 0 {
                continue;
            }

            touching_cells.insert(((row as i32 + i) as usize, (col as i32 + j) as usize), c);
        }
    }
    touching_cells
}

fn find_numbers_without_parts(grid: Vec<Vec<char>>, symbol_neighbor_cells: HashMap<(usize, usize), char>) -> Vec<u32> {
    let mut numbers = Vec::new();
    let mut current_number = None;
    let mut has_part: bool = false;

    for row in 0..grid.len() {
        has_part = false;
        current_number = None;
        for col in 0..grid[row].len() {
            let c = grid[row][col];
            if c == '.' || !c.is_digit(10) {
                if current_number.is_some() && !has_part {
                    numbers.push(current_number.unwrap());
                }
                current_number = None;
                has_part = false;
            }
            if symbol_neighbor_cells.get(&(row, col)).is_some() {
                has_part = true;
            } else {
                if current_number.is_none() {
                    has_part = false;
                }
            }
            if let Some(n) = c.to_digit(10) {
                match current_number {
                    None => {
                        current_number = Some(n)
                    },
                    Some(number) => {
                        current_number = Some(number * 10 + n)
                    },
                }
            }
        }
        if let Some(_number) = current_number {
            if !has_part  {
                numbers.push(current_number.unwrap());
            }
            current_number = None;
        }
        has_part = false;
    }
    numbers
}
