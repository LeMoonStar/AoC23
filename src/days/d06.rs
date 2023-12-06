use num_traits::int::PrimInt;
use std::ops::RangeBounds;
use std::str::FromStr;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 6;

#[derive(Debug, Clone)]
pub struct Race<T>
where
    T: PrimInt,
{
    time: T,
    record: T,
}

impl<T> Race<T>
where
    T: PrimInt,
{
    // This can be optimized by using maths - I intend on doing this soon,
    // but I've been doing this day's task duwing my break at school and don't
    // have enough time for this now.
    fn check_race_range<R: RangeBounds<T> + IntoIterator<Item = T>>(
        &self,
        button_press_range: R,
    ) -> u32 {
        let mut wins = 0;

        for button_press_time in button_press_range {
            if self.check_race(button_press_time) {
                wins += 1;
            }
        }

        wins
    }

    fn check_race(&self, button_press_time: T) -> bool {
        (self.time - button_press_time) * button_press_time > self.record
    }
}

impl<T> Race<T>
where
    T: PrimInt + FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    fn parse_race_list(input: &str) -> Vec<Self> {
        let mut lines = input.lines();
        let time_list: Vec<T> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|v| v.parse().unwrap())
            .collect();
        let distance_list: Vec<T> = lines
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

type Data = Vec<Race<u16>>;
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
        let mut result = 1;

        for race in data {
            result *= race.check_race_range(1..race.time);
        }

        Answer::Number(result as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let race = data
            .iter()
            .fold(Race::<u64> { record: 0, time: 0 }, |race, v| Race {
                record: race.record * 10_u64.pow((v.record as f64).log(10.0) as u32 + 1)
                    + v.record as u64,
                time: race.time * 10_u64.pow((v.time as f64).log(10.0) as u32 + 1) + v.time as u64,
            });

        Answer::Number(race.check_race_range(1..race.time) as u64)
    }
}
