use std::collections::HashSet;

use super::{
    utils::{Direction, Map},
    Answer, Day, DayImpl,
};

const CURRENT_DAY: u8 = 16;

#[derive(Debug, Clone)]
pub enum Mirror {
    Vertical,
    Horizontal,
    DiagonalTopRight, // /
    DiagonalTopLeft,  // \
    None,
}

impl Mirror {
    #[cfg(debug_assertions)]
    fn to_char(&self) -> char {
        match self {
            Self::Vertical => '|',
            Self::Horizontal => '-',
            Self::DiagonalTopLeft => '\\',
            Self::DiagonalTopRight => '/',
            Self::None => '.',
        }
    }
}

impl From<char> for Mirror {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            '/' => Self::DiagonalTopRight,
            '\\' => Self::DiagonalTopLeft,
            '.' => Self::None,
            _ => panic!("Invalid Input"),
        }
    }
}

type Data = Map<Mirror>;

impl Data {
    // Yes, this function is necessary, as the energize function only handles fields coming after its starting pos.
    fn start_energize(&self, x: usize, y: usize, dir: Direction) -> HashSet<(usize, usize)> {
        let mut energized: HashSet<(usize, usize, Direction)> = HashSet::new();

        match (self.get(x, y).unwrap(), dir) {
            (Mirror::Horizontal, Direction::North | Direction::South) => {
                self.energize(x, y, Direction::East, &mut energized);
                self.energize(x, y, Direction::West, &mut energized);
            }
            (Mirror::Vertical, Direction::East | Direction::West) => {
                self.energize(x, y, Direction::North, &mut energized);
                self.energize(x, y, Direction::South, &mut energized);
            }
            (Mirror::DiagonalTopLeft, _) => {
                let dir = match dir {
                    Direction::North => Direction::West,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                };
                self.energize(x, y, dir, &mut energized);
            }
            (Mirror::DiagonalTopRight, _) => {
                let dir = match dir {
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::East => Direction::North,
                    Direction::West => Direction::South,
                };
                self.energize(x, y, dir, &mut energized);
            }
            _ => {
                self.energize(x, y, dir, &mut energized);
            }
        }

        energized
            .iter()
            .map(|v| (v.0, v.1))
            .collect::<HashSet<(usize, usize)>>()
    }

    fn energize(
        &self,
        mut x: usize,
        mut y: usize,
        mut dir: Direction,
        energized: &mut HashSet<(usize, usize, Direction)>,
    ) {
        if energized.contains(&(x, y, dir)) {
            return;
        }
        energized.insert((x, y, dir));

        // TODO: This might ignore the very first tile.
        while let Some((new_x, new_y)) = self.move_in_direction(x, y, dir) {
            (x, y) = (new_x, new_y);

            match (self.get(x, y).unwrap(), dir) {
                (Mirror::Horizontal, Direction::North | Direction::South) => {
                    self.energize(x, y, Direction::East, energized);
                    self.energize(x, y, Direction::West, energized);
                    return;
                }
                (Mirror::Vertical, Direction::East | Direction::West) => {
                    self.energize(x, y, Direction::North, energized);
                    self.energize(x, y, Direction::South, energized);
                    return;
                }
                (Mirror::DiagonalTopLeft, _) => {
                    if energized.contains(&(x, y, dir)) {
                        return;
                    }
                    energized.insert((x, y, dir));
                    dir = match dir {
                        Direction::North => Direction::West,
                        Direction::East => Direction::South,
                        Direction::South => Direction::East,
                        Direction::West => Direction::North,
                    }
                }
                (Mirror::DiagonalTopRight, _) => {
                    if energized.contains(&(x, y, dir)) {
                        return;
                    }
                    energized.insert((x, y, dir));
                    dir = match dir {
                        Direction::North => Direction::East,
                        Direction::South => Direction::West,
                        Direction::East => Direction::North,
                        Direction::West => Direction::South,
                    }
                }
                _ => {
                    if energized.contains(&(x, y, dir)) {
                        return;
                    }
                    energized.insert((x, y, dir));
                    continue;
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    fn print(&self, a: &HashSet<(usize, usize)>) {
        use crate::{vprint, vprintln};

        vprintln!();
        for y in 0..self.dimensions().1 {
            for x in 0..self.dimensions().0 {
                vprint!(
                    "{}",
                    if a.contains(&(x, y)) {
                        '#'
                    } else {
                        self.get(x, y).unwrap().to_char()
                    }
                );
            }
            vprintln!();
        }
    }

    fn move_in_direction(&self, x: usize, y: usize, dir: Direction) -> Option<(usize, usize)> {
        let (x, y) = match dir {
            Direction::North => {
                if y == 0 {
                    return None;
                } else {
                    (x, y - 1)
                }
            }
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => {
                if x == 0 {
                    return None;
                } else {
                    (x - 1, y)
                }
            }
        };

        if x >= self.dimensions().0 || y >= self.dimensions().1 {
            None
        } else {
            Some((x, y))
        }
    }
}

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test16.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(46), Answer::Number(51))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        let energized = data.start_energize(0, 0, Direction::East);

        #[cfg(debug_assertions)]
        data.print(&energized);

        Answer::Number(
            energized
                .iter()
                .map(|v| (v.0, v.1))
                .collect::<HashSet<(usize, usize)>>()
                .len() as u64,
        )
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut max = 0;
        for x in 0..data.dimensions().0 {
            max = max.max(data.start_energize(x, 0, Direction::South).len());
            max = max.max(
                data.start_energize(x, data.dimensions().1 - 1, Direction::North)
                    .len(),
            );
        }

        for y in 1..data.dimensions().1 - 1 {
            max = max.max(data.start_energize(0, y, Direction::East).len());
            max = max.max(
                data.start_energize(data.dimensions().0 - 1, y, Direction::West)
                    .len(),
            );
        }
        Answer::Number(max as u64)
    }
}
