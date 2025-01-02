use std::fmt::Debug;
use itertools::{self, Itertools};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let safe_record_count = parse_input(input)
        .into_iter()
        .fold(0, |safe_records, next_record| {
            match is_valid_sequence(&next_record) {
                true => safe_records + 1,
                false => safe_records,
            }
        });
    Some(safe_record_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let safe_record_count = parse_input(input)
        .into_iter()
        .fold(0, |safe_records, next_record| {
            match is_valid_sequence(&next_record) {
                true => safe_records + 1,
                false => {
                    match from_seq_return_all_permutations_without_one_element(&next_record).any(|sub_seq: Vec<u64>| is_valid_sequence(&sub_seq)) {
                        true => safe_records + 1,
                        false => safe_records
                    }
                },
            }
        });
    Some(safe_record_count)
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let lines = input.lines();
    lines
        .map(|l| {
            l.split(' ')
                .map(|num| u64::from_str_radix(num, 10).unwrap())
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Asc,
    Desc,
    Unknown,
}

fn get_direction_from_pair(first: u64, second: u64, direction: Direction) -> Option<Direction> {
    match direction {
        Direction::Asc => {
            // if Some, implies that second is ge first
            if let Some(x) = second.checked_sub(first) {
                if x < 4 && x > 0 {
                    return Some(Direction::Asc);
                }
                return None;
            }
            None
        }
        Direction::Desc => {
            // if Some, implies that first is ge second
            if let Some(x) = first.checked_sub(second) {
                if x < 4 && x > 0 {
                    return Some(Direction::Desc);
                }
                return None;
            }
            None
        }
        Direction::Unknown => {
            // since the direction is unknown, we should return a valid direction if we can find one, or None if no valid direction
            let check_up = get_direction_from_pair(first, second, Direction::Asc);
            let check_down = get_direction_from_pair(first, second, Direction::Desc);
            match (check_up, check_down) {
                // if our ascending check returns ascending, then it must be ascending
                (Some(Direction::Asc), _) => Some(Direction::Asc),
                // if our descending check returns descending, then it must be descending
                (_, Some(Direction::Desc)) => Some(Direction::Desc),
                // otherwise it is neither a valid ascending or descending pair, so there is no direction to return
                (_, _) => None,
            }
        }
    }
}

fn is_valid_sequence(seq: &[u64]) -> bool {
    let mut seq_iter = seq.into_iter().tuple_windows();
    let first_pair: (&u64, &u64) = match seq_iter.next() {
        Some(pair) => pair,
        None => return false
    };
    let initial_dir = match get_direction_from_pair(*first_pair.0, *first_pair.1, Direction::Unknown) {
        Some(Direction::Asc) => Direction::Asc,
        Some(Direction::Desc) => Direction::Desc,
        _ => return false
    };
    for (&first, &second) in seq_iter {
        match get_direction_from_pair(first, second, initial_dir) {
            Some(dir) if dir == initial_dir => {},
            _ => return false
        };
    }
    true
}

fn from_seq_return_all_permutations_without_one_element(seq: &[u64]) -> impl Iterator<Item = Vec<u64>> + '_ {
    (0..seq.len()).into_iter().map(|skip| {
        let mut vec = Vec::with_capacity(seq.len() - 1);
        vec.extend_from_slice(&seq[..skip]);
        vec.extend_from_slice(&seq[skip + 1..]);
        vec
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
