use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 15;

type Data = Vec<String>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test15.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(1320), Answer::Number(145))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input.trim_end().split(',').map(|v| v.to_owned()).collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.iter().map(|v| Self::hash_string(v)).sum::<usize>() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut boxes: Vec<Vec<(&str, usize)>> = Vec::with_capacity(256);

        for _ in 0..256 {
            boxes.push(vec![]);
        }

        for step in data {
            if step.ends_with('-') {
                let label = step.split_once('-').unwrap().0;
                let hash = Self::hash_string(label);

                if let Some(index) = boxes[hash].iter().position(|v| v.0 == label) {
                    boxes[hash].remove(index);
                }
            } else {
                let (label, num_str) = step.split_once('=').unwrap();
                let num = num_str.parse::<usize>().unwrap();
                let hash = Self::hash_string(label);

                if let Some(index) = boxes[hash].iter().position(|v| v.0 == label) {
                    boxes[hash][index] = (label, num);
                } else {
                    boxes[hash].push((label, num));
                }
            }
        }

        Answer::Number(
            boxes
                .iter()
                .enumerate()
                .map(|(b_num, b)| {
                    b.iter()
                        .enumerate()
                        .map(|(slot, lens)| (1 + b_num) * (slot + 1) * lens.1)
                        .sum::<usize>()
                })
                .sum::<usize>() as u64,
        )
    }
}

impl Day<CURRENT_DAY> {
    fn hash_string(s: &str) -> usize {
        let mut hash = 0;

        for c in s.bytes() {
            hash = ((hash + c as usize) * 17) % 256;
        }

        hash
    }
}
