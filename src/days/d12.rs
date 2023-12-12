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
    pub fn find_possible_solutions(&self, springs: Option<&Vec<SpringState>>) -> u64 {
        let springs = springs.unwrap_or(&self.springs);

        for i in 0..springs.len() {
            if springs[i] == SpringState::Unknown {
                let mut a = springs.clone();
                let mut b = springs.clone();

                a[i] = SpringState::Operational;
                b[i] = SpringState::Damaged;

                return self.find_possible_solutions(Some(&a))
                    + self.find_possible_solutions(Some(&b));
            }
        }

        if self.check_spring_groups(springs) {
            1
        } else {
            0
        }
    }

    // HOLY F this is ugly... Way too tired today to re-write this in a nice way tho.
    pub fn check_spring_groups(&self, springs: &Vec<SpringState>) -> bool {
        let mut damaged = 0;
        let mut group_num = 0;

        dprintln!("{:?} - {:?}", self.springs, self.groups);
        dprintln!("{:?} - {:?}", springs, self.groups);

        let mut springs = springs.clone();
        springs.push(SpringState::Operational);

        for spring in springs {
            //dprintln!(" s: {:?}", spring);
            if spring == SpringState::Damaged {
                damaged += 1;
            } else if damaged != 0 {
                dprintln!("  Group of {} damaged", damaged);
                if group_num >= self.groups.len() {
                    dprintln!("    Too many groups: {} > {}", group_num, self.groups.len());
                    return false;
                } else {
                    dprintln!("    Checking group: {}", group_num);
                    dprintln!("      Expecting: {}", self.groups[group_num]);
                    dprintln!("      Damaged: {}", damaged);
                    if damaged == self.groups[group_num] {
                        dprintln!("      => Valid.");
                        damaged = 0;
                        group_num += 1;
                    } else {
                        dprintln!("      => Failed.");
                        return false;
                    }
                }
            }
        }

        dprintln!("  No invalid groups found. Checking group count.");
        dprintln!("  Found: {} Expected: {}.", group_num, self.groups.len());
        if group_num == self.groups.len() {
            dprintln!("    => Passed.");
            true
        } else {
            dprintln!("    => Failed.");
            false
        }
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
        (Answer::Number(21), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.lines().map(|v| v.into()).collect())
    }

    fn one(&self, data: &mut Data) -> Answer {
        dprintln!("{:?}", data);
        Answer::Number(data.iter().map(|v| v.find_possible_solutions(None)).sum())
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.len() as u64)
    }
}
