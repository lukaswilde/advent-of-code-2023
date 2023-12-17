use std::str::FromStr;

use anyhow::Result;

use utils::parse_text;

struct Game {
    id: u32,
    red: Vec<u32>,
    green: Vec<u32>,
    blue: Vec<u32>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (id, draws) = s
            .split_once(':')
            .expect("There should be a colon in a line");

        let id = id
            .split_whitespace()
            .last()
            .expect("There should be a whitespace between game and id")
            .parse::<u32>()
            .expect("Should be convertible to u32");

        let mut red = Vec::new();
        let mut green = Vec::new();
        let mut blue = Vec::new();

        draws.split(';').for_each(|seq| {
            seq.split(',').for_each(|substr| {
                let (num, color) = substr
                    .trim()
                    .split_once(' ')
                    .expect("Shoule be number _ color");
                let num = num
                    .parse::<u32>()
                    .expect("Number should be convertible to u32");
                match color {
                    "red" => red.push(num),
                    "green" => green.push(num),
                    "blue" => blue.push(num),
                    _ => panic!("Color should be either red, green or blue"),
                }
            })
        });
        Ok(Game {
            id,
            red,
            green,
            blue,
        })
    }
}

fn main() {
    let text = parse_text();
    let mut games = parse_games(&text);
    let power = get_power(&games);
    filter(&mut games, 12, 13, 14);
    let id_sum = sum_ids(&games);
    println!("The sum of IDs of valid games is {}", id_sum);
    println!("The summed power of the games is {}", power);
}

fn get_power(games: &[Game]) -> u32 {
    get_powers(games).into_iter().sum()
}

fn get_powers(games: &[Game]) -> Vec<u32> {
    games
        .iter()
        .map(|g| {
            g.red.iter().max().expect("Should have a max value")
                * g.green.iter().max().expect("Should have a max value")
                * g.blue.iter().max().expect("Should have a max value")
        })
        .collect()
}

fn sum_ids(games: &[Game]) -> u32 {
    games.iter().map(|g| g.id).sum()
}

fn filter(games: &mut Vec<Game>, num_red: u32, num_green: u32, num_blue: u32) {
    games.retain(|g| {
        g.red.iter().all(|&num| num <= num_red)
            && g.green.iter().all(|&num| num <= num_green)
            && g.blue.iter().all(|&num| num <= num_blue)
    });
}

fn parse_games(text: &str) -> Vec<Game> {
    text.split('\n')
        .map(|x| x.parse::<Game>().unwrap())
        .collect()
}
