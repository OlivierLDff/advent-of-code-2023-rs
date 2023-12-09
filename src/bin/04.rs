advent_of_code::solution!(4);

use std::collections::HashMap;

use anyhow::{Context, Result};

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>,
}

impl Card {
    fn get_winning_count(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|n| self.owned_numbers.contains(n))
            .count() as u32
    }
}

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .trim()
        .split_ascii_whitespace()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect()
}

fn parse_card(input: &str) -> Result<Card> {
    let mut parts = input.split(": ");
    let id = parts
        .next()
        .context("There should be a part before the colon, for 'Card x'")?
        .split_ascii_whitespace()
        .nth(1)
        .context("There should be a second part, for the card number")?
        .parse()
        .context("The card number should be a number")?;

    let mut numbers = parts
        .next()
        .context("There should be a part after the colon, for the numbers")?
        .split(" | ");

    let winning_numbers = parse_numbers(
        numbers
            .next()
            .context("There should be a part for the winning numbers")?,
    );
    let owned_numbers = parse_numbers(
        numbers
            .next()
            .context("There should be a part for the owned numbers")?,
    );

    Ok(Card {
        id,
        winning_numbers,
        owned_numbers,
    })
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(parse_card)
        .filter_map(|c| match c {
            Ok(c) => Some(c),
            Err(_) => {
                eprintln!("Failed to parse card: {:?}", c);
                None
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse_cards(input);
    let points = cards
        .iter()
        .map(Card::get_winning_count)
        .map(|c| if c > 0 { 2u32.pow(c - 1) } else { 0 })
        .sum();
    Some(points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse_cards(input);

    let mut card_amount: HashMap<u32, u32> = HashMap::from_iter(cards.iter().map(|c| (c.id, 1u32)));
    let card_scores: HashMap<u32, u32> =
        HashMap::from_iter(cards.iter().map(|c| (c.id, c.get_winning_count())));
    for card in &cards {
        let current_amount = *card_amount
            .get(&card.id)
            .expect("Card should have an amount");

        let score = card_scores.get(&card.id).expect("Card should have a score");
        for next_id in card.id + 1..=card.id + score {
            card_amount
                .entry(next_id)
                .and_modify(|c| *c += current_amount);
        }
    }

    Some(card_amount.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#"
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#;

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE);
        assert_eq!(result, Some(30));
    }
}
