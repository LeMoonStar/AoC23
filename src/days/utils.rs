/// A general purpose struct able to store 2-Dimensional maps of Tiles.
///
/// Can be parsed from a multi-line &str, if the T type implements [`From<char>`](std::convert::From).
#[derive(Debug, Clone, Hash)]
pub struct Map<T> {
    tiles: Vec<Vec<T>>,
    dim: (usize, usize),
}

#[allow(dead_code)]
impl<T> Map<T> {
    /// Returns the dimensions of the map as a tuple `(x: usize, y: usize)`
    pub fn dimensions(&self) -> (usize, usize) {
        self.dim
    }

    /// Gets the tile of a specified position.
    ///
    /// May return None, if the position is outside the dimensions.
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.tiles.get(y)?.get(x)
    }

    /// Sets the tile of a specified position.
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.tiles[y][x] = value;
    }

    /// Get the internally stored tile data.
    pub fn get_raw_tiles(&self) -> &Vec<Vec<T>> {
        &self.tiles
    }

    /// Get the internally stored tile data as mutable reference.
    pub fn get_raw_tiles_mut(&mut self) -> &mut Vec<Vec<T>> {
        &mut self.tiles
    }
}

impl<T> From<Vec<Vec<T>>> for Map<T> {
    /// Creates a Map from a `Vec<Vec<T>>`.
    /// The dimensions are derived from the length of the y-Axis and first x line.
    fn from(value: Vec<Vec<T>>) -> Self {
        Self {
            dim: (value[0].len(), value.len()),
            tiles: value,
        }
    }
}

impl<T> From<&str> for Map<T>
where
    T: From<char>,
{
    fn from(value: &str) -> Self {
        let tiles: Vec<Vec<T>> = value
            .lines()
            .map(|v| v.chars().map(|v| v.into()).collect())
            .collect();

        Self {
            dim: (tiles[0].len(), tiles.len()),
            tiles,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }

    pub fn walk_pos(
        &self,
        pos: (usize, usize),
        upper_limit: (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Self::North => {
                if pos.1 != 0 {
                    Some((pos.0, pos.1 - 1))
                } else {
                    None
                }
            }
            Self::South => {
                if pos.1 < upper_limit.1 {
                    Some((pos.0, pos.1 + 1))
                } else {
                    None
                }
            }
            Self::West => {
                if pos.0 != 0 {
                    Some((pos.0 - 1, pos.1))
                } else {
                    None
                }
            }
            Self::East => {
                if pos.0 < upper_limit.0 {
                    Some((pos.0 + 1, pos.1))
                } else {
                    None
                }
            }
        }
    }

    pub fn walk_pos_signed(
        &self,
        pos: (isize, isize),
        lower_limit: (isize, isize),
        upper_limit: (isize, isize),
    ) -> Option<(isize, isize)> {
        match self {
            Self::North => {
                if pos.1 > lower_limit.1 {
                    Some((pos.0, pos.1 - 1))
                } else {
                    None
                }
            }
            Self::South => {
                if pos.1 < upper_limit.1 {
                    Some((pos.0, pos.1 + 1))
                } else {
                    None
                }
            }
            Self::West => {
                if pos.0 > lower_limit.0 {
                    Some((pos.0 - 1, pos.1))
                } else {
                    None
                }
            }
            Self::East => {
                if pos.0 < upper_limit.0 {
                    Some((pos.0 + 1, pos.1))
                } else {
                    None
                }
            }
        }
    }
}
