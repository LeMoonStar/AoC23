use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Ash,
    Rock,
}

// The first number represents the position BEFORE which the reflection axis is located.
// The second number is the width of the reflection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reflection {
    OnXAxis(usize, usize),
    OnYAxis(usize, usize),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Invalid input."),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pattern {
    tiles: Vec<Vec<Tile>>,
    dimensions: (usize, usize),
}

impl Pattern {
    #[cfg(debug_assertions)]
    fn print(&self, marker: (Option<usize>, Option<usize>)) {
        use crate::{vprint, vprintln};
        vprintln!();
        vprintln!("{:?}", marker);
        // Print x-marker
        if let Some(marker_x) = marker.0 {
            if marker.1.is_some() {
                vprint!(" ");
            }

            for x in 0..self.dimensions.0 {
                if x == marker_x {
                    vprint!("v");
                } else {
                    vprint!(" ");
                }
            }

            vprintln!();
        }

        for y in 0..self.dimensions.1 {
            if let Some(marker_y) = marker.1 {
                if y == marker_y {
                    vprint!(">");
                } else {
                    vprint!(" ");
                }
            }

            for x in 0..self.dimensions.0 {
                vprint!(
                    "{}",
                    match self.tiles[y][x] {
                        Tile::Ash => '.',
                        Tile::Rock => '#',
                    }
                )
            }
            vprintln!();
        }
    }

    fn get_reflection_line(&self) -> Option<Reflection> {
        dprintln!("{:?}", self);

        // check reflection on x-axis
        for reflection_x in 1..self.dimensions.0 {
            let mut is_reflection = true;
            let width = reflection_x.min(self.dimensions.0 - reflection_x);
            // Width must not be 0.
            if width == 0 {
                continue;
            }

            dprintln!(
                "Checking for x-Axis reflection at x={}; width={}",
                reflection_x,
                width
            );

            for y in 0..self.dimensions.1 {
                for offset in 0..width {
                    if reflection_x + offset >= self.dimensions.0 {
                        continue;
                    }
                    dprintln!(
                        "  Comparing {:?} ({}, {}) - {:?} ({}, {})",
                        self.tiles[y][reflection_x - offset - 1],
                        reflection_x - offset - 1,
                        y,
                        self.tiles[y][reflection_x + offset],
                        reflection_x + offset,
                        y,
                    );
                    if self.tiles[y][reflection_x - offset - 1]
                        != self.tiles[y][reflection_x + offset]
                    {
                        dprintln!("    Not a reflection.");
                        is_reflection = false;
                        break;
                    }
                }
                if !is_reflection {
                    break;
                }
            }

            if is_reflection {
                dprintln!("{:?}", Reflection::OnXAxis(reflection_x, width));

                #[cfg(debug_assertions)]
                self.print((Some(reflection_x), None));

                return Some(Reflection::OnXAxis(reflection_x, width));
            }
        }

        // check reflection on y-axis
        for reflection_y in 0..self.dimensions.1 {
            let mut is_reflection = true;

            let width = reflection_y.min(self.dimensions.1 - reflection_y);
            // width must not be 0
            if width == 0 {
                continue;
            }

            dprintln!(
                "Checking for y-Axis reflection at y={}; width={}",
                reflection_y,
                width
            );

            for x in 0..self.dimensions.0 {
                for offset in 0..width {
                    if reflection_y + offset >= self.dimensions.1 {
                        continue;
                    }
                    dprintln!(
                        "  Comparing {:?} ({}, {}) - {:?} ({}, {})",
                        self.tiles[reflection_y - offset - 1][x],
                        x,
                        reflection_y - offset - 1,
                        self.tiles[reflection_y + offset][x],
                        x,
                        reflection_y + offset
                    );
                    if self.tiles[reflection_y - offset - 1][x]
                        != self.tiles[reflection_y + offset][x]
                    {
                        dprintln!("    Not a reflection.");
                        is_reflection = false;
                        break;
                    }
                }
                if !is_reflection {
                    break;
                }
            }

            if is_reflection {
                dprintln!("{:?}", Reflection::OnYAxis(reflection_y, width));

                #[cfg(debug_assertions)]
                self.print((None, Some(reflection_y)));

                return Some(Reflection::OnYAxis(reflection_y, width));
            }
        }

        panic!("Pattern without Reflection.");
        //None
    }

