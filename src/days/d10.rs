use crate::{dprintln, vprint, vprintln};
use std::collections::HashMap;

use super::{Answer, Day, DayImpl, utils::Direction};

const CURRENT_DAY: u8 = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
    Ground,
    Start,
}

impl Tile {
    pub fn get_connecting_directions(&self) -> Vec<Direction> {
        match self {
            Self::VerticalPipe => vec![Direction::North, Direction::South],
            Self::HorizontalPipe => vec![Direction::East, Direction::West],
            Self::NorthEastPipe => vec![Direction::North, Direction::East],
            Self::NorthWestPipe => vec![Direction::North, Direction::West],
            Self::SouthWestPipe => vec![Direction::South, Direction::West],
            Self::SouthEastPipe => vec![Direction::South, Direction::East],
            Self::Ground => vec![],
            Self::Start => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
        }
    }

    #[cfg(debug_assertions)]
    pub fn get_char(&self) -> char {
        match self {
            Self::VerticalPipe => '┃',
            Self::HorizontalPipe => '━',
            Self::NorthEastPipe => '┗',
            Self::NorthWestPipe => '┛',
            Self::SouthWestPipe => '┓',
            Self::SouthEastPipe => '┏',
            Self::Ground => '.',
            Self::Start => '╋',
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::VerticalPipe,
            '-' => Self::HorizontalPipe,
            'L' => Self::NorthEastPipe,
            'J' => Self::NorthWestPipe,
            '7' => Self::SouthWestPipe,
            'F' => Self::SouthEastPipe,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Invalid input"),
        }
    }
}

// Ain't using super::utils::Map here, as this one is different enough to be its
// own type - otherwise I'd have to implement another type wrapping around
// super::utils::Map to store the starting_pos, which is found during parsing.
#[derive(Debug, Clone)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
    start_pos: (usize, usize),
    dimensions: (usize, usize),
}

impl Map {
    pub fn get_loop(&self, start_pos: (usize, usize)) -> HashMap<(usize, usize), u16> {
        let mut distances = HashMap::new();
        self.walk_loop(start_pos, 0, &mut distances);

        distances
    }

    fn walk_loop(
        &self,
        pos: (usize, usize),
        distance: u16,
        loop_distances: &mut HashMap<(usize, usize), u16>,
    ) {
        if *loop_distances.get(&pos).unwrap_or(&u16::MAX) > distance {
            loop_distances.insert(pos, distance);
            dprintln!(
                "p={:?} -- n = {:?}",
                pos,
                self.get_connected_neighbours(pos)
            );
            for n in self.get_connected_neighbours(pos) {
                self.walk_loop(n, distance + 1, loop_distances);
            }
        }
    }

    fn get_connected_neighbours(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::with_capacity(4);

        dprintln!(
            "  Considered Dirs: {:?}",
            self.tiles[pos.1][pos.0].get_connecting_directions()
        );

        for dir in self.tiles[pos.1][pos.0].get_connecting_directions() {
            dprintln!(
                "    {:?}({:?}) -> {:?}",
                dir,
                pos,
                dir.walk_pos(pos, self.dimensions)
            );
            if let Some(neighbour) = dir.walk_pos(pos, self.dimensions) {
                dprintln!("      {:?}: {:?}", dir, neighbour);
                dprintln!(
                    "        Connects to: {:?}",
                    self.tiles[neighbour.1][neighbour.0].get_connecting_directions()
                );
                dprintln!("        Must contain: {:?}", &dir.opposite());
                if self.tiles[neighbour.1][neighbour.0]
                    .get_connecting_directions()
                    .contains(&dir.opposite())
                {
                    neighbours.push(neighbour);
                }
            }
        }

        neighbours
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut start_pos = (0, 0);
        let tiles: Vec<Vec<Tile>> = value
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let t = Tile::from(c);
                        if t == Tile::Start {
                            start_pos = (x, y)
                        }
                        t
                    })
                    .collect()
            })
            .collect();

        Self {
            dimensions: (tiles[0].len(), tiles.len()),
            tiles,
            start_pos,
        }
    }
}

type Data = Map;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test10.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(80), Answer::Number(10))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.into())
    }

    fn one(&self, data: &mut Data) -> Answer {
        let l = data.get_loop(data.start_pos);

        Answer::Number(*l.values().max().unwrap() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut area = 0;
        let l = data.get_loop(data.start_pos);

        for y in 0..data.dimensions.1 {
            for x in 0..data.dimensions.0 {
                if l.contains_key(&(x, y)) {
                    vprint!("{}", data.tiles[y][x].get_char());
                    continue;
                }

                let mut west_i = 0;
                let mut i: Option<Direction> = None;
                for r_x in 0..x {
                    if l.contains_key(&(r_x, y)) {
                        match (
                            &i,
                            data.tiles[y][r_x]
                                .get_connecting_directions()
                                .contains(&Direction::North),
                            data.tiles[y][r_x]
                                .get_connecting_directions()
                                .contains(&Direction::South),
                        ) {
                            (None, true, true) => west_i += 1,                 // Direct line
                            (None, true, false) => i = Some(Direction::North), // coming from north
                            (None, false, true) => i = Some(Direction::South), // coming from south
                            (Some(Direction::North), true, false) => i = None, // coming from north, going north
                            (Some(Direction::South), false, true) => i = None, // coming from south, going south
                            (Some(Direction::North), false, true) => {
                                west_i += 1;
                                i = None
                            } // coming from north, going south
                            (Some(Direction::South), true, false) => {
                                west_i += 1;
                                i = None
                            } // coming from south, going north
                            (_, false, false) => {}
                            _ => {}
                        }
                    }
                }

                let mut north_i = 0;
                let mut i: Option<Direction> = None;
                for r_y in 0..y {
                    if l.contains_key(&(x, r_y)) {
                        match (
                            &i,
                            data.tiles[r_y][x]
                                .get_connecting_directions()
                                .contains(&Direction::West),
                            data.tiles[r_y][x]
                                .get_connecting_directions()
                                .contains(&Direction::East),
                        ) {
                            (None, true, true) => north_i += 1,               // Direct line
                            (None, true, false) => i = Some(Direction::West), // coming from west
                            (None, false, true) => i = Some(Direction::East), // coming from east
                            (Some(Direction::West), true, false) => i = None, // coming from west, going east
                            (Some(Direction::East), false, true) => i = None, // coming from east, going west
                            (Some(Direction::West), false, true) => {
                                north_i += 1;
                                i = None
                            } // coming from west, going east
                            (Some(Direction::East), true, false) => {
                                north_i += 1;
                                i = None
                            } // coming from east, going west
                            (_, false, false) => {}
                            _ => {}
                        }
                    }
                }

                if west_i % 2 == 1 && north_i % 2 == 1 {
                    vprint!("X");
                    area += 1;
                } else {
                    vprint!("0");
                }
            }
            vprintln!();
        }
        Answer::Number(area)
    }
}
