use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid_info = parse_input(input);
    // use a hashset to store all locations where at least one antinode is found
    // we do not care if this overlaps with an antenna
    // we do not care which signal (char) the antinode comes from
    let mut unique_antinodes: HashSet<Coord> = HashSet::new();
    // work through the different signals
    // since antinodes only form from a pair of antennas of the same signal
    grid_info
        .antennas_by_char
        .drain()
        // for each signal
        .for_each(|(_, antennas)| {
            antennas
                .iter()
                // get all pairs (ignoring order: (A, B) == (B, A), and we don't want to double count)
                .tuple_combinations::<(_, _)>()
                // for each pair
                .for_each(
                    // calculate any in-bounds antinodes and add them to the hashset
                    |(a1, a2)| match calculate_antinode_pair(a1, a2, &grid_info.bounds) {
                        AntinodeCase::TwoNodes(node_1, node_2) => {
                            unique_antinodes.insert(node_1);
                            unique_antinodes.insert(node_2);
                        }
                        AntinodeCase::OneNode(node) => {
                            unique_antinodes.insert(node);
                        }
                        AntinodeCase::None => {}
                    },
                );
        });
    // finally return the number of unique antinodes
    Some(unique_antinodes.len() as u64)
}

fn parse_input(input: &str) -> GridInformation {
    let mut chars_to_coords: HashMap<char, Vec<Coord>> = HashMap::new();
    let bounds = get_grid_bounds(input);
    let iter = input.lines().enumerate().flat_map(|(row_ind, line)| {
        line.chars().enumerate().filter_map(move |(col_ind, ch)| {
            if ch != '.' {
                Some(((row_ind, col_ind), ch))
            } else {
                None
            }
        })
    });
    iter.for_each(|((row_ind, col_ind), ch)| {
        chars_to_coords.entry(ch).or_default().push(Coord {
            row: row_ind,
            col: col_ind,
        });
    });
    GridInformation {
        antennas_by_char: chars_to_coords,
        bounds,
    }
}

struct GridInformation {
    antennas_by_char: HashMap<char, Vec<Coord>>,
    bounds: GridBounds,
}

