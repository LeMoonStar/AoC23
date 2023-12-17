use std::collections::{BinaryHeap, HashMap};

use colored::Colorize;

use crate::{dprintln, vprint, vprintln};

use super::{
    utils::{Direction, Map},
    Answer, Day, DayImpl,
};

const CURRENT_DAY: u8 = 17;

#[derive(Debug, Clone)]
pub struct Block {
    heat_loss: u8,
}

impl From<char> for Block {
    fn from(value: char) -> Self {
        Self {
            heat_loss: value.to_digit(10).unwrap() as u8,
        }
    }
}

type Pos = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct APos(usize, usize, Direction, u8, u8);

impl std::cmp::Ord for APos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.4.cmp(&other.4)
    }
}

impl PartialOrd for APos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.4.cmp(&self.4))
    }
}

pub type Data = Map<Block>;

impl Data {
    fn reconstruct_path(mut came_from: HashMap<APos, APos>, mut current: APos) -> Vec<APos> {
        let mut path = vec![current];

        dprintln!("{:?}", came_from);
        while let Some(new_current) = came_from.get(&current) {
            let new_current = *new_current;
            came_from.remove(&current);
            current = new_current;

            path.push(current);
        }

        path
    }

    fn get_neighbours(&self, pos: &APos) -> Vec<APos> {
        let mut neighbours: Vec<APos> = Vec::with_capacity(4);
        let upper_limit = (self.dimensions().0 - 1, self.dimensions().1 - 1);

        dprintln!("  pos {:?}", pos);
        // Length restriction on straight parts.
        if pos.3 < 2 {
            if let Some((x, y)) = pos.2.walk_pos((pos.0, pos.1), upper_limit) {
                neighbours.push(APos(
                    x,
                    y,
                    pos.2,
                    pos.3 + 1,
                    self.get(x, y).unwrap().heat_loss,
                ));
            }
        }

        // 90° turns
        match pos.2 {
            Direction::East => {
                if let Some((x, y)) = Direction::North.walk_pos((pos.0, pos.1), upper_limit) {
                    neighbours.push(APos(
                        x,
                        y,
                        Direction::North,
                        0,
                        self.get(x, y).unwrap().heat_loss,
                    ));
                }
                if let Some((x, y)) = Direction::South.walk_pos((pos.0, pos.1), upper_limit) {
                    neighbours.push(APos(
                        x,
                        y,
                        Direction::South,
                        0,
                        self.get(x, y).unwrap().heat_loss,
                    ));
                }
            }
            Direction::North => {
                if let Some((x, y)) = Direction::West.walk_pos((pos.0, pos.1), upper_limit) {
                    neighbours.push(APos(
                        x,
                        y,
                        Direction::West,
                        0,
                        self.get(x, y).unwrap().heat_loss,
                    ));
                }
                if let Some((x, y)) = Direction::East.walk_pos((pos.0, pos.1), upper_limit) {
                    neighbours.push(APos(
                        x,
                        y,
                        Direction::East,
                        0,
                        self.get(x, y).unwrap().heat_loss,
                    ));
                }
            }
            Direction::South => {
                if let Some((x, y)) = Direction::West.walk_pos((pos.0, pos.1), upper_limit) {
                    neighbours.push(APos(
                        x,
                        y,
                        Direction::West,
                        0,
                        self.get(x, y).unwrap().heat_loss,
                    ));
                }
                if let Some((x, y)) = Direction::East.walk_pos((pos.0, pos.1), upper_limit) {
                    neighbours.push(APos(
                        x,
                        y,
                        Direction::East,
                        0,
                        self.get(x, y).unwrap().heat_loss,
                    ));
                }
            }
            Direction::West => {
                if let Some((x, y)) = Direction::North.walk_pos((pos.0, pos.1), upper_limit) {
                    neighbours.push(APos(
                        x,
                        y,
                        Direction::North,
                        0,
                        self.get(x, y).unwrap().heat_loss,
                    ));
                }
                if let Some((x, y)) = Direction::South.walk_pos((pos.0, pos.1), upper_limit) {
                    neighbours.push(APos(
                        x,
                        y,
                        Direction::South,
                        0,
                        self.get(x, y).unwrap().heat_loss,
                    ));
                }
            }
        }

        dprintln!("    => {:?}", neighbours);
        neighbours
    }

    // Thank you, wikipedia (https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode)
    pub fn modified_a_star(&self, start: APos, goal: Pos) -> Option<Vec<APos>> {
        let mut open_set: BinaryHeap<APos> = BinaryHeap::new();
        open_set.push(start);
        let mut came_from: HashMap<APos, APos> = HashMap::new();

        let mut g_score: HashMap<APos, usize> = HashMap::new();
        g_score.insert(start, 0);

        let mut f_score: HashMap<APos, usize> = HashMap::new();
        f_score.insert(start, Self::distance((start.0, start.1), goal));

        while let Some(current) = open_set.pop() {
            /*let current = open_set
            .iter()
            .fold((usize::MAX, None), |min, pos| {
                if min.1.is_none() || *f_score.get(pos).unwrap_or(&usize::MAX) < min.0 {
                    (*f_score.get(pos).unwrap_or(&usize::MAX), Some(*pos))
                } else {
                    min
                }
            })
            .1
            .unwrap();*/

            dprintln!("Current: {:?}", current);

            if (current.0, current.1) == goal {
                return Some(Self::reconstruct_path(came_from, current));
            }

            for neighbour in self.get_neighbours(&current) {
                let tentative_g_score = g_score.get(&current).unwrap_or(&usize::MAX)
                    + self.get(neighbour.0, neighbour.1).unwrap().heat_loss as usize;

                if tentative_g_score < *g_score.get(&neighbour).unwrap_or(&usize::MAX) {
                    came_from.insert(neighbour, current);
                    g_score.insert(neighbour, tentative_g_score);
                    f_score.insert(
                        neighbour,
                        tentative_g_score + Self::distance((neighbour.0, neighbour.1), goal),
                    );
                    open_set.push(neighbour);
                }
            }
        }

        None
    }

    pub fn distance(a: Pos, b: Pos) -> usize {
        dprintln!(
            "{:?} - {:?} => {} + {}",
            a,
            b,
            a.0.abs_diff(b.0),
            a.1.abs_diff(b.1)
        );
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
        //0
    }
}

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test17.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(102), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        dprintln!("{:?}", data);
        let path = data
            .modified_a_star(
                APos(0, 0, Direction::East, 0, 0),
                (data.dimensions().0 - 1, data.dimensions().1 - 1),
            )
            .unwrap();
        dprintln!("{:?}", path);

        for y in 0..data.dimensions().1 {
            for x in 0..data.dimensions().0 {
                vprint!(
                    "{}",
                    if path.iter().any(|v| v.0 == x && v.1 == y) {
                        format!("{}", data.get(x, y).unwrap().heat_loss)
                            .green()
                            .bold()
                    } else {
                        format!("{}", data.get(x, y).unwrap().heat_loss).red()
                    }
                )
            }
            vprintln!();
        }

        Answer::Number(path.iter().rev().skip(1).fold(0, |acc, pos| {
            acc + data.get(pos.0, pos.1).unwrap().heat_loss as u64
        }))
    }

    fn two(&self, _data: &mut Data) -> Answer {
        Answer::Number(0)
    }
}
