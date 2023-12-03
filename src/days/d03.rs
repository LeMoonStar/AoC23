use std::collections::HashMap;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 3;

fn parse_map(map: &str) -> Data {
    let chars: Vec<Vec<char>> = map.lines().map(|v| v.chars().collect()).collect();
    let mut parsed_numbers: Vec<Number> = vec![];

    for (y, line) in chars.iter().enumerate() {
        let mut current_number = 0;
        let mut start_pos = None;
        let mut len = 0;

        for (x, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                if start_pos.is_none() {
                    current_number = c.to_digit(10).unwrap();
                    start_pos = Some(x);
                } else {
                    current_number *= 10;
                    current_number += c.to_digit(10).unwrap();
                }

                len += 1;
            } else if let Some(start_x) = start_pos {
                parsed_numbers.push(Number {
                    num: current_number,
                    x: start_x,
                    y,
                    len,
                });
                current_number = 0;
                start_pos = None;
                len = 0;
            }
        }

        if let Some(start_x) = start_pos {
            parsed_numbers.push(Number {
                num: current_number,
                x: start_x,
                y,
                len,
            });
        }
    }

    (chars, parsed_numbers)
}

#[derive(Debug, Clone)]
pub struct Number {
    num: u32,
    x: usize,
    y: usize,
    len: usize,
}

type Data = (Vec<Vec<char>>, Vec<Number>);
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test03.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(4361), Answer::Number(467835))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, parse_map(input))
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut sum = 0;

        for number in &data.1 {
            let mut found = false;
            for line in &data.0[0.max(number.y as i64 - 1) as usize..data.0.len().min(number.y + 2)]
            {
                for c in &line
                    [0.max(number.x as i64 - 1) as usize..line.len().min(number.x + number.len + 1)]
                {
                    if !c.is_ascii_digit() && *c != '.' {
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }

            if found {
                sum += number.num;
                continue;
            }
        }

        Answer::Number(sum as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut sum = 0;
        let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

        for number in &data.1 {
            for y in 0.max(number.y as i64 - 1) as usize..data.0.len().min(number.y + 2) {
                let line: &Vec<char> = &data.0[y];
                for (x, c) in line
                    .iter()
                    .enumerate()
                    .take(line.len().min(number.x + number.len + 1))
                    .skip(0.max(number.x as i64 - 1) as usize)
                {
                    if *c == '*' {
                        if let Some(gear) = gears.get_mut(&(x, y)) {
                            gear.push(number.num);
                        } else {
                            let gear = vec![number.num];
                            gears.insert((x, y), gear);
                        }
                    }
                }
            }
        }

        for gear in gears {
            if gear.1.len() == 2 {
                sum += gear.1[0] as u64 * gear.1[1] as u64;
            }
        }

        Answer::Number(sum)
    }
}
