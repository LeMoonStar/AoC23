use std::collections::HashMap;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 3;

fn parse_map(map: &str) -> Data {
    let chars: Vec<Vec<char>> = map.lines().map(|v| v.chars().collect()).collect();
    let mut parsed_numbers: Vec<(u32, (usize, usize, usize))> = vec![];

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
                parsed_numbers.push((current_number, (start_x, y, len)));
                current_number = 0;
                start_pos = None;
                len = 0;
            }
        }

        if let Some(start_x) = start_pos {
            parsed_numbers.push((current_number, (start_x, y, len)));
        }
    }

    (chars, parsed_numbers)
}

type Data = (Vec<Vec<char>>, Vec<(u32, (usize, usize, usize))>);
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
            for line in
                &data.0[0.max(number.1 .1 as i64 - 1) as usize..data.0.len().min(number.1 .1 + 2)]
            {
                for c in &line[0.max(number.1 .0 as i64 - 1) as usize
                    ..line.len().min(number.1 .0 + number.1 .2 + 1)]
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
                sum += number.0;
                continue;
            }
        }

        Answer::Number(sum as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut sum = 0;
        let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

        for number in &data.1 {
            //let mut found = false;
            for y in 0.max(number.1 .1 as i64 - 1) as usize..data.0.len().min(number.1 .1 + 2) {
                let line: &Vec<char> = &data.0[y];
                for x in 0.max(number.1 .0 as i64 - 1) as usize
                    ..line.len().min(number.1 .0 + number.1 .2 + 1)
                {
                    if line[x] == '*' {
                        if let Some(gear) = gears.get_mut(&(x, y)) {
                            gear.push(number.0);
                        } else {
                            let gear = vec![number.0];
                            gears.insert((x, y), gear);
                        }
                        //found = true;
                        //break;
                    }
                }
                //if found {
                //    break;
                //}
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
