use itertools::Itertools;
use stack_grid::{CardinalDirection, DiagonalDirection, StackGrid};

advent_of_code::solution!(4);

const N: usize = 39;

pub fn part_one(input: &str) -> Option<u64> {
    let mut max_line_length = 0;
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            let v: Vec<char> = line.chars().collect();
            if v.len() > max_line_length {
                max_line_length = v.len();
            }
            v
        })
        .collect();

    let mut output: u64 = 0;

    let x_max = max_line_length - 3;
    let y_max = input.len() - 3;
    let mut x = 0;
    let mut y = 0;
    while y < y_max {
        while x < x_max {
            let search_window = StackGrid::from_str(&input, x, y);
            for dir in [
                CardinalDirection::LeftToRight,
                CardinalDirection::RightToLeft,
                CardinalDirection::TopToBottom,
                CardinalDirection::BottomToTop,
            ] {
                for line in search_window.cardinal_iter(dir) {
                    for word in line.into_iter().tuple_windows() {
                        match word {
                            ('X', 'M', 'A', 'S') => output += 1,
                            _ => {}
                        }
                    }
                }
            }
            println!(
                "after checking all cardinal directions, count is {}",
                output
            );
            for dir in [
                DiagonalDirection::FromBottomLeft,
                DiagonalDirection::FromBottomRight,
                DiagonalDirection::FromTopLeft,
                DiagonalDirection::FromTopRight,
            ] {
                println!("{:?}", dir);
                for line in search_window.diagonal_iter(dir) {
                    println!("{:?}", line);
                    for word in line.into_iter().tuple_windows() {
                        match word {
                            ('X', 'M', 'A', 'S') => output += 1,
                            _ => {}
                        }
                    }
                }
            }
            x += N - 3;
        }
        x = 0;
        y += N - 3;
    }

    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

mod stack_grid {
    use crate::N;


    pub(crate) struct StackGrid {
        search_window: [char; N * N],
        top_to_bottom: [char; N * N],
        diag_from_top_left: [char; N * N],
        diag_from_top_right: [char; N * N],
        diag_from_top_left_rev: [char; N * N],
        diag_from_top_right_rev: [char; N * N],
    }

    impl StackGrid {
        /// Attempts to fill the StackGrid from the (`position`, `position`) position of the heap-allocated object
        pub fn from_str(input: &Vec<Vec<char>>, x_position: usize, y_position: usize) -> Self {
            if input.len() < y_position {
                return Self::new();
            }
            let mut stack_grid = [' '; N * N];
            for (ind, line) in input.into_iter().skip(y_position).take(N).enumerate() {
                for i in 0..N {
                    if let Some(&ch) = line.get(x_position + i) {
                        stack_grid[ind * N + i] = ch;
                    }
                }
            }
            let diag_from_top_left = StackGrid::calculate_top_left_diagonal(stack_grid);
            let diag_from_top_right = StackGrid::calculate_top_right_diagonal(stack_grid);
            Self {
                search_window: stack_grid,
                top_to_bottom: StackGrid::calculate_top_to_bottom(stack_grid),
                diag_from_top_left: diag_from_top_left,
                diag_from_top_right: diag_from_top_right,
                diag_from_top_left_rev: calculate_reverse_char_array(diag_from_top_left),
                diag_from_top_right_rev: calculate_reverse_char_array(diag_from_top_right),
            }
        }

        pub fn new() -> Self {
            Self {
                search_window: [' '; N * N],
                top_to_bottom: [' '; N * N],
                diag_from_top_left: [' '; N * N],
                diag_from_top_right: [' '; N * N],
                diag_from_top_left_rev: [' '; N * N],
                diag_from_top_right_rev: [' '; N * N],
            }
        }

        pub fn cardinal_iter(&self, direction: CardinalDirection) -> CardinalIter<'_> {
            use CardinalDirection::*;
            match direction {
                LeftToRight => CardinalIter::new(&self.search_window, false),
                RightToLeft => CardinalIter::new(&self.search_window, true),
                TopToBottom => CardinalIter::new(&self.top_to_bottom, false),
                BottomToTop => CardinalIter::new(&self.top_to_bottom, true),
            }
        }

