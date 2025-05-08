use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let starting_stones = parse_input_to_stones(input);
    let mut mem_dict = MemDict::new();
    let result = starting_stones.into_iter().map(|stone| get_stone_state(stone, 25, &mut mem_dict)).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let starting_stones = parse_input_to_stones(input);
    let mut mem_dict = MemDict::new();
    let result = starting_stones.into_iter().map(|stone| get_stone_state(stone, 75, &mut mem_dict)).sum();
    Some(result)
}

type MemDict = HashMap<StoneQuery, u64>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct StoneQuery {
    stone: Stone,
    number_of_blinks: u64
}

fn get_stone_state(stone: Stone, number_of_blinks: u64, mem_dict: &mut MemDict) -> u64 {
    if number_of_blinks == 0 {
        return 1;
    }
    let s_q = StoneQuery { stone, number_of_blinks };
    if let Some(&prev) = mem_dict.get(&s_q) {
        return prev;
    }
    let blink = stone.blink();
    let result = match blink {
        StoneOutput::Single(stone) => get_stone_state(stone, number_of_blinks - 1, mem_dict),
        StoneOutput::Split(s_1, s_2) => get_stone_state(s_1, number_of_blinks - 1, mem_dict)
            + get_stone_state(s_2, number_of_blinks - 1, mem_dict),
    };
    // update the mem_dict for next time
    mem_dict.insert(s_q, result);
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Stone {
    num: u64
}

impl Stone {
    pub fn blink(self) -> StoneOutput {
        // first rule - 0 -> 1
        if self.num == 0 {
            return StoneOutput::Single(Stone { num: 1 });
        }
        // second rule, split in half by char
        let num_string = self.num.to_string();
        let num_len = num_string.len();
        if num_len % 2 == 0 {
            let half_len = num_len / 2;
            let first_num = (&num_string[0..half_len]).parse()
                .expect("valid input should create splittable numbers");
            let second_num = (&num_string[half_len..]).parse()
                .expect("valid input should create splittable numbers");
            return StoneOutput::Split(Stone { num: first_num }, Stone { num: second_num });
        }
        // otherwise multiply by 2024
        StoneOutput::Single(Stone { num: self.num * 2024 })
    }
}

enum StoneOutput {
    Single(Stone),
    Split(Stone, Stone),
}

/// Takes in the input format and returns a Vec of Stones.
/// For example `0 1 2` would return `vec![ Stone { num: 0 }, Stone { num: 1 }, Stone { num: 2 }]`
fn parse_input_to_stones(input: &str) -> Vec<Stone> {
    let row = input.split(" ")
        .map(|num_str| {
            match num_str.parse() {
                Ok(num) => Stone { num },
                Err(e) => panic!("Error in StoneRow::from_str(): {}", e),
            }
        })
        .collect();
    row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55_312));
    }

    // test two is invalid since there is no given answer for the example taken to 75 blinks
    /* #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    } */
}
