use itertools::Itertools;
use num_rational::Rational64;
use num_traits::ops::checked::{CheckedMul, CheckedSub};

advent_of_code::solution!(13);

const OFFSET: i64 = 10_000_000_000_000;

pub fn part_one(input: &str) -> Option<u64> {
    let tokens_spent = parse_input(input)
        .flat_map(|(matrix, solutions)| calculate_solutions(matrix, solutions))
        // filter out any with more than 100 button presses
        .flat_map(|(a, b)| {
            if a <= 100 && b <= 100 {
                Some(a * 3 + b)
            } else {
                None
            }
        })
        .sum();

    Some(tokens_spent)
}

pub fn part_two(input: &str) -> Option<u64> {
    let offset = Rational64::from(OFFSET);
    let tokens_spent = parse_input(input)
        .map(|(matrix, mut solutions)| {
            solutions.0 += offset;
            solutions.1 += offset;
            (matrix, solutions)
        })
        .flat_map(|(matrix, solutions)| calculate_solutions(matrix, solutions))
        .map(|(a, b)| a * 3 + b)
        .sum();

    Some(tokens_spent)
}

fn calculate_solutions(
    matrix: [Rational64; 4],
    solutions: (Rational64, Rational64),
) -> Option<(u64, u64)> {
    let zero = Rational64::from(0);
    // returns None if det is 0
    let inv_det = get_inv_det(matrix)?;
    let pre_x = calculate_pre_x_factor(matrix, solutions);
    let pre_y = calculate_pre_y_factor(matrix, solutions);
    let x = pre_x / inv_det;
    let y = pre_y / inv_det;
    // if we have integer answers and they are positive
    if x.is_integer() && x > zero && y.is_integer() && y > zero {
        Some((x.to_integer() as u64, y.to_integer() as u64))
    } else {
        None
    }
}

fn calculate_pre_x_factor(
    matrix: [Rational64; 4],
    solutions: (Rational64, Rational64),
) -> Rational64 {
    let mut pre_x = None;
    let mul_1 = solutions.0.checked_mul(&matrix[3]);
    let mul_2 = solutions.1.checked_mul(&matrix[1]);
    if let Some((a, b)) = mul_1.zip(mul_2) {
        pre_x = a.checked_sub(&b);
    }
    match pre_x {
        Some(val) => val,
        None => panic!("reached an overflow!"),
    }
}

fn calculate_pre_y_factor(
    matrix: [Rational64; 4],
    solutions: (Rational64, Rational64),
) -> Rational64 {
    let mut pre_y = None;
    let mul_1 = solutions.1.checked_mul(&matrix[0]);
    let mul_2 = solutions.0.checked_mul(&matrix[2]);
    if let Some((a, b)) = mul_1.zip(mul_2) {
        pre_y = a.checked_sub(&b);
    }
    match pre_y {
        Some(val) => val,
        None => panic!("reached an overflow!"),
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = ([Rational64; 4], (Rational64, Rational64))> {
    input
        .lines()
        // filter out empty lines
        .filter(|line| !line.trim().is_empty())
        // put into groups of 3
        .tuples()
        // process each 3-line group and map into our problem inputs
        .flat_map(|(line_a, line_b, line_p)| {
            let (ax, ay) = parse_line(line_a, "+")?;
            let (bx, by) = parse_line(line_b, "+")?;
            let (px, py) = parse_line(line_p, "=")?;
            let matrix = [
                Rational64::from(ax),
                Rational64::from(bx),
                Rational64::from(ay),
                Rational64::from(by),
            ];
            let solutions = (Rational64::from(px), Rational64::from(py));
            Some((matrix, solutions))
        })
}

fn parse_line(line: &str, pat: &'static str) -> Option<(i64, i64)> {
    let x_pos = line.find(pat)?;
    let y_pos = {
        // plus 1 to skip the previous pattern, otherwise
        // we just get x_pos again
        let offset = x_pos + 1;
        line[offset..].find(pat)?
        // and then re-offset by x_position again
        + offset
    };

    // then slice out our numbers and parse them
    let x = line[x_pos + 1..y_pos - 3].parse().ok()?;
    let y = line[y_pos + 1..].parse().ok()?;

    Some((x, y))
}

fn get_inv_det(input: [Rational64; 4]) -> Option<Rational64> {
    let ad = input[0] * input[3];
    let bc = input[1] * input[2];
    let inv_det = ad - bc;
    if inv_det != Rational64::from(0) {
        Some(inv_det)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875_318_608_908));
    }
}
