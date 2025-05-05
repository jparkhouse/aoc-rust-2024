advent_of_code::solution!(10);

use std::collections::HashSet;

use crate::coord::Coord;

const ALL_HEIGHTS: [Height; 10] = {
    use Height::*;
    [
        Zero,
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine
    ]
};

pub fn part_one(input: &str) -> Option<u64> {
    let bounds = get_map_bounds_from_input(input);
    let map = parse_input_to_map(input, &bounds);

    let trail_heads = map.find_trail_heads();
    let mut all_paths: Option<Vec<HashSet<MapPoint>>> = None;
    // since we start at Zero, we need to skip 1 and start our search from One
    for height in ALL_HEIGHTS.iter().skip(1) {
        match all_paths {
            // this will be our first run, so we need to work from the trail_heads vec and initialise all_paths
            None => {
                all_paths = Some(
                    // for each trail head
                    trail_heads.iter()
                        // we must start a new HashSet of valid paths (since we want to dedupe locations)
                        .map(|trail_head| {
                            find_valid_paths(
                                &map,
                                height,
                                // we must clone the trail_head because rust is silly
                                // and worries we might come back to it later
                                trail_head.clone()
                            )
                            .into_iter()
                            .collect()
                        })
                        .collect()
                )
            }
            // otherwise, we already have some paths, and we need to check for the next height
            Some(paths) => {
                all_paths = Some(
                    paths.into_iter()
                        // for each trail head's possible paths
                        .map(|trail_head_paths| {
                            // check each point
                            trail_head_paths.into_iter()
                                // and flatten all possible paths back into the trail head's HashSet, deduping locations
                                .flat_map(|mp| {
                                    find_valid_paths(&map, height, mp)
                                })
                                .collect()
                        })
                        .collect()
                    )
                }
            }

        if let Some(paths) = all_paths {
            // filter out any trail heads that did not survive the last round
            all_paths = Some(paths.into_iter().filter(|set| !set.is_empty()).collect());
        }
    }
    
    let scores: Vec<u64> = all_paths.expect("must be initialised by now")
        .into_iter()
        // a trail head's score is equal to the number of Nines it can reach
        .map(|set| set.len() as u64)
        .collect();

    // our final answer is the sum of the scores of all trail heads
    Some(scores.into_iter().sum())
}

fn find_valid_paths<'a>(map: &'a Map, target_height: &Height, mp: MapPoint<'a>) -> Vec<MapPoint<'a>> {
    // for each point from the original trail head, we need to get its neighbours
    let possible_paths = map.get_neighbours(mp.loc);
    // maximum possible paths are 3 (since we must have come from one of them)
    // unless we are starting at a trail head
    let mut output = Vec::with_capacity(4);
    for path in [possible_paths.up, possible_paths.down, possible_paths.left, possible_paths.right] {
        // if it is a valid path, i.e. does not leave the grid
        if let Some(new_mp) = path {
            // and it is exactly one step up in height (from the original iterator)
            if new_mp.height == target_height {
                // add it to the output
                output.push(new_mp);
            }
        }
    }
    // this will then be unpacked for the next run, so that even if there are more than one paths away
    // from a point, we track all of them
    // dead paths are just ignored
    output
}

pub fn part_two(input: &str) -> Option<u64> {
    let bounds = get_map_bounds_from_input(input);
    let map = parse_input_to_map(input, &bounds);

    let trail_heads = map.find_trail_heads();
    let mut all_paths: Option<Vec<Vec<MapPoint>>> = None;
    // since we start at Zero, we need to skip 1 and start our search from One
    for height in ALL_HEIGHTS.iter().skip(1) {
        match all_paths {
            // this will be our first run, so we need to work from the trail_heads vec and initialise all_paths
            None => {
                all_paths = Some(
                    // for each trail head
                    trail_heads.iter()
                        // we must start a new Vec of valid paths
                        .map(|trail_head| {
                            find_valid_paths(
                                &map,
                                height,
                                // we must clone the trail_head because rust is silly
                                // and worries we might come back to it later
                                trail_head.clone()
                            )
                        })
                        .collect()
                )
            }
            // otherwise, we already have some paths, and we need to check for the next height
            Some(paths) => {
                all_paths = Some(
                    paths.into_iter()
                        // for each trail head's possible paths
                        .map(|trail_head_paths| {
                            // check each point
                            trail_head_paths.into_iter()
                                // and flatten all possible paths back into the trail head's vec
                                .flat_map(|mp| {
                                    find_valid_paths(&map, height, mp)
                                })
                                .collect()
                        })
                        .collect()
                    )
                }
            }

        if let Some(paths) = all_paths {
            // filter out any trail heads that did not survive the last round
            all_paths = Some(paths.into_iter().filter(|vec| !vec.is_empty()).collect());
        }
    }
    
    let scores: Vec<u64> = all_paths.expect("must be initialised by now")
        .into_iter()
        // a trail head's score is equal to the number of Nines it can reach
        .map(|vec| vec.len() as u64)
        .collect();

    // our final answer is the sum of the scores of all trail heads
    Some(scores.into_iter().sum())
}

