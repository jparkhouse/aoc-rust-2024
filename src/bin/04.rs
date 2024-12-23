use itertools::Itertools;
use stack_grid::StackGrid;

advent_of_code::solution!(4);

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

    let y_max = input.len();
    let mut x = 0;
    let mut y = 0;
    while y < y_max {
        while x < max_line_length {
            let search_window = StackGrid::from_str(&input, x, y);
            for row in search_window.iter_left_to_right() {
                for (&ch_0, &ch_1, &ch_2, &ch_3) in row.into_iter().tuple_windows() {
                    if let ('X', 'M', 'A', 'S') = (ch_0, ch_1, ch_2, ch_3) {
                        output += 1
                    }
                }
            }
            for r_row in search_window.iter_left_to_right() {
                for (&ch_0, &ch_1, &ch_2, &ch_3) in r_row.into_iter().tuple_windows() {
                    if let ('X', 'M', 'A', 'S') = (ch_0, ch_1, ch_2, ch_3) {
                        output += 1
                    }
                }
            }
        }
    }

    todo!()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

mod stack_grid {

    pub(crate) struct StackGrid {
        search_window: [char; 39 * 39],
        top_to_bottom: [char; 39 * 39],
        diag_from_top_left: [char; 39 * 39],
        diag_from_top_right: [char; 39 * 39],
    }

    impl StackGrid {
        /// Attempts to fill the StackGrid from the (`position`, `position`) position of the heap-allocated object
        pub fn from_str(input: &Vec<Vec<char>>, x_position: usize, y_position: usize) -> Self {
            if input.len() < y_position {
                return Self::new();
            }
            let mut stack_grid = [' '; 39 * 39];
            for (ind, line) in input.into_iter().skip(y_position).take(39).enumerate() {
                for i in 0..39 {
                    if let Some(&ch) = line.get(x_position + i) {
                        stack_grid[ind * 39 + i] = ch;
                    }
                }
            }
            Self {
                search_window: stack_grid,
                top_to_bottom: StackGrid::calculate_top_to_bottom(stack_grid),
                diag_from_top_left: StackGrid::calculate_top_left_diagonal(stack_grid),
                diag_from_top_right: StackGrid::calculate_top_right_diagonal(stack_grid),
            }
        }

        pub fn new() -> Self {
            Self {
                search_window: [' '; 39 * 39],
                top_to_bottom: [' '; 39 * 39],
                diag_from_top_left: [' '; 39 * 39],
                diag_from_top_right: [' '; 39 * 39],
            }
        }

        pub fn iter_left_to_right(&self) -> LeftToRight<'_> {
            LeftToRight::new(&self.search_window)
        }

