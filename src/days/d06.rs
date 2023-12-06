use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 6;

#[derive(Debug, Clone)]
pub struct Race<T> {
    time: T,
    record: T,
}

impl Race<u16> {
    fn parse_race_list(input: &str) -> Vec<Self> {
        let mut lines = input.lines();
        let time_list: Vec<u16> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|v| v.parse().unwrap())
            .collect();
        let distacne_list: Vec<u16> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|v| v.parse().unwrap())
            .collect();

        time_list
            .iter()
            .zip(distacne_list.iter())
            .map(|(time, distance)| Self {
                time: *time,
                record: *distance,
            })
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

        for race in &*data {
            let mut wins = 0;
            for button_press_time in 1..race.time {
                let distance_travelled = (race.time - button_press_time) * button_press_time;
                if distance_travelled > race.record {
                    wins += 1;
                }
            }

            result *= wins;
        }

        Answer::Number(result)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let race = data
            .iter()
            .fold(Race::<u64> { record: 0, time: 0 }, |race, v| Race {
                record: race.record * 10_u64.pow((v.record as f64).log(10.0) as u32 + 1)
                    + v.record as u64,
                time: race.time * 10_u64.pow((v.time as f64).log(10.0) as u32 + 1) + v.time as u64,
            });
        let mut result = 1;

        let mut wins = 0;
        for button_press_time in 1..race.time {
            let distance_travelled = (race.time - button_press_time) * button_press_time;
            if distance_travelled > race.record {
                wins += 1;
            }
        }

        result *= wins;

        Answer::Number(result)
    }
}
