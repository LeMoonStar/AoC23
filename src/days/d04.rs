use std::collections::HashMap;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 4;

#[derive(Debug, Clone)]
pub struct Card {
    card_id: u8,
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (header, card) = value.split_once(": ").unwrap();

        let (winning_list, numbers_list) = card.split_once(" | ").unwrap();

        Self {
            card_id: header.split_whitespace().nth(1).unwrap().parse().unwrap(),
            winning_numbers: winning_list
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
            numbers: numbers_list
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
        }
    }
}

fn traverse_card_tree(tree: &mut HashMap<usize, (Vec<usize>, Option<usize>)>, id: usize) -> usize {
    if let Some(cached) = tree.get(&id).unwrap().1 {
        cached
    } else {
        let tmp = tree.get(&id).unwrap().0.len()
            + tree
                .get(&id)
                .unwrap()
                .clone()
                .0
                .iter()
                .map(|i| traverse_card_tree(tree, *i))
                .sum::<usize>();
        tree.get_mut(&id).unwrap().1 = Some(tmp);
        tmp
    }
}

type Data = Vec<Card>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test04.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(13), Answer::Number(30))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.lines().map(|v| v.into()).collect())
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut score = 0;

        for card in data {
            let mut card_score = 0;

            for num in &card.numbers {
                if card.winning_numbers.contains(num) {
                    card_score = 1.max(card_score * 2);
                }
            }

            score += card_score;
        }

        Answer::Number(score)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut i = 0;
        let mut m: HashMap<usize, (Vec<usize>, Option<usize>)> = HashMap::new();

        loop {
            let mut wins = 0;

            for num in &data[i].numbers {
                if data[i].winning_numbers.contains(num) {
                    wins += 1;
                }
            }

            m.insert(
                i,
                (
                    (data[i].card_id as usize..(data[i].card_id as usize + wins)).collect(),
                    None,
                ),
            );

            i += 1;
            if i >= data.len() {
                break;
            }
        }

        let mut sum = data.len();
        for i in 0..data.len() {
            sum += traverse_card_tree(&mut m, i);
        }

        Answer::Number(sum as u64)
    }
}