        pub fn iter_top_to_bottom(&self) -> TopToBottom<'_> {
            TopToBottom::new(&self.top_to_bottom)
        }

        pub fn iter_diag_from_top_left(&self) -> DiagFromTopLeft<'_> {
            DiagFromTopLeft::new(&self.top_to_bottom)
        }

        pub fn iter_diag_from_top_right(&self) -> DiagFromTopRight<'_> {
            DiagFromTopRight::new(&self.top_to_bottom)
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
        fn calculate_top_to_bottom(input: [char; 39 * 39]) -> [char; 39 * 39] {
            let mut output = [' '; 39 * 39];
            for (ind, ch) in input.into_iter().enumerate() {
                // if it were a standard 39x39 grid
                // you could count all elements,
                // and get the col and row by:
                let col = (ind % 39);
                let row = (ind / 39);
                // so then we just swap these to read the other way
                output[col * 39 + row] = ch;
            }
            output
        }

        /// Starts from (0, 0) and outputs the diagonals
        fn calculate_top_left_diagonal(input: [char; 39 * 39]) -> [char; 39 * 39] {
            let mut output = [' '; 39 * 39];
            let mut counter = 0;
            for i in 0..38 {
                for j in 0..(i + 1) {
                    output[counter] = input[j * (39 - 1) + i];
                    counter += 1;
                }
            }
            // and then switch to the next stage
            for i in (0..39).rev() {
                for j in 0..(i + 1) {
                    output[counter] = input[(j as i32 - (i as i32 - 38)) as usize * 39 + (38 - j)];
                    counter += 1;
                }
            }
            // check we got them all
            assert_eq!(counter, 39 * 39);
            output
        }

        /// Starts from (0, 38) and outputs the diagonals
        fn calculate_top_right_diagonal(input: [char; 39 * 39]) -> [char; 39 * 39] {
            let mut output = [' '; 39 * 39];
            let mut counter = 0;
            for i in 0..38 {
                for j in 0..(i + 1) {
                    output[counter] = input[j * (39 + 1) + (38 - i)];
                    counter += 1;
                }
            }
            // and then switch to the next stage
            for i in 0..39 {
                for j in 0..(39 - i) {
                    output[counter] = input[(i * 39) + (j * (39 + 1))];
                    counter += 1;
                }
            }
            // check we got them all
            assert_eq!(counter, 39 * 39);
            output
        }
    }

    enum CardinalDirection {
        LeftToRight,
        RightToLeft,
        TopToBottom,
        BottomToTop,
    }

    enum DiagonalDirection {
        FromTopLeft,
        FromTopRight,
        FromBottomRight,
        FromBottomLeft,
    }

    struct CardinalIter<'a> {
        data: &'a [char; 39 * 39],
        index: usize,
        reversed: bool,
    }

    impl<'a> CardinalIter<'a> {
        fn new(data: &'a [char; 39 * 39], is_reversed: bool) -> Self {
            Self {
                data: data,
                index: if is_reversed { 38 } else { 0 },
                reversed: is_reversed,
            }
        }
    }

    impl<'a> Iterator for CardinalIter<'a> {
        type Item = &'a [char];

        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= 39 && !self.reversed {
                return None;
            }

            if self.index == 0 && self.reversed {
                return None;
            }

            let output = &self.data[(self.index * 39)..((self.index + 1) * 39)];
            match self.reversed {
                false => self.index += 1,
                true => self.index -= 1,
            };

            Some(output)
        }
    }

    stuct DiagonalIter<'a> {
        data: &'a [char; 39 * 39],
        index: usize,
        reversed: bool,
        direction_flip: bool
    }

    impl<'a> DiagonalIter<'a> {
        fn new(data: &'a [char; 39 * 39], is_reversed: bool) -> Self {
            Self {
                data: data,
                index: 0,
                reversed: is_reversed,
                direction_flip: false
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
                    if self.index == 39 {
                        self.direction = true
                    }
                    Some(output)
                }
                true => {
                    // we have now flipped
                    // all the ones we have already done
                    let modifier: usize = (0..39).sum();
                    let output: &[char] = &self.data[(modifier..modifier + (39 - self.index)).sum()
                        ..((modifier..modifier + (38 - self.index)).sum())];
                    self.index += 1;
                    Some(output)
                }
            }
        }
    }

    struct DiagFromTopLeft<'a> {
        data: &'a [char; 39 * 39],
        index: usize,
        direction: bool,
    }

    impl<'a> DiagFromTopLeft<'a> {
        fn new(data: &'a [char; 39 * 39]) -> Self {
            DiagFromTopLeft {
                data: data,
                index: 0,
                direction: false,
            }
        }
    }

    impl<'a> Iterator for DiagFromTopLeft<'a> {
        type Item = &'a [char];

        fn next(&mut self) -> Option<Self::Item> {
            
        }
    }

    struct DiagFromTopRight<'a> {
        data: &'a [char; 39 * 39],
        index: usize,
        direction: bool,
    }

    impl<'a> DiagFromTopRight<'a> {
        fn new(data: &'a [char; 39 * 39]) -> Self {
            DiagFromTopRight {
                data: data,
                index: 0,
                direction: false,
            }
        }
    }

    impl<'a> Iterator for DiagFromTopRight<'a> {
        type Item = &'a [char];

        fn next(&mut self) -> Option<Self::Item> {
            if self.index == 0 && self.direction {
                return None;
            }
            match self.direction {
                false => {
                    // we have not yet flipped
                    let output: &[char] =
                        &self.data[(0..self.index).sum()..((0..(self.index + 1)).sum())];
                    self.index += 1;
                    if self.index == 39 {
                        self.direction = true
                    }
                    Some(output)
                }
                true => {
                    // we have now flipped
                    // all the ones we have already done
                    let modifier: usize = (0..39).sum();
                    let output: &[char] = &self.data[(modifier..modifier + (39 - self.index)).sum()
                        ..((modifier..modifier + (38 - self.index)).sum())];
                    self.index += 1;
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
