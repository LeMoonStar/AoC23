use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 11;

#[derive(Debug, Clone)]
pub struct Map {
    stars: Vec<(u64, u64)>,
    dimensions: (u64, u64),
}

impl Map {
    fn star_distance(&self, a: usize, b: usize) -> u64 {
        self.stars[a].0.abs_diff(self.stars[b].0) + self.stars[a].1.abs_diff(self.stars[b].1)
    }

    fn expand(&mut self, factor: u64) {
        let (rows, cols) = self.get_empty_rows_and_cols();
        self.stars = self
            .stars
            .iter()
            .map(|star| {
                (
                    star.0 + cols.iter().filter(|v| **v < star.0).count() as u64 * factor,
                    star.1 + rows.iter().filter(|v| **v < star.1).count() as u64 * factor,
                )
            })
            .collect();
        self.dimensions.1 += rows.len() as u64 * factor;
        self.dimensions.0 += cols.len() as u64 * factor;
    }

    fn get_empty_rows_and_cols(&self) -> (Vec<u64>, Vec<u64>) {
        let (mut rows, mut cols) = (Vec::new(), Vec::new());

        for y in 0..self.dimensions.1 {
            dprintln!("ROW {}", y);
            let mut is_empty = true;
            for x in 0..self.dimensions.0 {
                if self.stars.contains(&(x, y)) {
                    dprintln!("  NOT EMPTY: {:?}", (x, y));
                    is_empty = false;
                    continue;
                }
            }
            if is_empty {
                rows.push(y);
            }
        }

        for x in 0..self.dimensions.0 {
            dprintln!("COL {}", x);
            let mut is_empty = true;
            for y in 0..self.dimensions.1 {
                if self.stars.contains(&(x, y)) {
                    dprintln!("  NOT EMPTY: {:?}", (x, y));
                    is_empty = false;
                    continue;
                }
            }
            if is_empty {
                cols.push(x);
            }
        }

        (rows, cols)
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut dimensions = (0_u64, 0_u64);

        Self {
            stars: value
                .lines()
                .enumerate()
                .inspect(|(y, _)| {
                    if (*y + 1) as u64 > dimensions.1 {
                        dimensions.1 = (*y + 1) as u64
                    }
                })
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .inspect(|(x, _)| {
                            if (*x + 1) as u64 > dimensions.0 {
                                dimensions.0 = (*x + 1) as u64
                            }
                        })
                        .fold(Vec::new(), |mut tmp, (x, c)| {
                            if c == '#' {
                                tmp.push((x as u64, y as u64));
                            }
                            tmp
                        })
                })
                .collect(),
            dimensions,
        }
    }
}

type Data = Map;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test11.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(374), Answer::Number(82000210))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        data.expand(1);
        let mut sum = 0;
        for a in 0..data.stars.len() {
            for b in (a + 1)..data.stars.len() {
                sum += data.star_distance(a, b);
            }
        }

        Answer::Number(sum)
    }

    fn two(&self, data: &mut Data) -> Answer {
        // I have no clue why I need the -1 here...
        // am just too tired today.
        data.expand(1000000 - 1);

        let mut sum = 0;
        for a in 0..data.stars.len() {
            for b in (a + 1)..data.stars.len() {
                sum += data.star_distance(a, b);
            }
        }

        Answer::Number(sum)
    }
}
