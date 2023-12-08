use crate::solutions::Solution;
pub struct DaySeven;

impl Solution for DaySeven {
    fn solve(&self, lines: Vec<String>) -> () {
        let start_time = std::time::Instant::now();
        let part_one_answer = part_one(lines.clone());
        let elapsed = start_time.elapsed();
        println!("Day 7 part 1 answer: {} ({:?})", part_one_answer, elapsed);
        let start_time = std::time::Instant::now();
        let part_two_answer = part_two();
        let elapsed = start_time.elapsed();
        println!("Day 7 part 2 answer: {} ({:?})", part_two_answer, elapsed);
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
enum Cards {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl Cards {
    fn to_index(&self) -> usize {
        match self {
            Cards::Two => 0,
            Cards::Three => 1,
            Cards::Four => 2,
            Cards::Five => 3,
            Cards::Six => 4,
            Cards::Seven => 5,
            Cards::Eight => 6,
            Cards::Nine => 7,
            Cards::Ten => 8,
            Cards::Jack => 9,
            Cards::Queen => 10,
            Cards::King => 11,
            Cards::Ace => 12,
        }
    }
}

#[derive(Debug)]
enum Types {
    FiveOfAKind(Vec<Cards>),
    FourOfAKind(Vec<Cards>),
    FullHouse(Vec<Cards>),
    ThreeOfAKind(Vec<Cards>),
    TwoPair(Vec<Cards>),
    OnePair(Vec<Cards>),
    HighCard(Vec<Cards>),
}

impl PartialEq for Types {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Types::FiveOfAKind(_) => match other {
                Types::FiveOfAKind(_) => true,
                _ => false,
            },
            Types::FourOfAKind(_) => match other {
                Types::FourOfAKind(_) => true,
                _ => false,
            },
            Types::FullHouse(_) => match other {
                Types::FullHouse(_) => true,
                _ => false,
            },
            Types::ThreeOfAKind(_) => match other {
                Types::ThreeOfAKind(_) => true,
                _ => false,
            },
            Types::TwoPair(_) => match other {
                Types::TwoPair(_) => true,
                _ => false,
            },
            Types::OnePair(_) => match other {
                Types::OnePair(_) => true,
                _ => false,
            },
            Types::HighCard(_) => match other {
                Types::HighCard(_) => true,
                _ => false,
            },
        }
    }
}

impl PartialOrd for Types {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Types::FiveOfAKind(first_array) => match other {
                Types::FiveOfAKind(second_array) => first_array.partial_cmp(second_array),
                _ => Some(std::cmp::Ordering::Greater),
            },
            Types::FourOfAKind(first_array) => match other {
                Types::FourOfAKind(second_array) => first_array.partial_cmp(second_array),
                Types::FiveOfAKind(_) => Some(std::cmp::Ordering::Less),
                _ => Some(std::cmp::Ordering::Greater),
            },
            Types::FullHouse(first_array) => match other {
                Types::FullHouse(second_array) => first_array.partial_cmp(second_array),
                Types::FiveOfAKind(_) => Some(std::cmp::Ordering::Less),
                Types::FourOfAKind(_) => Some(std::cmp::Ordering::Less),
                _ => Some(std::cmp::Ordering::Greater),
            },
            Types::ThreeOfAKind(first_array) => match other {
                Types::ThreeOfAKind(second_array) => first_array.partial_cmp(second_array),
                Types::FiveOfAKind(_) => Some(std::cmp::Ordering::Less),
                Types::FourOfAKind(_) => Some(std::cmp::Ordering::Less),
                Types::FullHouse(_) => Some(std::cmp::Ordering::Less),
                _ => Some(std::cmp::Ordering::Greater),
            },
            Types::TwoPair(first_array) => match other {
                Types::TwoPair(second_array) => first_array.partial_cmp(second_array),
                Types::FiveOfAKind(_) => Some(std::cmp::Ordering::Less),
                Types::FourOfAKind(_) => Some(std::cmp::Ordering::Less),
                Types::FullHouse(_) => Some(std::cmp::Ordering::Less),
                Types::ThreeOfAKind(_) => Some(std::cmp::Ordering::Less),
                _ => Some(std::cmp::Ordering::Greater),
            },
            Types::OnePair(first_array) => match other {
                Types::OnePair(second_array) => first_array.partial_cmp(second_array),
                Types::FiveOfAKind(_) => Some(std::cmp::Ordering::Less),
                Types::FourOfAKind(_) => Some(std::cmp::Ordering::Less),
                Types::FullHouse(_) => Some(std::cmp::Ordering::Less),
                Types::ThreeOfAKind(_) => Some(std::cmp::Ordering::Less),
                Types::TwoPair(_) => Some(std::cmp::Ordering::Less),
                _ => Some(std::cmp::Ordering::Greater),
            },
            Types::HighCard(first_array) => match other {
                Types::HighCard(second_array) => first_array.partial_cmp(second_array),
                _ => Some(std::cmp::Ordering::Less),
            },
        }
    }
}

impl From<Hand> for Types {
    fn from(hand: Hand) -> Self {
        let cards = hand.cards.clone();
        let mut counts = vec![0; 13];
        for card in cards.iter() {
            counts[card.to_index()] += 1;
        }
        counts.sort();
        counts.reverse();
        match counts[0] {
            5 => Types::FiveOfAKind(hand.cards),
            4 => Types::FourOfAKind(hand.cards),
            3 => match counts[1] {
                2 => Types::FullHouse(hand.cards),
                _ => Types::ThreeOfAKind(hand.cards),
            },
            2 => match counts[1] {
                2 => Types::TwoPair(hand.cards),
                _ => Types::OnePair(hand.cards),
            },
            _ => Types::HighCard(cards),
        }
    }
}

struct Hand {
    cards: Vec<Cards>,
}

impl From<String> for Hand {
    fn from(hand: String) -> Self {
        let cards = hand
            .chars()
            .map(|card| match card {
                'A' => Cards::Ace,
                '2' => Cards::Two,
                '3' => Cards::Three,
                '4' => Cards::Four,
                '5' => Cards::Five,
                '6' => Cards::Six,
                '7' => Cards::Seven,
                '8' => Cards::Eight,
                '9' => Cards::Nine,
                'T' => Cards::Ten,
                'J' => Cards::Jack,
                'Q' => Cards::Queen,
                'K' => Cards::King,
                _ => panic!("Invalid card"),
            })
            .collect::<Vec<_>>();
        Hand { cards }
    }
}

fn part_one(lines: Vec<String>) -> usize {
    let mut hands = lines
        .iter()
        .map(|line| {
            let hand: Hand = line.split_whitespace().take(1).collect::<String>().into();
            let types = Types::from(hand);
            let bid = line
                .split_whitespace()
                .skip(1)
                .take(1)
                .nth(0)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            (types, bid)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            (i + 1) * hand.1
        })
        .sum::<usize>()
}

fn part_two() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::utils::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let lines = read_file(&"2023/example7".to_string());
        assert_eq!(part_one(lines), 6440);
    }

    #[test]
    fn test_part_two() {}
}
