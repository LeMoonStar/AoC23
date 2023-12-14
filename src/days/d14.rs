use crate::{dprintln, vprintln};

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 14;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    CubeRock,
    RoundRock,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::CubeRock,
            'O' => Self::RoundRock,
            '.' => Self::Empty,
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
    dimensions: (usize, usize),
}

impl Map {
    fn slide_all(&mut self, dir: Direction, tile_type: Tile) {
        match dir {
            Direction::North => {
                for y in 0..self.dimensions.1 {
                    for x in 0..self.dimensions.0 {
                        if self.tiles[y][x] == tile_type {
                            self.slide_tile(x, y, dir);
                        }
                    }
                }
            }
            Direction::South => {
                for y in (0..self.dimensions.1).rev() {
                    for x in 0..self.dimensions.0 {
                        if self.tiles[y][x] == tile_type {
                            self.slide_tile(x, y, dir);
                        }
                    }
                }
            }

            Direction::East => {
                for x in (0..self.dimensions.0).rev() {
                    for y in 0..self.dimensions.1 {
                        if self.tiles[y][x] == tile_type {
                            self.slide_tile(x, y, dir);
                        }
                    }
                }
            }
            Direction::West => {
                for x in 0..self.dimensions.0 {
                    for y in 0..self.dimensions.1 {
                        if self.tiles[y][x] == tile_type {
                            self.slide_tile(x, y, dir);
                        }
                    }
                }
            }
        }
    }

    fn slide_tile(&mut self, x: usize, y: usize, dir: Direction) {
        let mut curr_x: isize = x as isize;
        let mut curr_y: isize = y as isize;

        dprintln!("x{} y{}", x, y);

        loop {
            let next_x = curr_x
                + match dir {
                    Direction::East => 1,
                    Direction::West => -1,
                    _ => 0,
                };
            let next_y = curr_y
                + match dir {
                    Direction::North => -1,
                    Direction::South => 1,
                    _ => 0,
                };

            dprintln!(
                "  c_x{} c_y{} = dir{:?} => n_x{} n_y{}",
                curr_x,
                curr_y,
                dir,
                next_x,
                next_y
            );

            if next_x.is_negative()
                || next_x as usize >= self.dimensions.0
                || next_y.is_negative()
                || next_y as usize >= self.dimensions.1
                || self.tiles[next_y as usize][next_x as usize] != Tile::Empty
            {
                // Hit a block.
                dprintln!("    HIT A BLOCK!");
                dprintln!("      x{} y{} => x{} y{}", x, y, curr_x, curr_y);
                if curr_x as usize != x || curr_y as usize != y {
                    self.tiles[curr_y as usize][curr_x as usize] = self.tiles[y][x];
                    self.tiles[y][x] = Tile::Empty;
                }

                return;
            }

            (curr_x, curr_y) = (next_x, next_y);
        }
    }

    fn get_load(&self) -> usize {
        let mut load = 0;

        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                if self.tiles[y][x] == Tile::RoundRock {
                    load += self.dimensions.1 - y;
                }
            }
        }

        load
    }

    #[cfg(debug_assertions)]
    fn print(&self) {
        use crate::{vprint, vprintln};

        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                vprint!(
                    "{}",
                    match self.tiles[y][x] {
                        Tile::RoundRock => 'O',
                        Tile::CubeRock => '#',
                        Tile::Empty => '.',
                    }
                )
            }
            vprintln!();
        }
    }
}

impl From<&str> for Map {
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

type Data = Map;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test14.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(136), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        #[cfg(debug_assertions)]
        data.print();

        data.slide_all(Direction::North, Tile::RoundRock);

        vprintln!();

        #[cfg(debug_assertions)]
        data.print();

        #[cfg(debug_assertions)]
        data.print();

        Answer::Number(data.get_load() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0)
    }
}
