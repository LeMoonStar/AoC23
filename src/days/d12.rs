use std::collections::HashMap;

use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for SpringState {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Invalid input."),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    springs: Vec<SpringState>,
    groups: Vec<u8>,
}

impl Record {
    pub fn unfold(&mut self) -> &mut Self {
        self.springs.push(SpringState::Unknown);
        self.springs = self.springs.repeat(5);
        self.springs.pop();

        self.groups = self.groups.repeat(5);

        self
    }

    pub fn find_possible_solutions(
        &self,
        pos: usize,
        group_id: usize,
        continuous_damaged: u8,
        cache: &mut Option<HashMap<(usize, usize, u8), u64>>,
    ) -> u64 {
        if let Some(cache) = cache {
            if let Some(cached) = cache.get(&(pos, group_id, continuous_damaged)) {
                dprintln!("Cache Hit!");
                return *cached;
            }
        }

        // Have we reached the end?
        if pos >= self.springs.len() {
            // Check whether the amount of finished groups is correct.
            // We might still be in a group. If so, check if it is the correct size.
            return if group_id == self.groups.len() && continuous_damaged == 0
                || group_id == self.groups.len() - 1 && continuous_damaged == self.groups[group_id]
            {
                1
            } else {
                0
            };
        }

        let res = (if self.springs[pos] == SpringState::Operational
            || self.springs[pos] == SpringState::Unknown
        {
            if continuous_damaged == 0 {
                // Operational -> Operational
                self.find_possible_solutions(pos + 1, group_id, 0, cache)
            } else if continuous_damaged > 0 && self.groups[group_id] == continuous_damaged {
                // Damaged -> Operational | Group ended with correct size.
                self.find_possible_solutions(pos + 1, group_id + 1, 0, cache)
            } else {
                // Damaged -> Operational | Group ended with wrong size. Terminate path
                0
            }
        } else {
            0
        } + if self.springs[pos] == SpringState::Damaged
            || self.springs[pos] == SpringState::Unknown
        {
            if group_id < self.groups.len() && continuous_damaged < self.groups[group_id] {
                // ANYTHING -> Damaged | There aren't too many groups and the current group isn't too big.
                self.find_possible_solutions(pos + 1, group_id, continuous_damaged + 1, cache)
            } else {
                // ANYTHING -> Damaged | The current group is too big or there are too many.
                0
            }
        } else {
            0
        });

        if let Some(cache) = cache {
            cache.insert((pos, group_id, continuous_damaged), res);
        }
        res
    }
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let (springs_str, group_str) = value.split_once(' ').unwrap();

        Self {
            springs: springs_str.chars().map(|v| v.into()).collect(),
            groups: group_str.split(',').map(|v| v.parse().unwrap()).collect(),
        }
    }
}

type Data = Vec<Record>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test12.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(21), Answer::Number(525152))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.lines().map(|v| v.into()).collect())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(
            data.iter()
                .map(|v| v.find_possible_solutions(0, 0, 0, &mut None))
                .sum(),
        )
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(
            data.iter_mut()
                .map(|v| {
                    v.unfold()
                        .find_possible_solutions(0, 0, 0, &mut Some(HashMap::new()))
                })
                .sum(),
        )
    }
}
