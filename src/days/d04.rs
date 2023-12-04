use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 4;

#[derive(Debug, Clone)]
pub struct Card {
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (winning_list, numbers_list) = value.split_once(" | ").unwrap();

        Self {
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

type Data = Vec<Card>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test04.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(13), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| v.split_once(": ").unwrap().1.into())
                .collect(),
        )
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
        Answer::Number(data.len() as u64)
    }
}
