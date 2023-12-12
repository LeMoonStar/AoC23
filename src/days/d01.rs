use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 1;

#[derive(Debug, Copy, Clone)]
pub enum NumberType {
    Digit(u8),
    Spelled(u8),
}

impl NumberType {
    fn get_value(&self) -> u8 {
        match self {
            Self::Digit(v) => *v,
            Self::Spelled(v) => *v,
        }
    }

    fn parse_next(text: &str) -> (Option<Self>, &str) {
        let first_char = text.chars().next().unwrap();

        if first_char.is_numeric() {
            (
                Some(Self::Digit(
                    first_char.to_digit(10).unwrap().try_into().unwrap(),
                )),
                &text[1..],
            )
        } else if text.starts_with("one") {
            (
                Some(Self::Spelled(1)),
                /*text.strip_prefix("one").unwrap()*/ &text[1..],
            ) // Numbers can overlap... that took quite a while to figure out.
        } else if text.starts_with("two") {
            (
                Some(Self::Spelled(2)),
                /*text.strip_prefix("two").unwrap()*/ &text[1..],
            )
        } else if text.starts_with("three") {
            (
                Some(Self::Spelled(3)),
                /*text.strip_prefix("three").unwrap()*/ &text[1..],
            )
        } else if text.starts_with("four") {
            (
                Some(Self::Spelled(4)),
                /*text.strip_prefix("four").unwrap()*/ &text[1..],
            )
        } else if text.starts_with("five") {
            (
                Some(Self::Spelled(5)),
                /*text.strip_prefix("five").unwrap()*/ &text[1..],
            )
        } else if text.starts_with("six") {
            (
                Some(Self::Spelled(6)),
                /*text.strip_prefix("six").unwrap()*/ &text[1..],
            )
        } else if text.starts_with("seven") {
            (
                Some(Self::Spelled(7)),
                /*text.strip_prefix("seven").unwrap()*/ &text[1..],
            )
        } else if text.starts_with("eight") {
            (
                Some(Self::Spelled(8)),
                /*text.strip_prefix("eight").unwrap()*/ &text[1..],
            )
        } else if text.starts_with("nine") {
            (
                Some(Self::Spelled(9)),
                /*text.strip_prefix("nine").unwrap()*/ &text[1..],
            )
        } else {
            (None, &text[1..])
        }
    }

    fn parse_line(mut line: &str) -> Vec<Self> {
        let mut parsed = vec![];

        while !line.is_empty() {
            let spelled_digit;
            (spelled_digit, line) = Self::parse_next(line);
            if let Some(spelled_digit) = spelled_digit {
                parsed.push(spelled_digit);
            }
        }

        parsed
    }
}

type Data = Vec<Vec<NumberType>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test01.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(55712), Answer::Number(55413))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.lines().map(NumberType::parse_line).collect())
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut sum: u64 = 0;

        for line in data {
            let digits: Vec<u8> = line
                .iter()
                .filter_map(|v| {
                    if let NumberType::Digit(x) = v {
                        Some(*x)
                    } else {
                        None
                    }
                })
                .collect();

            sum += std::convert::Into::<u64>::into(
                digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0),
            )
        }

        Answer::Number(sum)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut sum: u64 = 0;

        for digits in data {
            sum += std::convert::Into::<u64>::into(
                digits.first().unwrap().get_value() * 10 + digits.last().unwrap().get_value(),
            )
        }

        Answer::Number(sum)
    }
}