fn get_map_bounds_from_input(input: &str) -> GridBounds {
    let lines: Vec<&str> = input.lines().collect();
    let max_col = lines[0].len();
    let max_row = lines.len();
    GridBounds { max_row, max_col }
}

fn parse_input_to_map<'a>(input: &str, grid_bounds: &'a GridBounds) -> Map<'a> {
    let grid: Vec<Height> = input.lines()
        .flat_map(|line| line.chars().map(|ch| char_to_height(ch)))
        .collect();
    Map { grid, grid_bounds }
} 

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Height {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

fn char_to_height(ch: char) -> Height {
    use Height::*;
    match ch {
        '0' => Zero,
        '1' => One,
        '2' => Two,
        '3' => Three,
        '4' => Four,
        '5' => Five,
        '6' => Six,
        '7' => Seven,
        '8' => Eight,
        '9' => Nine,
        _ => panic!("invalid input"),
    }
}

struct Map<'a> {
    grid: Vec<Height>,
    grid_bounds: &'a GridBounds
}

impl<'a> Map<'a> {

    /// Takes in a coordinate and returns a MapNeighbours struct containing
    /// optional `MapPoint`s, which can be used to check that point's height.
    /// Here, None represents leaving the grid's bounds, so top left corner
    /// would return `Some(...)` for `self.right` and `self.down`, and
    /// `None` otherwise
    pub fn get_neighbours(&'a self, coord: Coord<'a>) -> MapNeighbours<'a> {
        use Direction::*;
        let coords_to_check = [
            coord.shift(Up),
            coord.shift(Down),
            coord.shift(Left),
            coord.shift(Right),
        ];

        MapNeighbours {
            up: self.opt_coord_to_opt_map_point(coords_to_check[0]),
            down: self.opt_coord_to_opt_map_point(coords_to_check[1]),
            left: self.opt_coord_to_opt_map_point(coords_to_check[2]),
            right: self.opt_coord_to_opt_map_point(coords_to_check[3]),
        }
    }

    /// Filters the grid to return a `Vec` of `MapPoint`s for all locations in the grid
    /// where the height is `Zero`, defined as a trail head in the problem
    pub fn find_trail_heads(&'a self) -> Vec<MapPoint<'a>> {
        self.grid.iter().enumerate().filter(|(_, height)| {
            **height == Height::Zero
        }).map(|(ind, height)| {
            let loc = Coord::from_raw_ind(ind, self.grid_bounds);
            MapPoint {
                loc, height
            }
        }).collect()
    }
    
    fn opt_coord_to_opt_map_point(&'a self, x: Option<Coord<'a>>) -> Option<MapPoint<'a>> {
        x.map(|loc| self.coord_to_map_point(loc))
    }

    /// Uses the map to turn a `Coord` into a `MapPoint` by enriching it with a reference
    /// to the height at that location. Assumes that the `Coord` is within the bounds of
    /// the map, otherwise panics
    pub fn coord_to_map_point(&'a self, loc: Coord<'a>) -> MapPoint<'a> {
        let height = self.grid.get(loc.to_raw_ind())
            .expect("coord should not leave bounds of grid");
        MapPoint { loc, height }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MapPoint<'a> {
    loc: Coord<'a>,
    height: &'a Height
}

struct MapNeighbours<'a> {
    up: Option<MapPoint<'a>>,
    down: Option<MapPoint<'a>>,
    left: Option<MapPoint<'a>>,
    right: Option<MapPoint<'a>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GridBounds {
    pub max_row: usize,
    pub max_col: usize,
}

