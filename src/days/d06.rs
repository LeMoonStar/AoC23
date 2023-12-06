use num_traits::int::PrimInt;
use std::ops::RangeBounds;
use std::str::FromStr;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 6;

#[derive(Debug, Clone)]
pub struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn count_wins(&self) -> u64 {
        let tmp = ((self.time as f64 / 2.0).powf(2.0) - self.record as f64).sqrt();

        let max = self.time as f64 / 2.0 + tmp;
        let min = self.time as f64 / 2.0 - tmp;

        let res = max.ceil() - min.max(0.0).ceil() + if tmp % 1.0 == 0.0 { -1.0 } else { 0.0 };

        (res.floor() as i64).try_into().unwrap()
    }
}

impl Race {
    fn parse_race_list(input: &str) -> Vec<Self> {
        let mut lines = input.lines();
        let time_list: Vec<u64> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|v| v.parse().unwrap())
            .collect();
        let distance_list: Vec<u64> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|v| v.parse().unwrap())
            .collect();

        time_list
            .into_iter()
            .zip(distance_list)
            .map(|(time, record)| Self { time, record })
            .collect()
    }
}

type Data = Vec<Race>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test06.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(288), Answer::Number(71503))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, Race::parse_race_list(input))
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.iter().map(|v| v.count_wins()).product())
    }

    fn two(&self, data: &mut Data) -> Answer {
        let race = data
            .iter()
            .fold(Race { record: 0, time: 0 }, |race, v| Race {
                record: race.record * 10_u64.pow((v.record as f64).log(10.0) as u32 + 1) + v.record,
                time: race.time * 10_u64.pow((v.time as f64).log(10.0) as u32 + 1) + v.time,
            });

        Answer::Number(race.count_wins())
    }
}