fn get_grid_bounds(input: &str) -> GridBounds {
    // collect our static grid size references
    let row_max = input.lines().count();
    let col_max = input
        .lines()
        .next()
        .expect("input should not be empty")
        .chars()
        .count();
    GridBounds { row_max, col_max }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

struct Distance {
    row_diff: RowDir,
    col_diff: ColDir,
}

struct GridBounds {
    row_max: usize,
    col_max: usize,
}

enum RowDir {
    Up(usize),
    Down(usize),
    None,
}

enum ColDir {
    Left(usize),
    Right(usize),
    None,
}

impl Distance {
    /// Returns the distance from A to B, with direction
    pub fn from_coords(a: &Coord, b: &Coord) -> Self {
        let row_diff = {
            if a.row == b.row {
                RowDir::None
            } else if a.row > b.row {
                RowDir::Up(a.row - b.row)
            } else {
                RowDir::Down(b.row - a.row)
            }
        };
        let col_diff = {
            if a.col == b.col {
                ColDir::None
            } else if a.col > b.col {
                ColDir::Left(a.col - b.col)
            } else {
                ColDir::Right(b.col - a.col)
            }
        };
        Self { row_diff, col_diff }
    }

    pub fn flip(&mut self) {
        match self.row_diff {
            RowDir::Up(num) => self.row_diff = RowDir::Down(num),
            RowDir::Down(num) => self.row_diff = RowDir::Up(num),
            RowDir::None => {}
        }
        match self.col_diff {
            ColDir::Left(num) => self.col_diff = ColDir::Right(num),
            ColDir::Right(num) => self.col_diff = ColDir::Left(num),
            ColDir::None => {}
        }
    }
}

impl Coord {
    pub fn add_distance(&self, dist: &Distance, bounds: &GridBounds) -> Option<Self> {
        let new_row = match dist.row_diff {
            RowDir::Up(diff) => self.row.checked_sub(diff)?,
            RowDir::Down(diff) => {
                let temp = self.row.checked_add(diff)?;
                if temp >= bounds.row_max {
                    return None;
                }
                temp
            }
            RowDir::None => self.row,
        };
        let new_col = match dist.col_diff {
            ColDir::Left(diff) => self.col.checked_sub(diff)?,
            ColDir::Right(diff) => {
                let temp = self.col.checked_add(diff)?;
                if temp >= bounds.col_max {
                    return None;
                }
                temp
            }
            ColDir::None => self.col,
        };
        Some(Self {
            row: new_row,
            col: new_col,
        })
    }
}

fn calculate_antinode_pair(
    first_antenna: &Coord,
    second_antenna: &Coord,
    bounds: &GridBounds,
) -> AntinodeCase {
    let mut dist = Distance::from_coords(first_antenna, second_antenna);
    let node_2 = second_antenna.add_distance(&dist, bounds);
    dist.flip();
    let node_1 = first_antenna.add_distance(&dist, bounds);
    match (node_1, node_2) {
        (Some(node_1), Some(node_2)) => AntinodeCase::TwoNodes(node_1, node_2),
        (Some(node), None) | (None, Some(node)) => AntinodeCase::OneNode(node),
        (None, None) => AntinodeCase::None,
    }
}

fn calculate_antinode_harmonics(
    first_antenna: &Coord,
    second_antenna: &Coord,
    bounds: &GridBounds,
) -> Vec<Coord> {
    let mut output: Vec<Coord> = vec![first_antenna.clone(), second_antenna.clone()];
    let mut dist = Distance::from_coords(first_antenna, second_antenna);
    // start by going from second onwards
    let mut next_node = second_antenna.add_distance(&dist, bounds);
    while let Some(node) = next_node {
        output.push(node.clone());
        next_node = output
            .last()
            .expect("just pushed, must exist")
            .add_distance(&dist, bounds);
    }
    // then do from first, backwards
    dist.flip();
    next_node = first_antenna.add_distance(&dist, bounds);
    while let Some(node) = next_node {
        output.push(node.clone());
        next_node = output
            .last()
            .expect("just pushed, must exist")
            .add_distance(&dist, bounds);
    }
    output
}

enum AntinodeCase {
    TwoNodes(Coord, Coord),
    OneNode(Coord),
    None,
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid_info = parse_input(input);
    // use a hashset to store all locations where at least one antinode is found
    // we do not care if this overlaps with an antenna
    // we do not care which signal (char) the antinode comes from
    let mut unique_antinodes: HashSet<Coord> = HashSet::new();
    // work through the different signals
    // since antinodes only form from a pair of antennas of the same signal
    grid_info
        .antennas_by_char
        .drain()
        // for each signal
        .for_each(|(_, antennas)| {
            antennas
                .iter()
                // get all pairs (ignoring order: (A, B) == (B, A), and we don't want to double count)
                .tuple_combinations::<(_, _)>()
                // for each pair
                .for_each(
                    // calculate all in-bounds antinodes and add them to the hashset
                    |(a1, a2)| {
                        let harmonics = calculate_antinode_harmonics(a1, a2, &grid_info.bounds);
                        for antinode in harmonics {
                            unique_antinodes.insert(antinode);
                        }
                    },
                );
        });
    // finally return the number of unique antinodes
    Some(unique_antinodes.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn finds_2_nodes_when_using_simple_two_antenna_test_case() {
        // arrange
        let test_case = [
            "..........",
            "..........",
            "....a.....",
            "..........",
            ".....a....",
            "..........",
            "..........",
        ]
        .join("\n");

        // act
        let result = part_one(&test_case);

        // assert
        assert_eq!(result, Some(2))
    }

    #[test]
    fn finds_4_nodes_when_using_simple_three_antenna_test_case() {
        // arrange
        let test_case = [
            "..........",
            "..........",
            "....a.....",
            "........a.",
            ".....a....",
            "..........",
            "..........",
        ]
        .join("\n");

        // act
        let result = part_one(&test_case);

        // assert
        assert_eq!(result, Some(4))
    }

    #[test]
    fn ignores_lone_antenna_when_using_multiple_signals_test_case() {
        // arrange
        let test_case = [
            "..........",
            "..........",
            "....a.....",
            "........a.",
            ".....a....",
            ".......A..",
            "..........",
        ]
        .join("\n");

        // act
        let result = part_one(&test_case);

        // assert
        assert_eq!(result, Some(4))
    }
}
