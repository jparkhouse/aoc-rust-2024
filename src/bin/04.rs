advent_of_code::solution!(4);
use std::iter::FromIterator;

pub fn part_one(input: &str) -> Option<u64> {
    let mut output: u64 = 0;
    let input: Vec<&str> = input.lines().collect();
    let n = input.len();
    let input: Vec<char> = input.iter().flat_map(|line| line.chars()).collect();

    // the coords of all the X chars
    let xs: Vec<Coord> = input
        .iter()
        .enumerate()
        .filter_map(|(raw_ind, &ch)| {
            if ch == 'X' {
                Some(Coord::from_raw_ind(raw_ind, n))
            } else {
                None
            }
        })
        .collect();

    [
        Direction::Up,
        Direction::UpRight,
        Direction::Right,
        Direction::DownRight,
        Direction::Down,
        Direction::DownLeft,
        Direction::Left,
        Direction::UpLeft,
    ]
    .into_iter()
    .for_each(|dir| {
        xs.iter().for_each(|x_loc| {
            x_loc
                .shift(dir, n)
                .and_then(|m_loc| {
                    if let Some('M') = input.get(
                        m_loc
                            .to_raw_ind(n)
                            .expect("shift should rule out invalid coords"),
                    ) {
                        return m_loc.shift(dir, n);
                    }
                    None
                })
                .and_then(|a_loc| {
                    if let Some('A') = input.get(
                        a_loc
                            .to_raw_ind(n)
                            .expect("shift should rule out invalid coords"),
                    ) {
                        return a_loc.shift(dir, n);
                    }
                    None
                })
                .and_then(|s_loc| {
                    if let Some('S') = input.get(
                        s_loc
                            .to_raw_ind(n)
                            .expect("shift should rule out invalid coords"),
                    ) {
                        return Some(s_loc);
                    }
                    None
                })
                .map(|_| output += 1);
        });
    });
    Some(output)
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn to_raw_ind(&self, n: usize) -> Option<usize> {
        if self.row >= n || self.col >= n {
            return None;
        }
        Some(self.row * n + self.col)
    }

    fn from_raw_ind(raw_ind: usize, n: usize) -> Self {
        Self {
            row: raw_ind / n,
            col: raw_ind % n,
        }
    }

    fn shift_up(self, _n: usize) -> Option<Self> {
        if self.row > 0 {
            return Some(Self {
                row: self.row - 1,
                col: self.col,
            });
        }
        None
    }

    fn shift_right(self, n: usize) -> Option<Self> {
        if self.col < n - 1 {
            return Some(Self {
                row: self.row,
                col: self.col + 1,
            });
        }
        None
    }

    fn shift_down(self, n: usize) -> Option<Self> {
        if self.row < n - 1 {
            return Some(Self {
                row: self.row + 1,
                col: self.col,
            });
        }
        None
    }

    fn shift_left(self, _n: usize) -> Option<Self> {
        if self.col > 0 {
            return Some(Self {
                row: self.row,
                col: self.col - 1,
            });
        }
        None
    }

    fn shift(self, direction: Direction, n: usize) -> Option<Self> {
        use Direction::*;
        match direction {
            Up => self.shift_up(n),
            UpRight => self.shift_up(n).and_then(|c| c.shift_right(n)),
            Right => self.shift_right(n),
            DownRight => self.shift_down(n).and_then(|c| c.shift_right(n)),
            Down => self.shift_down(n),
            DownLeft => self.shift_down(n).and_then(|c| c.shift_left(n)),
            Left => self.shift_left(n),
            UpLeft => self.shift_up(n).and_then(|c| c.shift_left(n)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

pub fn part_two(input: &str) -> Option<u64> {
    // initialise some stuff
    let mut output: u64 = 0;
    let input: Vec<&str> = input.lines().collect();
    let n = input.len();
    let input: Vec<char> = input.iter().flat_map(|line| line.chars()).collect();

    // this time we find the 'A' chars
    let a_chars: Vec<Coord> = input
        .iter()
        .enumerate()
        .filter_map(|(raw_ind, &ch)| {
            if ch == 'A' {
                Some(Coord::from_raw_ind(raw_ind, n))
            } else {
                None
            }
        })
        .collect();

    // now we need to check around the 'A' chars, shifting in the diagonals, and look for a valid pattern
    a_chars.into_iter().for_each(|coord| {
        let diags: Vec<Option<char>> = [
            Direction::UpRight,
            Direction::DownRight,
            Direction::DownLeft,
            Direction::UpLeft,
        ]
        .into_iter()
        .map(|dir| {
            // check we can shift in that direction
            if let Some(diag) = coord.clone().shift(dir, n) {
                // check that there is a char there in the input
                if let Some(&ch) = input.get(
                    diag.to_raw_ind(n)
                        .expect("shift should rule out invalid coords"),
                ) {
                    // only return the chars we care about
                    if ch == 'M' || ch == 'S' {
                        return Some(ch);
                    }
                }
            }
            // otherwise return None
            None
        })
        .collect();
        match diags.as_slice() {
            [Some('M'), Some('M'), Some('S'), Some('S')] => output += 1,
            [Some('S'), Some('M'), Some('M'), Some('S')] => output += 1,
            [Some('S'), Some('S'), Some('M'), Some('M')] => output += 1,
            [Some('M'), Some('S'), Some('S'), Some('M')] => output += 1,
            _ => {}
        }
    });
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
