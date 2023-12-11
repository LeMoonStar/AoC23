use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 11;

#[derive(Debug, Clone)]
pub struct Map {
    stars: Vec<(usize, usize)>,
    dimensions: (usize, usize),
}

impl Map {
    fn star_distance(&self, a: usize, b: usize) -> usize {
        self.stars[a].0.abs_diff(self.stars[b].0) + self.stars[a].1.abs_diff(self.stars[b].1)
    }

    fn expand(&mut self, factor: usize) {
        let (rows, cols) = self.get_empty_rows_and_cols();
        self.stars = self
            .stars
            .iter()
            .map(|star| {
                (
                    star.0 + cols.iter().filter(|v| **v < star.0).count() * factor,
                    star.1 + rows.iter().filter(|v| **v < star.1).count() * factor,
                )
            })
            .collect();
        self.dimensions.1 += rows.len() * factor;
        self.dimensions.0 += cols.len() * factor;
    }

    fn get_empty_rows_and_cols(&self) -> (Vec<usize>, Vec<usize>) {
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
        let mut dimensions = (0, 0);

        Self {
            stars: value
                .lines()
                .enumerate()
                .inspect(|(y, _)| {
                    if *y + 1 > dimensions.1 {
                        dimensions.1 = *y + 1
                    }
                })
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .inspect(|(x, _)| {
                            if *x + 1 > dimensions.0 {
                                dimensions.0 = *x + 1
                            }
                        })
                        .fold(Vec::new(), |mut tmp, (x, c)| {
                            if c == '#' {
                                tmp.push((x, y));
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

        Answer::Number(sum as u64)
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

        Answer::Number(sum as u64)
    }
}
