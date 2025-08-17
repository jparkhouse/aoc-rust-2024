pub fn match_numeric(ch: char) -> Option<u64> {
    let num: u64 = match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => return None,
    };
    Some(num)
}

pub fn parse_number_from_str(target: &str) -> Result<u64, String> {
    let chars: Vec<char> = target.chars().collect();
    let chars_max_ind = chars.len() - 1;
    let nums = chars
        .into_iter()
        .enumerate()
        .map(|(ind, ch)| match match_numeric(ch) {
            Some(num) => Ok(num * 10u64.pow((chars_max_ind - ind).try_into().unwrap())),
            None => Err("Invalid numeric char in target".to_string()),
        })
        .collect::<Result<Vec<u64>, String>>()?;
    Ok(nums.into_iter().sum())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardinalDirection {
    Up,
    Down,
    Left,
    Right,
}

impl CardinalDirection {
    pub const ALL: [CardinalDirection; 4] = {
        use CardinalDirection::*;
        [Up, Down, Left, Right]
    };

    pub fn turn_clockwise(&self) -> Self {
        use CardinalDirection::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn turn_anti_clockwise(&self) -> Self {
        use CardinalDirection::*;
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    pub fn flip(&self) -> Self {
        use CardinalDirection::*;
        match self {
            Up => Down,
            Left => Right,
            Down => Up,
            Right => Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GridBounds {
    pub max_row: usize,
    pub max_col: usize,
}

impl GridBounds {
    pub fn from_input(input: &str) -> GridBounds {
        let lines: Vec<&str> = input.lines().collect();
        let max_col = lines[0].len();
        let max_row = lines.len();
        GridBounds { max_row, max_col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardinalCoord<'a> {
    row: usize,
    col: usize,
    grid_bounds: &'a GridBounds,
}

impl<'a> CardinalShift for CardinalCoord<'a> {
    fn shift_left(self) -> Option<Self> {
        if self.col > 0 {
            Some(Self {
                row: self.row,
                col: self.col - 1,
                grid_bounds: self.grid_bounds,
            })
        } else {
            None
        }
    }

    fn shift_right(self) -> Option<Self> {
        if self.col < self.grid_bounds.max_col - 1 {
            Some(Self {
                row: self.row,
                col: self.col + 1,
                grid_bounds: self.grid_bounds,
            })
        } else {
            None
        }
    }

    fn shift_up(self) -> Option<Self> {
        if self.row > 0 {
            Some(Self {
                row: self.row - 1,
                col: self.col,
                grid_bounds: self.grid_bounds,
            })
        } else {
            None
        }
    }

    fn shift_down(self) -> Option<Self> {
        if self.row < self.grid_bounds.max_row - 1 {
            Some(Self {
                row: self.row + 1,
                col: self.col,
                grid_bounds: self.grid_bounds,
            })
        } else {
            None
        }
    }
}

impl<'a> RawIndex<'a> for CardinalCoord<'a> {
    fn to_raw_ind(&self) -> usize {
        let row_len = self.grid_bounds.max_col;
        self.row * row_len + self.col
    }

    fn from_raw_ind(raw_index: usize, bounds: &'a GridBounds) -> Option<Self> {
        // check if out of bounds
        if raw_index >= bounds.max_col * bounds.max_row {
            return None;
        }
        // otherwise return the coord
        let row_len = bounds.max_col;
        Some(Self {
            row: raw_index / row_len,
            col: raw_index % row_len,
            grid_bounds: bounds,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_raw_ind() {
        static GRID_BOUNDS: GridBounds = GridBounds {
            max_row: 5,
            max_col: 4,
        };
        let coord = CardinalCoord::from_raw_ind(6, &GRID_BOUNDS).unwrap();
        assert_eq!(coord.row, 1);
        assert_eq!(coord.col, 2);
    }

    #[test]
    fn test_to_raw_ind() {
        static GRID_BOUNDS: GridBounds = GridBounds {
            max_row: 5,
            max_col: 4,
        };
        let coord = CardinalCoord {
            row: 1,
            col: 2,
            grid_bounds: &GRID_BOUNDS,
        };
        assert_eq!(coord.to_raw_ind(), 6);
    }

    #[test]
    fn test_shift_left() {
        static GRID_BOUNDS: GridBounds = GridBounds {
            max_row: 5,
            max_col: 5,
        };
        let coord = CardinalCoord {
            row: 2,
            col: 2,
            grid_bounds: &GRID_BOUNDS,
        };
        let shifted = coord.shift(CardinalDirection::Left);
        assert_eq!(
            shifted,
            Some(CardinalCoord {
                row: 2,
                col: 1,
                grid_bounds: &GRID_BOUNDS
            })
        );
    }

    #[test]
    fn test_shift_right() {
        static GRID_BOUNDS: GridBounds = GridBounds {
            max_row: 5,
            max_col: 5,
        };
        let coord = CardinalCoord {
            row: 2,
            col: 2,
            grid_bounds: &GRID_BOUNDS,
        };
        let shifted = coord.shift(CardinalDirection::Right);
        assert_eq!(
            shifted,
            Some(CardinalCoord {
                row: 2,
                col: 3,
                grid_bounds: &GRID_BOUNDS
            })
        );
    }

    #[test]
    fn test_shift_up() {
        static GRID_BOUNDS: GridBounds = GridBounds {
            max_row: 5,
            max_col: 5,
        };
        let coord = CardinalCoord {
            row: 2,
            col: 2,
            grid_bounds: &GRID_BOUNDS,
        };
        let shifted = coord.shift(CardinalDirection::Up);
        assert_eq!(
            shifted,
            Some(CardinalCoord {
                row: 1,
                col: 2,
                grid_bounds: &GRID_BOUNDS
            })
        );
    }

    #[test]
    fn test_shift_down() {
        static GRID_BOUNDS: GridBounds = GridBounds {
            max_row: 5,
            max_col: 5,
        };
        let coord = CardinalCoord {
            row: 2,
            col: 2,
            grid_bounds: &GRID_BOUNDS,
        };
        let shifted = coord.shift(CardinalDirection::Down);
        assert_eq!(
            shifted,
            Some(CardinalCoord {
                row: 3,
                col: 2,
                grid_bounds: &GRID_BOUNDS
            })
        );
    }

    #[test]
    fn test_shift_left_out_of_bounds() {
        static GRID_BOUNDS: GridBounds = GridBounds {
            max_row: 5,
            max_col: 5,
        };
        let coord = CardinalCoord {
            row: 2,
            col: 0,
            grid_bounds: &GRID_BOUNDS,
        };
        let shifted = coord.shift(CardinalDirection::Left);
        assert_eq!(shifted, None);
    }

    #[test]
    fn test_shift_right_out_of_bounds() {
        static GRID_BOUNDS: GridBounds = GridBounds {
            max_row: 5,
            max_col: 5,
        };
        let coord = CardinalCoord {
            row: 2,
            col: 4,
            grid_bounds: &GRID_BOUNDS,
        };
        let shifted = coord.shift(CardinalDirection::Right);
        assert_eq!(shifted, None);
    }

    #[test]
    fn test_shift_up_out_of_bounds() {
        static GRID_BOUNDS: GridBounds = GridBounds {
            max_row: 5,
            max_col: 5,
        };
        let coord = CardinalCoord {
            row: 0,
            col: 2,
            grid_bounds: &GRID_BOUNDS,
        };
        let shifted = coord.shift(CardinalDirection::Up);
        assert_eq!(shifted, None);
    }

    #[test]
    fn test_shift_down_out_of_bounds() {
        static GRID_BOUNDS: GridBounds = GridBounds {
            max_row: 5,
            max_col: 5,
        };
        let coord = CardinalCoord {
            row: 4,
            col: 2,
            grid_bounds: &GRID_BOUNDS,
        };
        let shifted = coord.shift(CardinalDirection::Down);
        assert_eq!(shifted, None);
    }
}

pub trait RawIndex<'a>
where
    Self: Sized,
{
    fn to_raw_ind(&self) -> usize;
    fn from_raw_ind(raw_index: usize, bounds: &'a GridBounds) -> Option<Self>;
}

pub trait CardinalShift
where
    Self: Sized,
{
    fn shift(self, card_dir: CardinalDirection) -> Option<Self> {
        use CardinalDirection::*;
        match card_dir {
            Up => self.shift_up(),
            Down => self.shift_down(),
            Left => self.shift_left(),
            Right => self.shift_right(),
        }
    }

    fn shift_up(self) -> Option<Self>;

    fn shift_down(self) -> Option<Self>;

    fn shift_left(self) -> Option<Self>;

    fn shift_right(self) -> Option<Self>;
}

use std::marker::PhantomData;

pub use grid::Grid;
mod grid {
    use std::marker::PhantomData;

    use super::{
        CardinalDirection, CardinalNeighbors, CardinalShift, GridBounds, GridPoint, RawIndex,
    };

    /// A generic grid component for solving AoC grid problems. The vec is laid out in order, under the assumption that
    /// there is a fixed MxN grid size.
    pub struct Grid<'a, T> {
        pub contents: Vec<T>,
        pub grid_bounds: &'a GridBounds,
    }

    impl<'a, T: Copy> Grid<'a, T> {
        pub fn new(contents: Vec<T>, grid_bounds: &'a GridBounds) -> Self {
            Self {
                contents,
                grid_bounds,
            }
        }

        pub fn get_from_raw_ind(&self, raw_ind: usize) -> Option<&T> {
            self.contents.get(raw_ind)
        }

        pub fn get_from_coord<C: RawIndex<'a>>(&self, coord: C) -> Option<&T> {
            let raw_ind = coord.to_raw_ind();
            self.get_from_raw_ind(raw_ind)
        }

        pub fn set_at_coord<C: RawIndex<'a>>(&mut self, coord: C, val: T) {
            let raw_ind = coord.to_raw_ind();
            self.contents[raw_ind] = val;
        }

        pub fn get_map_neighbors_from_coord<C>(
            &self,
            coord: C,
        ) -> CardinalNeighbors<Option<(C, &T)>>
        where
            C: CardinalShift + RawIndex<'a> + Copy,
        {
            CardinalDirection::ALL
                .into_iter()
                .map(|card_dir| coord.shift(card_dir))
                .map(|new_coord| {
                    new_coord.map(|c| (c, self.get_from_coord(c).expect("must be valid coord")))
                })
                .collect()
        }

        /// Returns an iterator over the contents of the grid
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.contents.iter()
        }

        pub fn coord_iter<C: RawIndex<'a> + Copy>(&self) -> impl Iterator<Item = C> {
            (0..self.contents.len())
                .map(move |ind| C::from_raw_ind(ind, self.grid_bounds).expect("valid coord"))
        }

        /// Returns an iterator over the grid points, which are tuples of (coord, point)
        pub fn grid_point_iter<C: RawIndex<'a> + Copy>(
            &self,
        ) -> impl Iterator<Item = GridPoint<'a, C, T>> {
            self.contents.iter().enumerate().map(move |(ind, &point)| {
                let loc = C::from_raw_ind(ind, self.grid_bounds).expect("valid coord");
                GridPoint {
                    loc,
                    point,
                    _marker: PhantomData,
                }
            })
        }

        pub fn get_grid_point_from_coord<C: RawIndex<'a> + Copy>(
            &self,
            coord: C,
        ) -> Option<GridPoint<'a, C, T>> {
            let point = self.get_from_coord(coord)?;
            Some(GridPoint {
                loc: coord,
                point: *point,
                _marker: PhantomData,
            })
        }
    }
}

pub struct CardinalNeighbors<T>([T; 4]);

impl<T> CardinalNeighbors<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

impl<A: Copy + Default> FromIterator<A> for CardinalNeighbors<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut inner: [A; 4] = [A::default(); 4];
        for (ind, element) in (0..4).zip(iter) {
            inner[ind] = element;
        }
        Self(inner)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPoint<'a, C: RawIndex<'a> + Copy, T: Copy> {
    pub loc: C,
    pub point: T,
    _marker: PhantomData<&'a ()>,
}

impl<'a, C: RawIndex<'a> + Copy, T: Copy> GridPoint<'a, C, T> {
    pub fn from(loc: C, point: T) -> Self {
        GridPoint {
            loc,
            point,
            _marker: PhantomData,
        }
    }
}
