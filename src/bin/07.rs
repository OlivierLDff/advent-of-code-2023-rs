use std::collections::HashMap;

advent_of_code::solution!(7);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn to_int(self) -> u32 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_int().cmp(&other.to_int())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Number(u8),
    WeakJoker,
}

impl Card {
    fn to_int(self) -> u32 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::Number(n) => n as u32,
            Card::WeakJoker => 0,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_int().cmp(&other.to_int())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bid: u32,
}

fn compare_cards_values(cards: &[Card; 5], other_cards: &[Card; 5]) -> std::cmp::Ordering {
    for (v, other_v) in cards.iter().zip(other_cards.iter()) {
        if v == other_v {
            continue;
        }
        return v.cmp(other_v);
    }
    std::cmp::Ordering::Equal
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            compare_cards_values(&self.cards, &other.cards)
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_card(c: char, use_weak_joker: bool) -> Card {
    match c {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => {
            if use_weak_joker {
                Card::WeakJoker
            } else {
                Card::J
            }
        }
        'T' => Card::T,
        c => {
            let n = c.to_string().parse().expect("Should be a number");
            match n {
                2..=9 => Card::Number(n),
                _ => panic!("unexpected number {}", n),
            }
        }
    }
}

fn get_hand_type(cards: &[Card; 5]) -> HandType {
    let mut cards_count = cards
        .iter()
        .fold(HashMap::<Card, u32>::new(), |mut acc, card| {
            acc.entry(*card)
                .and_modify(|card_count| *card_count += 1)
                .or_insert(1);
            acc
        });

    if cards_count.values().any(|c| *c == 5) {
        return HandType::FiveOfAKind;
    }

    let joker_count = cards_count.get(&Card::WeakJoker).copied().unwrap_or(0);
    cards_count.remove(&Card::WeakJoker);

    if cards_count.values().any(|c| *c == 4) {
        if joker_count == 1 {
            return HandType::FiveOfAKind;
        }
        return HandType::FourOfAKind;
    }
    if cards_count.values().any(|c| *c == 3) {
        if joker_count == 2 {
            return HandType::FiveOfAKind;
        }
        if joker_count == 1 {
            return HandType::FourOfAKind;
        }
        if cards_count.values().any(|c| *c == 2) {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    }
    match cards_count.values().filter(|c| **c == 2).count() {
        2 => match joker_count {
            2 => return HandType::FourOfAKind,
            1 => return HandType::FullHouse,
            _ => return HandType::TwoPair,
        },
        1 => match joker_count {
            3 => return HandType::FiveOfAKind,
            2 => return HandType::FourOfAKind,
            1 => return HandType::ThreeOfAKind,
            _ => return HandType::OnePair,
        },
        _ => {}
    }
    match joker_count {
        4 => HandType::FiveOfAKind,
        3 => HandType::FourOfAKind,
        2 => HandType::ThreeOfAKind,
        1 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn parse_hand(input: &str, use_weak_joker: bool) -> Hand {
    let mut parts = input.split_whitespace();
    let cards = parts
        .next()
        .expect("There should be a hand")
        .chars()
        .map(|c| parse_card(c, use_weak_joker))
        .collect::<Vec<_>>()
        .try_into()
        .expect("There should be 5 cards");

    let bid = parts
        .next()
        .map(|bid| bid.parse().expect("bid should be a number"))
        .expect("There should be a bid");
    let hand_type = get_hand_type(&cards);
    Hand {
        cards,
        hand_type,
        bid,
    }
}

fn parse_hands(input: &str, use_weak_joker: bool) -> Vec<Hand> {
    input
        .lines()
        .map(|line| parse_hand(line, use_weak_joker))
        .collect()
}

fn solve(input: &str, use_weak_joker: bool) -> Option<u32> {
    let mut hands = parse_hands(input, use_weak_joker);
    hands.sort_unstable();

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i + 1) as u32 * h.bid)
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    static EXAMPLE2: &str = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";

    #[test]
    fn test_get_hand_type() {
        assert_eq!(
            get_hand_type(&[Card::A, Card::A, Card::A, Card::A, Card::A]),
            HandType::FiveOfAKind
        );
        assert_eq!(
            get_hand_type(&[Card::A, Card::A, Card::A, Card::A, Card::K]),
            HandType::FourOfAKind
        );
        assert_eq!(
            get_hand_type(&[Card::A, Card::A, Card::A, Card::K, Card::T]),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            get_hand_type(&[
                Card::Number(2),
                Card::Number(3),
                Card::Number(3),
                Card::Number(2),
                Card::Number(2)
            ]),
            HandType::FullHouse
        );
        assert_eq!(
            get_hand_type(&[Card::A, Card::A, Card::A, Card::K, Card::Q]),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            get_hand_type(&[Card::A, Card::A, Card::K, Card::K, Card::Q]),
            HandType::TwoPair
        );
        assert_eq!(
            get_hand_type(&[Card::A, Card::A, Card::K, Card::Q, Card::J]),
            HandType::OnePair
        );
        assert_eq!(
            get_hand_type(&[Card::A, Card::K, Card::Q, Card::J, Card::T]),
            HandType::HighCard
        );

        assert_eq!(
            get_hand_type(&[
                Card::T,
                Card::Number(5),
                Card::Number(5),
                Card::WeakJoker,
                Card::Number(5)
            ]),
            HandType::FourOfAKind
        );
        assert_eq!(
            get_hand_type(&[
                Card::T,
                Card::Number(5),
                Card::Number(5),
                Card::WeakJoker,
                Card::WeakJoker
            ]),
            HandType::FourOfAKind
        );
        assert_eq!(
            get_hand_type(&[Card::Q, Card::Q, Card::Q, Card::WeakJoker, Card::A]),
            HandType::FourOfAKind
        );
        assert_eq!(
            get_hand_type(&[Card::Q, Card::Q, Card::Q, Card::WeakJoker, Card::WeakJoker]),
            HandType::FiveOfAKind
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, Some(6440));

        let result = part_one(EXAMPLE2);
        assert_eq!(result, Some(6592));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE);
        assert_eq!(result, Some(5905));
        let result = part_two(EXAMPLE2);
        assert_eq!(result, Some(6839));
    }
}