        pub fn diagonal_iter(&self, direction: DiagonalDirection) -> DiagonalIter<'_> {
            use DiagonalDirection::*;
            match direction {
                FromTopLeft => DiagonalIter::new(&self.diag_from_top_left),
                FromTopRight => DiagonalIter::new(&self.diag_from_top_right),
                FromBottomRight => DiagonalIter::new(&self.diag_from_top_right_rev),
                FromBottomLeft => DiagonalIter::new(&self.diag_from_top_left_rev),
            }
        }

        /// Takes the in order array, and returns it top to bottom.
        /// For example:
        /// ```
        /// 0 1 2
        /// 3 4 5
        /// 6 7 8
        ///
        ///
        /// position 0 -> (0 % 3 = 0, 0 / 3 = 0)
        /// position 1 -> (1 % 3 = 1, 1 / 3 = 0)
        /// position 2 -> (2 % 3 = 2, 2 / 3 = 0)
        /// position 3 -> (3 % 3 = 0, 3 / 3 = 1)
        /// position 4 -> (4 % 3 = 1, 4 / 3 = 1)
        /// etc
        /// ```
        ///
        /// If we swap these indices around, we then get:
        /// ```
        /// 0 3 6
        /// 1 4 7
        /// 2 5 8
        /// ```
        fn calculate_top_to_bottom(input: [char; N * N]) -> [char; N * N] {
            let mut output = [' '; N * N];
            for (ind, ch) in input.into_iter().enumerate() {
                // if it were a standard NxN grid
                // you could count all elements,
                // and get the col and row by:
                let col = ind % N;
                let row = ind / N;
                // so then we just swap these to read the other way
                output[col * N + row] = ch;
            }
            output
        }

        /// Starts from (0, 0) and outputs the diagonals
        fn calculate_top_left_diagonal(input: [char; N * N]) -> [char; N * N] {
            println!("top left diagonal");
            let mut output = [' '; N * N];
            let mut counter = 0;

            // First half
            for i in 0..(N - 1) {
                for j in 0..(i + 1) {
                    output[counter] = input[j * N + (i - j)];
                    counter += 1;
                }
            }

            // Second half
            for i in 0..N {
                for j in 0..(N - i) {
                    let start = (N - 1) + (i * N);
                    output[counter] = input[j * (N - 1) + start];
                    counter += 1;
                }
            }

            assert_eq!(counter, N * N);
            println!("{:?}", output);
            output
        }

        /// Starts from (0, 38) and outputs the diagonals
        fn calculate_top_right_diagonal(input: [char; N * N]) -> [char; N * N] {
            println!("top right diagonal");
            let mut output = [' '; N * N];
            let mut counter = 0;

            // First half
            for i in 0..(N - 1) {
                for j in 0..(i + 1) {
                    let start = (N - 1) - i;
                    output[counter] = input[j * (N + 1) + start];
                    counter += 1;
                }
            }

            // Second half
            for i in 0..N {
                for j in 0..(N - i) {
                    let start = i * N;
                    output[counter] = input[start + j * (N + 1)];
                    counter += 1;
                }
            }

            assert_eq!(counter, N * N);
            println!("{:?}", output);
            output
        }
    }

    fn calculate_reverse_char_array(input: [char; N * N]) -> [char; N * N] {
        let mut output = [' '; N * N];
        for (ind, ch) in input.into_iter().enumerate() {
            output[(N * N - 1) - ind] = ch;
        }
        output
    }

    pub enum CardinalDirection {
        LeftToRight,
        RightToLeft,
        TopToBottom,
        BottomToTop,
    }

    #[derive(Debug)]
    pub enum DiagonalDirection {
        FromTopLeft,
        FromTopRight,
        FromBottomRight,
        FromBottomLeft,
    }

    pub struct CardinalIter<'a> {
        data: &'a [char; N * N],
        index: usize,
        reversed: bool,
        done: bool,
    }

    impl<'a> CardinalIter<'a> {
        fn new(data: &'a [char; N * N], is_reversed: bool) -> Self {
            Self {
                data: data,
                index: if is_reversed { N - 1 } else { 0 },
                reversed: is_reversed,
                done: false,
            }
        }
    }

    impl<'a> Iterator for CardinalIter<'a> {
        type Item = &'a [char];

        fn next(&mut self) -> Option<Self::Item> {
            if self.done {
                return None;
            }

            let output = &self.data[(self.index * N)..((self.index + 1) * N)];
            match self.reversed {
                false => {
                    if self.index == N - 1 {
                        self.done = true
                    } else {
                        self.index += 1
                    }
                }
                true => {
                    if self.index == 0 {
                        self.done = true
                    } else {
                        self.index -= 1
                    }
                }
            };

            Some(output)
        }
    }

    pub struct DiagonalIter<'a> {
        data: &'a [char; N * N],
        index: usize,
        direction_flip: bool,
    }

    impl<'a> DiagonalIter<'a> {
        fn new(data: &'a [char; N * N]) -> Self {
            Self {
                data: data,
                index: 1,
                direction_flip: false,
            }
        }
    }

    impl<'a> Iterator for DiagonalIter<'a> {
        type Item = &'a [char];

        fn next(&mut self) -> Option<Self::Item> {
            if self.index == 0 && self.direction_flip {
                return None;
            }
            match self.direction_flip {
                false => {
                    // we have not yet flipped
                    let output: &[char] =
                        &self.data[(0..self.index).sum()..((0..(self.index + 1)).sum())];
                    self.index += 1;
                    if self.index == N {
                        self.direction_flip = true
                    }
                    Some(output)
                }
                true => {
                    // we have now flipped
                    // all the ones we have already done
                    let first_half_length: usize = (0..N).sum();
                    let already_got_from_second_half: usize = ((self.index + 1)..(N+1)).sum();
                    let start = first_half_length + already_got_from_second_half;
                    let end = first_half_length + already_got_from_second_half + self.index;
                    let output: &[char] = &self.data[start..end];
                    self.index -= 1;
                    Some(output)
                }
            }
        }
    }
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
        assert_eq!(result, None);
    }
}
