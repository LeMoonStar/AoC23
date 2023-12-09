use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 9;

#[derive(Debug, Clone)]
pub struct DifferencesTable {
    layers: Vec<Vec<i64>>,
}

impl DifferencesTable {
    // Delta time of 14 mins for this simple task cuz SOMEBODY decided to
    // start making a lot of noise, which killed my concentration.
    pub fn predict_previous_number(&self) -> i64 {
        let mut n = 0;
        for layer in self.layers.iter().rev() {
            n = layer.first().unwrap() - n;
        }

        n
    }

    pub fn predict_next_number(&self) -> i64 {
        let mut n = 0;
        for d in self.layers.iter().rev() {
            n += d.last().unwrap();
        }

        n
    }

    pub fn create(first_layer: &[i64]) -> Self {
        let mut layers = vec![first_layer.to_owned()];

        loop {
            let new_layer = Self::calculate_next_layer(layers.last().unwrap());
            if Self::is_zero_layer(&new_layer) {
                break;
            }
            layers.push(new_layer);
        }

        Self { layers }
    }

    fn calculate_next_layer(layer: &[i64]) -> Vec<i64> {
        layer.windows(2).map(|pair| pair[1] - pair[0]).collect()
    }

    fn is_zero_layer(layer: &[i64]) -> bool {
        for n in layer {
            if *n != 0 {
                return false;
            }
        }
        true
    }
}

type Data = Vec<Vec<i64>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test09.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(114), Answer::Number(2))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| {
                    v.split_whitespace()
                        .map(|v| v.parse::<i64>().expect("error while parsing input."))
                        .collect()
                })
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let tables: Vec<DifferencesTable> = data
            .iter()
            .map(|v| DifferencesTable::create(v.as_slice()))
            .collect();
        Answer::Number(tables.iter().map(|v| v.predict_next_number()).sum::<i64>() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let tables: Vec<DifferencesTable> = data
            .iter()
            .map(|v| DifferencesTable::create(v.as_slice()))
            .collect();
        Answer::Number(
            tables
                .iter()
                .map(|v| v.predict_previous_number())
                .sum::<i64>() as u64,
        )
    }
}