mod coord {
    use crate::{Direction, GridBounds};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Coord<'a> {
        row: usize,
        col: usize,
        grid_bounds: &'a GridBounds
    }

    impl<'a> Coord<'a> {
        fn shift_left(self) -> Option<Self> {
            if self.col > 0 {
                Some(Self {
                    row: self.row,
                    col: self.col - 1,
                    grid_bounds: self.grid_bounds
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
                    grid_bounds: self.grid_bounds
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
                    grid_bounds: self.grid_bounds
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
                    grid_bounds: self.grid_bounds
                })
            } else {
                None
            }
        }

        /// Shifts the coordinate 1 step in the given direction.
        /// Returns an option where None represents leaving the bounds of the grid.
        pub fn shift(self, dir: Direction) -> Option<Self> {
            use Direction::*;
            match dir {
                Left => self.shift_left(),
                Right => self.shift_right(),
                Up => self.shift_up(),
                Down => self.shift_down(),
            }
        }

        /// Uses grid_bounds to convert a usize into a coord
        pub fn from_raw_ind(raw_ind: usize, grid_bounds: &'a GridBounds) -> Self {
            let row_len = grid_bounds.max_col;
            Self {
                row: raw_ind / row_len,
                col: raw_ind % row_len,
                grid_bounds
            }
        }

        /// Returns coord to usize for indexing
        pub fn to_raw_ind(&self) -> usize {
            let row_len = self.grid_bounds.max_col;
            self.row * row_len + self.col
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_from_raw_ind() {
            let grid_bounds = GridBounds { max_row: 5, max_col: 4 };
            let coord = Coord::from_raw_ind(6, &grid_bounds);
            assert_eq!(coord.row, 1);
            assert_eq!(coord.col, 2);
        }

        #[test]
        fn test_to_raw_ind() {
            let grid_bounds = GridBounds { max_row: 5, max_col: 4 };
            let coord = Coord { row: 1, col: 2, grid_bounds: &grid_bounds };
            assert_eq!(coord.to_raw_ind(), 6);
        }

        #[test]
        fn test_shift_left() {
            let grid_bounds = GridBounds { max_row: 5, max_col: 5 };
            let coord = Coord { row: 2, col: 2, grid_bounds: &grid_bounds };
            let shifted = coord.shift(Direction::Left);
            assert_eq!(shifted, Some(Coord { row: 2, col: 1, grid_bounds: &grid_bounds }));
        }
    
        #[test]
        fn test_shift_right() {
            let grid_bounds = GridBounds { max_row: 5, max_col: 5 };
            let coord = Coord { row: 2, col: 2, grid_bounds: &grid_bounds };
            let shifted = coord.shift(Direction::Right);
            assert_eq!(shifted, Some(Coord { row: 2, col: 3, grid_bounds: &grid_bounds }));
        }
    
        #[test]
        fn test_shift_up() {
            let grid_bounds = GridBounds { max_row: 5, max_col: 5 };
            let coord = Coord { row: 2, col: 2, grid_bounds: &grid_bounds };
            let shifted = coord.shift(Direction::Up);
            assert_eq!(shifted, Some(Coord { row: 1, col: 2, grid_bounds: &grid_bounds }));
        }
    
        #[test]
        fn test_shift_down() {
            let grid_bounds = GridBounds { max_row: 5, max_col: 5 };
            let coord = Coord { row: 2, col: 2, grid_bounds: &grid_bounds };
            let shifted = coord.shift(Direction::Down);
            assert_eq!(shifted, Some(Coord { row: 3, col: 2, grid_bounds: &grid_bounds }));
        }
    
        #[test]
        fn test_shift_left_out_of_bounds() {
            let grid_bounds = GridBounds { max_row: 5, max_col: 5 };
            let coord = Coord { row: 2, col: 0, grid_bounds: &grid_bounds };
            let shifted = coord.shift(Direction::Left);
            assert_eq!(shifted, None);
        }
    
        #[test]
        fn test_shift_right_out_of_bounds() {
            let grid_bounds = GridBounds { max_row: 5, max_col: 5 };
            let coord = Coord { row: 2, col: 4, grid_bounds: &grid_bounds };
            let shifted = coord.shift(Direction::Right);
            assert_eq!(shifted, None);
        }
    
        #[test]
        fn test_shift_up_out_of_bounds() {
            let grid_bounds = GridBounds { max_row: 5, max_col: 5 };
            let coord = Coord { row: 0, col: 2, grid_bounds: &grid_bounds };
            let shifted = coord.shift(Direction::Up);
            assert_eq!(shifted, None);
        }
    
        #[test]
        fn test_shift_down_out_of_bounds() {
            let grid_bounds = GridBounds { max_row: 5, max_col: 5 };
            let coord = Coord { row: 4, col: 2, grid_bounds: &grid_bounds };
            let shifted = coord.shift(Direction::Down);
            assert_eq!(shifted, None);
        }
    }
}


enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}