use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 2;

#[derive(Debug, Clone, Copy)]
pub struct Handful {
    red: u8,
    green: u8,
    blue: u8,
}

impl From<&str> for Handful {
    fn from(value: &str) -> Self {
        let mut out = Self {
            red: 0,
            green: 0,
            blue: 0,
        };

        for colour_count in value.split(", ") {
            let (num, colour) = colour_count.split_once(' ').unwrap();
            let num = num.parse::<u8>().unwrap();
            match colour {
                "red" => out.red = num,
                "green" => out.green = num,
                "blue" => out.blue = num,
                _ => panic!("unexpected input"),
            }
        }

        out
    }
}

type Data = Vec<Vec<Handful>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test02.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(8), Answer::Number(2286))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|line| {
                    line.split_once(": ")
                        .unwrap()
                        .1
                        .split("; ")
                        .map(|handful| handful.into())
                        .collect()
                })
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut result: u64 = 0;

        for (game_id, game) in data.iter().enumerate() {
            let mut possible = true;
            for hand in game {
                if hand.red > 12 || hand.green > 13 || hand.blue > 14 {
                    possible = false;
                }
            }

            if possible {
                result += (game_id + 1) as u64;
            }
        }
        Answer::Number(result)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut result: u64 = 0;

        for game in data {
            let mut max_hand: Handful = Handful {
                red: 0,
                green: 0,
                blue: 0,
            };

            for hand in game {
                if hand.red > max_hand.red {
                    max_hand.red = hand.red;
                }
                if hand.green > max_hand.green {
                    max_hand.green = hand.green;
                }
                if hand.blue > max_hand.blue {
                    max_hand.blue = hand.blue;
                }
            }

            result += max_hand.red as u64 * max_hand.green as u64 * max_hand.blue as u64;
        }
        Answer::Number(result)
    }
}
