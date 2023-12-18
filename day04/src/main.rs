use std::str::FromStr;

use anyhow::Result;

use utils::parse_text;

#[derive(Debug)]
struct ScratchCard {
    winners: Vec<u32>,
    chosen: Vec<u32>,
}

impl FromStr for ScratchCard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let numbers: String = s.chars().skip_while(|&c| c != ':').skip(1).collect();
        let numbers = numbers.trim();
        let (winners, chosen) = numbers
            .split_once('|')
            .expect("There should be a | to split");

        let winners: Vec<u32> = winners
            .split_whitespace()
            .map(|chunk| chunk.parse::<u32>().expect("Should be convertible to u32"))
            .collect();
        let chosen: Vec<u32> = chosen
            .split_whitespace()
            .map(|chunk| chunk.parse::<u32>().expect("Should be convertible to u32"))
            .collect();

        Ok(ScratchCard { winners, chosen })
    }
}

fn parse_scratchcards(text: &str) -> Vec<ScratchCard> {
    text.split('\n')
        .map(|line| {
            line.parse::<ScratchCard>()
                .expect("Line should be convertible to ScratchCard")
        })
        .collect()
}

fn get_card_value(card: &ScratchCard) -> usize {
    let matches = get_num_matches(card);
    if matches == 0 {
        return 0;
    }
    usize::pow(2, matches as u32 - 1)
}

fn get_num_matches(card: &ScratchCard) -> usize {
    card.chosen
        .iter()
        .filter(|num| card.winners.contains(num))
        .count()
}

fn get_value(cards: &[ScratchCard]) -> usize {
    cards.iter().map(get_card_value).sum()
}

fn process(cards: &[ScratchCard]) -> usize {
    let mut copies = vec![1; cards.len()];
    for idx in 0..cards.len() {
        let winning_numbers = get_num_matches(&cards[idx]);
        (1..=winning_numbers).for_each(|offset| {
            let new_idx = idx + offset;
            if new_idx < cards.len() {
                copies[new_idx] += copies[idx]
            }
        })
    }
    copies.into_iter().sum()
}

fn main() {
    let text = parse_text();
    let scratchcards = parse_scratchcards(&text);
    let value = get_value(&scratchcards);
    let changed_rule_num = process(&scratchcards);
    println!("The sum of card values is {}", value);
    println!(
        "The number of scratchcards with new rules is {}",
        changed_rule_num
    );
}
