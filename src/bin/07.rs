use advent_of_code::shared::{match_numeric, parse_number_from_str};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| parse_line(line).and_then(|case| Ok(evaluate_test_case(case))))
            .fold(0, |acc, next_res| {
                if let Ok(Some(num)) = next_res {
                    return acc + num;
                }
                acc
            }),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                parse_line(line).and_then(|case| Ok(evaluate_test_case_with_concatenation(case)))
            })
            .fold(0, |acc, next_res| {
                if let Ok(Some(num)) = next_res {
                    return acc + num;
                }
                acc
            }),
    )
}
struct TestCase {
    target: u64,
    inputs: Vec<u64>,
}

/// Used to evalutate test cases for part 1.
/// Returns `Some(case.target)` if the test case is valid
/// otherwise `None`
fn evaluate_test_case(case: TestCase) -> Option<u64> {
    fn inner_rsolve(target: u64, current: u64, next: &[u64]) -> bool {
        match next.len() {
            0 => {
                // no more items, so if not target, must not match
                current == target
            }
            1 => {
                // one last chance to meet the target
                current + next[0] == target || current * next[0] == target
            }
            _ => {
                // if there are more than 1 remaining element, we must pass to the next level of recursion
                inner_rsolve(target, current + next[0], &next[1..])
                    || inner_rsolve(target, current * next[0], &next[1..])
            }
        }
    }

    let valid_case = match case.inputs.len() {
        0 => false,
        1 => case.target == case.inputs[0],
        _ => inner_rsolve(case.target, case.inputs[0], &case.inputs[1..]),
    };

    if valid_case { Some(case.target) } else { None }
}

/// Used to evalutate test cases for part 2, with the addition of concatenation.
/// Returns `Some(case.target)` if the test case is valid
/// otherwise `None`
fn evaluate_test_case_with_concatenation(case: TestCase) -> Option<u64> {
    fn inner_rsolve(target: u64, current: u64, next: &[u64]) -> bool {
        if next.len() == 0 {
            return current == target;
        }
        let possible_currents = [
            current + next[0],
            current * next[0],
            current.concatenate(&next[0]),
        ];
        possible_currents
            .into_iter()
            .any(|num| inner_rsolve(target, num, &next[1..]))
    }

    let valid_case = match case.inputs.len() {
        0 => false,
        1 => case.target == case.inputs[0],
        _ => inner_rsolve(case.target, case.inputs[0], &case.inputs[1..]),
    };

    if valid_case { Some(case.target) } else { None }
}

trait Concatenation {
    fn concatenate(&self, other: &Self) -> Self;
}

impl Concatenation for u64 {
    fn concatenate(&self, other: &Self) -> Self {
        let nums: Vec<u64> = other
            .to_string()
            .chars()
            .map(|ch| match_numeric(ch).expect("must be numeric"))
            .collect();
        nums.into_iter().fold(*self, |acc, next| acc * 10 + next)
    }
}

fn parse_line(line: &str) -> Result<TestCase, String> {
    let parts: Vec<&str> = line.split(": ").collect();

    let target: u64 = match parts.get(0) {
        Some(&target) => parse_number_from_str(target)?,
        None => return Err(format!("Invalid line format: {}", line)),
    };

    let inputs = match parts.get(1) {
        Some(&inputs) => inputs
            .split(" ")
            .map(|str| parse_number_from_str(str))
            .collect::<Result<Vec<u64>, String>>()?,
        None => return Err(format!("Invalid line format: {}", line)),
    };
    Ok(TestCase { target, inputs })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3_749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11_387));
    }
}
