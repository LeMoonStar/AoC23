use std::collections::HashMap;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 8;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Node {
    name: [char; 3],
    left: [char; 3],
    right: [char; 3],
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let (name_str, connections_str) = value.split_once(" = ").unwrap();
        let (conn_left, conn_right) = connections_str.split_once(", ").unwrap();

        let mut name_chars = name_str.chars();
        let mut left_chars = conn_left.chars().skip(1);
        let mut right_chars = conn_right.chars();

        Self {
            name: [
                name_chars.next().unwrap(),
                name_chars.next().unwrap(),
                name_chars.next().unwrap(),
            ],
            left: [
                left_chars.next().unwrap(),
                left_chars.next().unwrap(),
                left_chars.next().unwrap(),
            ],
            right: [
                right_chars.next().unwrap(),
                right_chars.next().unwrap(),
                right_chars.next().unwrap(),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<[char; 3], Node>,
}

impl Map {
    fn follow_path_to_goal(&self, mut node: [char; 3], goal: [Option<char>; 3]) -> u64 {
        let mut steps = 0;

        for dir in self.directions.iter().cycle() {
            node = match dir {
                Direction::Left => self.nodes.get(&node).unwrap().left,
                Direction::Right => self.nodes.get(&node).unwrap().right,
            };

            steps += 1;

            if match goal {
                [None, None, Some(c)] => node[2] == c,
                [Some(a), Some(b), Some(c)] => node == [a, b, c],
                _ => unimplemented!("This part should never trip"),
            } {
                break;
            }
        }

        steps
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        Self {
            directions: lines
                .next()
                .unwrap()
                .chars()
                .map(|v| match v {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Invalid Input."),
                })
                .collect(),
            nodes: lines
                .skip(1)
                .map(|v| {
                    let n: Node = v.into();
                    (n.name, n)
                })
                .collect(),
        }
    }
}

type Data = Map;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test08.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(17263), Answer::Number(14631604759649))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.follow_path_to_goal(['A', 'A', 'A'], [Some('Z'), Some('Z'), Some('Z')]))
    }

    fn two(&self, data: &mut Data) -> Answer {
        let start_nodes: Vec<[char; 3]> =
            data.nodes.keys().filter(|v| v[2] == 'A').copied().collect();

        let frequencies: Vec<u64> = start_nodes
            .into_iter()
            .map(|v| data.follow_path_to_goal(v, [None, None, Some('Z')]))
            .collect();

        Answer::Number(frequencies.iter().fold(1, |a, v| Self::lcm(a, *v)))
    }
}

impl Day<CURRENT_DAY> {
    fn lcm(a: u64, b: u64) -> u64 {
        (a / Self::gcd(a, b)) * b
    }

    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            return a;
        }
        Self::gcd(b, a % b)
    }
}
