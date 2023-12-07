use std::{cmp::Ordering, collections::HashMap};

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 7;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
}

impl Card {
    fn part2_cmp(&self, other: &Card) -> std::cmp::Ordering {
        if self == other {
            Ordering::Equal
        } else {
            match (self, other) {
                (Self::J, _) => Ordering::Greater,
                (_, Self::J) => Ordering::Less,
                _ => self.partial_cmp(other).unwrap(),
            }
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::N9,
            '8' => Self::N8,
            '7' => Self::N7,
            '6' => Self::N6,
            '5' => Self::N5,
            '4' => Self::N4,
            '3' => Self::N3,
            '2' => Self::N2,
            _ => panic!("Invalid Input"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    bid: u16,
}

impl Hand {
    fn part2_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for (s, o) in self.cards.iter().zip(other.cards.iter()) {
            match s.part2_cmp(o) {
                Ordering::Equal => continue,
                a => return Some(a),
            }
        }

        Some(Ordering::Equal)
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for (s, o) in self.cards.iter().zip(other.cards.iter()) {
            match s.cmp(o) {
                Ordering::Equal => continue,
                a => return Some(a),
            }
        }

        Some(Ordering::Equal)
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut counted_cards: HashMap<Card, u8> = HashMap::new();

        for card in &self.cards {
            counted_cards.insert(*card, counted_cards.get(card).unwrap_or(&0) + 1);
        }

        let mut cards: Vec<u8> = counted_cards.values().copied().collect::<Vec<u8>>();
        cards.sort();
        match cards.len() {
            1 => HandType::FiveOfAKind,
            5 => HandType::HighCard,
            2 => match (cards[0], cards[1]) {
                (1, 4) => HandType::FourOfAKind,
                (2, 3) => HandType::FullHouse,
                _ => panic!("Inpossible Input"),
            },
            3 => match (cards[0], cards[1], cards[2]) {
                (1, 1, 3) => HandType::ThreeOfAKind,
                (1, 2, 2) => HandType::TwoPair,
                _ => panic!("Impossible Input"),
            },
            4 => HandType::OnePair,
            _ => panic!("Imposssible Input"),
        }
    }

    fn get_part2_type(&self) -> HandType {
        let mut counted_cards: HashMap<Card, u8> = HashMap::new();

        for card in &self.cards {
            counted_cards.insert(*card, counted_cards.get(card).unwrap_or(&0) + 1);
        }

        let mut cards: Vec<(Card, u8)> = counted_cards.into_iter().collect();
        cards.sort_by(|a, b| a.1.cmp(&b.1));
        match cards.len() {
            1 => HandType::FiveOfAKind,
            5 => match cards.contains(&(Card::J, 1)) {
                true => HandType::OnePair,
                false => HandType::HighCard,
            },
            2 => match (cards[0], cards[1]) {
                ((Card::J, 1), (_, 4)) => HandType::FiveOfAKind,
                ((Card::J, 2), (_, 3)) => HandType::FiveOfAKind,
                ((_, 1), (Card::J, 4)) => HandType::FiveOfAKind,
                ((_, 2), (Card::J, 3)) => HandType::FiveOfAKind,

                ((_, 1), (_, 4)) => HandType::FourOfAKind,
                ((_, 2), (_, 3)) => HandType::FullHouse,
                _ => panic!("Impossible Input"),
            },
            3 => match (cards[0], cards[1], cards[2]) {
                ((Card::J, 1), (_, 1), (_, 3)) => HandType::FourOfAKind,
                ((_, 1), (Card::J, 1), (_, 3)) => HandType::FourOfAKind,
                ((Card::J, 1), (_, 2), (_, 2)) => HandType::FullHouse,
                ((_, 1), (Card::J, 2), (_, 2)) => HandType::FourOfAKind,
                ((_, 1), (_, 2), (Card::J, 2)) => HandType::FourOfAKind,
                ((_, 1), (_, 1), (Card::J, 3)) => HandType::FourOfAKind,
                ((_, 1), (_, 1), (_, 3)) => HandType::ThreeOfAKind,
                ((_, 1), (_, 2), (_, 2)) => HandType::TwoPair,
                _ => panic!("Impossible Input"),
            },
            4 => match (cards[0], cards[1], cards[2], cards[3]) {
                ((Card::J, 1), (_, 1), (_, 1), (_, 2)) => HandType::ThreeOfAKind,
                ((_, 1), (Card::J, 1), (_, 1), (_, 2)) => HandType::ThreeOfAKind,
                ((_, 1), (_, 1), (Card::J, 1), (_, 2)) => HandType::ThreeOfAKind,
                ((_, 1), (_, 1), (_, 1), (Card::J, 2)) => HandType::ThreeOfAKind,
                ((_, 1), (_, 1), (_, 1), (_, 2)) => HandType::OnePair,
                _ => panic!("Impossible Input"),
            },
            _ => panic!("Imposssible Input"),
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (cards_str, bid_str) = value.split_once(' ').unwrap();
        Self {
            cards: cards_str.chars().map(|v| v.into()).collect(),
            bid: bid_str.parse().unwrap(),
        }
    }
}

type Data = Vec<Hand>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test07.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(6440), Answer::Number(5905))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.lines().map(|v| v.into()).collect())
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut data: Vec<(Hand, HandType)> =
            data.iter().map(|v| (v.clone(), v.get_type())).collect();

        data.sort_by(|a, b| match b.1.cmp(&a.1) {
            Ordering::Equal => b.0.partial_cmp(&a.0).unwrap(),
            other => other,
        });

        Answer::Number(data.iter().enumerate().fold(0, |acc, (rank, hand)| {
            acc + ((rank + 1) * hand.0.bid as usize)
        }) as u64)
    }

    // This took soooooooooooooo much debugging
    fn two(&self, data: &mut Data) -> Answer {
        let mut data: Vec<(Hand, HandType)> = data
            .iter()
            .map(|v| (v.clone(), v.get_part2_type()))
            .collect();

        data.sort_by(|a, b| match b.1.cmp(&a.1) {
            Ordering::Equal => b.0.part2_cmp(&a.0).unwrap(),
            other => other,
        });

        Answer::Number(data.iter().enumerate().fold(0, |acc, (rank, hand)| {
            acc + ((rank + 1) * hand.0.bid as usize)
        }) as u64)
    }
}