    // This adaptation literally took about 3 minutes at most.
    // Would've been an amazing delta time,
    // if only I didn't do something else in-between for... SEVEN! hours O.o
    fn get_corrected_reflection_line(&self) -> Option<Reflection> {
        dprintln!("{:?}", self);

        // check reflection on x-axis
        for reflection_x in 1..self.dimensions.0 {
            let mut errors = 0;
            let width = reflection_x.min(self.dimensions.0 - reflection_x);
            // Width must not be 0.
            if width == 0 {
                continue;
            }

            dprintln!(
                "Checking for x-Axis reflection at x={}; width={}",
                reflection_x,
                width
            );

            for y in 0..self.dimensions.1 {
                for offset in 0..width {
                    if reflection_x + offset >= self.dimensions.0 {
                        continue;
                    }
                    dprintln!(
                        "  Comparing {:?} ({}, {}) - {:?} ({}, {})",
                        self.tiles[y][reflection_x - offset - 1],
                        reflection_x - offset - 1,
                        y,
                        self.tiles[y][reflection_x + offset],
                        reflection_x + offset,
                        y,
                    );
                    if self.tiles[y][reflection_x - offset - 1]
                        != self.tiles[y][reflection_x + offset]
                    {
                        dprintln!("    Not a reflection.");
                        errors += 1;
                    }
                }
            }

            if errors == 1 {
                dprintln!("{:?}", Reflection::OnXAxis(reflection_x, width));

                #[cfg(debug_assertions)]
                self.print((Some(reflection_x), None));

                return Some(Reflection::OnXAxis(reflection_x, width));
            }
        }

        // check reflection on y-axis
        for reflection_y in 0..self.dimensions.1 {
            let mut errors = 0;

            let width = reflection_y.min(self.dimensions.1 - reflection_y);
            // width must not be 0
            if width == 0 {
                continue;
            }

            dprintln!(
                "Checking for y-Axis reflection at y={}; width={}",
                reflection_y,
                width
            );

            for x in 0..self.dimensions.0 {
                for offset in 0..width {
                    if reflection_y + offset >= self.dimensions.1 {
                        continue;
                    }
                    dprintln!(
                        "  Comparing {:?} ({}, {}) - {:?} ({}, {})",
                        self.tiles[reflection_y - offset - 1][x],
                        x,
                        reflection_y - offset - 1,
                        self.tiles[reflection_y + offset][x],
                        x,
                        reflection_y + offset
                    );
                    if self.tiles[reflection_y - offset - 1][x]
                        != self.tiles[reflection_y + offset][x]
                    {
                        dprintln!("    Not a reflection.");
                        errors += 1;
                        break;
                    }
                }
            }

            if errors == 1 {
                dprintln!("{:?}", Reflection::OnYAxis(reflection_y, width));

                #[cfg(debug_assertions)]
                self.print((None, Some(reflection_y)));

                return Some(Reflection::OnYAxis(reflection_y, width));
            }
        }

        panic!("Pattern without Reflection.");
        //None
    }
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        let tiles: Vec<Vec<Tile>> = value
            .lines()
            .map(|v| v.chars().map(|v| v.into()).collect())
            .collect();

        Self {
            dimensions: (tiles[0].len(), tiles.len()),
            tiles,
        }
    }
}

type Data = Vec<Pattern>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test13.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(405), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.split("\n\n").map(|v| v.into()).collect())
    }

    fn one(&self, data: &mut Data) -> Answer {
        dprintln!("{:?}", data);

        Answer::Number(
            data.iter()
                .map(|p| match p.get_reflection_line() {
                    Some(Reflection::OnXAxis(x, _)) => x,
                    Some(Reflection::OnYAxis(y, _)) => 100 * y,
                    None => panic!(),
                })
                .sum::<usize>() as u64,
        )
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(
            data.iter()
                .map(|p| match p.get_corrected_reflection_line() {
                    Some(Reflection::OnXAxis(x, _)) => x,
                    Some(Reflection::OnYAxis(y, _)) => 100 * y,
                    None => panic!(),
                })
                .sum::<usize>() as u64,
        )
    }
}
