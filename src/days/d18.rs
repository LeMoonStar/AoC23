use std::{
    collections::{HashMap, VecDeque},
    default,
};

use colored::{Colorize, CustomColor};

use crate::{dprintln, vprint, vprintln};

use super::{utils::Direction, Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 18;

#[derive(Debug, Clone)]
pub struct Instruction {
    direction: Direction,
    count: u8,
    colour: u32, // I know, I know... One byte wasted... But this makes parsing so much easier!
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut split = value.split(' ');
        Self {
            direction: match split.next().unwrap() {
                "U" => Direction::North,
                "R" => Direction::East,
                "D" => Direction::South,
                "L" => Direction::West,
                _ => panic!(),
            },
            count: split.next().unwrap().parse::<u8>().unwrap(),
            colour: u32::from_str_radix(
                split
                    .next()
                    .unwrap()
                    .strip_prefix("(#")
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap(),
                16,
            )
            .unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Point {
    depth: u8,
    colour: Option<u32>,
}

#[derive(Debug, Clone, Default)]
struct Map {
    points: HashMap<(isize, isize), Point>,
    min_pos: (isize, isize),
    max_pos: (isize, isize),
}

impl Map {
    pub fn execute_instructions(&mut self, instructions: &[Instruction], mut pos: (isize, isize)) {
        for instruction in instructions {
            for _ in 0..instruction.count {
                pos = instruction
                    .direction
                    .walk_pos_signed(pos, (isize::MIN, isize::MIN), (isize::MAX, isize::MAX))
                    .unwrap();

                let point = *self.points.get(&pos).unwrap_or(&Point::default());
                self.points.insert(
                    pos,
                    Point {
                        depth: point.depth + 1,
                        colour: Some(instruction.colour),
                    },
                );
                self.max_pos = (self.max_pos.0.max(pos.0), self.max_pos.1.max(pos.1));
                self.min_pos = (self.min_pos.0.min(pos.0), self.min_pos.1.min(pos.1));
            }
        }
    }

    fn fill_surrounded(&mut self) {
        let mut start_pos = (0, 0);

        'find_start_pos: for y in self.min_pos.1..self.max_pos.1 + 1 {
            for x in self.min_pos.0..self.max_pos.0 + 1 {
                if !self.points.contains_key(&(x, y)) {
                    let mut has_border_to_west = false;
                    let mut has_border_to_east = false;

                    let mut a = (x, y);
                    while let Some(b) =
                        Direction::West.walk_pos_signed(a, self.min_pos, self.max_pos)
                    {
                        a = b;
                        if self.points.contains_key(&b) {
                            has_border_to_west = true;
                            break;
                        }
                    }

                    a = (x, y);
                    while let Some(b) =
                        Direction::East.walk_pos_signed(a, self.min_pos, self.max_pos)
                    {
                        a = b;
                        if self.points.contains_key(&b) {
                            has_border_to_east = true;
                            break;
                        }
                    }

                    if has_border_to_east && has_border_to_west {
                        start_pos = (x, y);
                        break 'find_start_pos;
                    }
                }
            }
        }

        let mut to_be_filled: VecDeque<(isize, isize)> = VecDeque::new();

        to_be_filled.push_back(start_pos);

        while let Some(pos) = to_be_filled.pop_front() {
            dprintln!("{:?}", pos);
            self.points.insert(pos, Point::default());

            if let Some(pos) = Direction::North.walk_pos_signed(pos, self.min_pos, self.max_pos) {
                if !self.points.contains_key(&pos) && !to_be_filled.contains(&pos) {
                    to_be_filled.push_back(pos);
                }
            }

            if let Some(pos) = Direction::East.walk_pos_signed(pos, self.min_pos, self.max_pos) {
                if !self.points.contains_key(&pos) && !to_be_filled.contains(&pos) {
                    to_be_filled.push_back(pos);
                }
            }

            if let Some(pos) = Direction::South.walk_pos_signed(pos, self.min_pos, self.max_pos) {
                if !self.points.contains_key(&pos) && !to_be_filled.contains(&pos) {
                    to_be_filled.push_back(pos);
                }
            }

            if let Some(pos) = Direction::West.walk_pos_signed(pos, self.min_pos, self.max_pos) {
                if !self.points.contains_key(&pos) && !to_be_filled.contains(&pos) {
                    to_be_filled.push_back(pos);
                }
            }
        }
    }

    fn print(&self) {
        vprintln!();
        for y in self.min_pos.1..self.max_pos.1 + 1 {
            for x in self.min_pos.0..self.max_pos.0 + 1 {
                if let Some(point) = self.points.get(&(x, y)) {
                    if let Some(colour) = point.colour {
                        vprint!(
                            "{}",
                            "#".custom_color(CustomColor::new(
                                (colour & 0x00FF0000 >> 16) as u8,
                                (colour & 0x0000FF00 >> 8) as u8,
                                (colour & 0x000000FF) as u8
                            ))
                        );
                    } else {
                        vprint!("#");
                    }
                } else {
                    vprint!(".");
                }
            }
            vprintln!();
        }
    }

    fn count_points(&self) -> usize {
        self.points.len()
    }
}

type Data = Vec<Instruction>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test18.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(62), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.lines().map(|v| v.into()).collect())
    }

    fn one(&self, data: &mut Data) -> Answer {
        dprintln!("{:?}", data);
        let mut map = Map::default();
        map.execute_instructions(data, (0, 0));
        dprintln!("{:?}", map);
        map.print();
        map.fill_surrounded();
        map.print();

        Answer::Number(map.count_points() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.len() as u64)
    }
}
