use std::cmp::min;
use std::collections::HashSet;
use std::str::FromStr;

use anyhow::Result;

use utils::parse_text;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point(usize, usize);

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Symbol {
    symbol: char,
    pos: Point,
}

#[derive(Default, Debug)]
struct PartNumber {
    number: u32,
    occupied_positions: HashSet<Point>,
}

struct Engine {
    width: usize,
    height: usize,
    symbol_positions: HashSet<Symbol>,
    parts: Vec<PartNumber>,
}

impl FromStr for Engine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Engine> {
        let mut parts = Vec::new();
        let mut symbols = HashSet::new();
        let width = s.split('\n').nth(0).unwrap().len();
        let height = s.split('\n').enumerate().last().unwrap().0 + 1;

        s.split('\n')
            .enumerate()
            .for_each(|(idx, line)| process_line(line, idx, &mut parts, &mut symbols));

        Ok(Engine {
            symbol_positions: symbols,
            parts,
            width,
            height,
        })
    }
}

fn process_line(
    line: &str,
    row: usize,
    parts: &mut Vec<PartNumber>,
    symbols: &mut HashSet<Symbol>,
) {
    let mut col = 0;
    while col < line.len() {
        let c = line
            .chars()
            .nth(col)
            .expect("Should be possible since col < len");

        if !c.is_ascii_digit() && c != '.' {
            symbols.insert(Symbol {
                symbol: c,
                pos: Point(col, row),
            });
            col += 1;
        } else if c.is_ascii_digit() {
            let number_str = &line[col..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();
            let number = number_str
                .parse::<u32>()
                .expect("Should be convertible to number");

            let mut occupied_positions = HashSet::new();
            for i in col..(col + number_str.len()) {
                occupied_positions.insert(Point(i, row));
            }
            parts.push(PartNumber {
                number,
                occupied_positions,
            });
            col += number_str.len();
        } else {
            col += 1;
        }
    }
}

fn get_neighbors(pos: &Point, width: usize, height: usize) -> HashSet<Point> {
    let mut points = HashSet::new();
    // center down
    points.insert(Point(pos.0, min(pos.1 + 1, height - 1)));
    // center up
    points.insert(Point(pos.0, pos.1.saturating_sub(1)));
    // center left
    points.insert(Point(pos.0.saturating_sub(1), pos.1));
    // center right
    points.insert(Point(min(pos.0 + 1, width - 1), pos.1));

    // upper right
    points.insert(Point(min(pos.0 + 1, width - 1), pos.1.saturating_sub(1)));
    // lower right
    points.insert(Point(min(pos.0 + 1, width - 1), min(pos.1 + 1, height - 1)));
    // upper left
    points.insert(Point(pos.0.saturating_sub(1), pos.1.saturating_sub(1)));
    // lower left
    points.insert(Point(pos.0.saturating_sub(1), min(pos.1 + 1, height - 1)));

    points
}

fn get_neighboring_cells(part: &PartNumber, width: usize, height: usize) -> HashSet<Point> {
    let mut points = HashSet::new();
    part.occupied_positions
        .iter()
        .map(|pos| get_neighbors(pos, width, height))
        .for_each(|set| points.extend(set));
    points.retain(|p| !part.occupied_positions.contains(p));
    points
}

fn get_valid_parts_sum(engine: &Engine) -> u32 {
    engine
        .parts
        .iter()
        .filter_map(|part| {
            let neighbors = get_neighboring_cells(part, engine.width, engine.height);
            engine
                .symbol_positions
                .iter()
                .cloned()
                .map(|s| s.pos)
                .collect::<HashSet<_>>()
                .intersection(&neighbors)
                .next()
                .is_some()
                .then_some(part.number)
        })
        .sum()
}

fn get_gear_ratio_sum(engine: &Engine) -> u32 {
    let valid_symbol_pos = engine
        .symbol_positions
        .iter()
        .cloned()
        .filter_map(|s| (s.symbol == '*').then_some(s.pos))
        .collect::<HashSet<_>>();

    valid_symbol_pos
        .iter()
        .filter_map(|pos| {
            let (num_neighbors, product) = engine
                .parts
                .iter()
                // .map(|part| (part.occupied_positions.contains(pos) as usize, part))
                .filter(|part| {
                    get_neighboring_cells(part, engine.width, engine.height).contains(pos)
                })
                .fold((0, 1), |acc, el| (acc.0 + 1, acc.1 * el.number));

            num_neighbors.eq(&2).then_some(product)
        })
        .sum()
}

fn main() {
    let text = parse_text();
    let engine = text
        .parse::<Engine>()
        .expect("Problem text should be convertible to engine");
    let parts_sum = get_valid_parts_sum(&engine);
    let gear_ratio_sum = get_gear_ratio_sum(&engine);
    println!("The sum of valid plans in the schematic is {}", parts_sum);
    println!(
        "The sum of valid gear ratios in the schematic is {}",
        gear_ratio_sum
    );
}
