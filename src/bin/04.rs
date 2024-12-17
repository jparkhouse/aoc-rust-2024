advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

struct CharGrid(Vec<Vec<char>>);

impl CharGrid {
    fn from_str(s: &str) -> Self {
        Self {0: s.lines().map(|line| line.chars().collect()).collect()} 
    }
}
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
