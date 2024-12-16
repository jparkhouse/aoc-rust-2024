use std::{collections::HashMap, iter::zip};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left_array, mut right_array) =
        parse_input_to_lists(input).expect("file should be parsible");
    left_array.sort();
    right_array.sort();
    let mut count = 0;
    for (left, right) in zip(left_array, right_array) {
        let dif = match left >= right {
            true => left - right,
            false => right - left,
        };
        count += dif;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left_array, right_array) = parse_input_to_lists(input).expect("file should be parsible");
    // first we look through the right array and count each unique location
    let mut similarity_table: HashMap<u64, u64> = HashMap::new();
    right_array.into_iter().for_each(|loc| {
        let _ = match similarity_table.get(&loc) {
            // already contained, so we just add one more
            Some(quantity) => similarity_table.insert(loc, *quantity + 1),
            // not contained, so we add the first sighting of it
            None => similarity_table.insert(loc, 1),
        };
    });
    let similarity_score = left_array.into_iter().map(|loc| {
        // location from left list * count in right list
        match similarity_table.get(&loc) {
            Some(score) => loc * *score,
            None => 0
        }
    }).fold(0, |acc, loc_score| acc + loc_score);
    Some(similarity_score)
}

fn parse_input_to_lists(input: &str) -> Result<(Vec<u64>, Vec<u64>), String> {
    let lines = input.lines();
    let no_of_lines = lines.size_hint().0;
    let id_pairs = lines.map(|l| l.split("   "));
    let mut left_array: Vec<u64> = Vec::with_capacity(no_of_lines);
    let mut right_array: Vec<u64> = Vec::with_capacity(no_of_lines);
    id_pairs.for_each(|mut s| {
        let left = s.next().expect("should contain first of two entries");
        let right = s.next().expect("should contain second of two entries");
        left_array.push(
            u64::from_str_radix(left, 10)
                .expect(format!("left part ({}) should be valid int", left).as_str()),
        );
        right_array.push(
            u64::from_str_radix(right, 10)
                .expect(format!("right part ({}) should be valid int", right).as_str()),
        );
    });
    Ok((left_array, right_array))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
