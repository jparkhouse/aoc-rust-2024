use itertools::Itertools;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    // find some muls to narrow our search
    let muls = input
        .chars()
        .enumerate()
        .tuple_windows::<(_, _, _)>()
        .filter_map(|((ind, ch_0), (_, ch_1), (_, ch_2))| {
            if is_mul(&[ch_0, ch_1, ch_2]) {
                Some(ind)
            } else {
                None
            }
        });

    // then assemble each mul with its proceeding characters for checking
    let all_chars: Vec<char> = input.chars().collect();
    let mul_slices = muls.map(|ind| {
        let max = match ind + 12 < all_chars.len() {
            true => ind + 12,
            false => all_chars.len(),
        };
        let mut vec = Vec::with_capacity(12);
        vec.extend_from_slice(&all_chars[ind..max]);
        vec
    });

    let valid_pairs = mul_slices.filter_map(|vec| {
        // we know from previous filtering that the first 3 characters are already 'm', 'u', 'l'
        // then we need an opening bracket
        let _ = match vec.get(3) {
            Some(ch) if is_opening_bracket(ch) => ch,
            _ => {
                return None;
            }
        };

        // at least one number char
        let mut count = 0;
        while let Some(ch) = vec.get(4 + count) {
            if is_number(ch) {
                count += 1
            } else {
                break;
            }
        }

        // need at least one
        if count == 0 {
            return None;
        }
        // but no more than 3
        if count > 3 {
            return None;
        }

        // parse the number we just collected
        let first_num_string = vec
            .get(4..4 + count)
            .expect("must exist")
            .into_iter()
            .join("");
        let first_num =
            u64::from_str_radix(first_num_string.as_str(), 10).expect("made from number chars");

        // test for a comma next
        let _ = match vec.get(4 + count) {
            Some(ch) if is_comma(ch) => ch,
            _ => {
                return None;
            }
        };

        // stash count in a new starting index, then search for the second number
        let starting_index = 4 + count + 1;
        // at least one number char to make the second number
        let mut count = 0;
        while let Some(ch) = vec.get(starting_index + count) {
            if is_number(ch) {
                count += 1
            } else {
                break;
            }
        }

        // need at least one
        if count == 0 {
            return None;
        }
        // but no more than 3
        if count > 3 {
            return None;
        }

        // parse the number we just collected
        let second_num_string = vec
            .get(starting_index..starting_index + count)
            .expect("must exist")
            .into_iter()
            .join("");
        let second_num =
            u64::from_str_radix(second_num_string.as_str(), 10).expect("made from number chars");

        // finally check for the closing bracket
        let _ = match vec.get(starting_index + count) {
            Some(ch) if is_closing_bracket(ch) => ch,
            _ => {
                return None;
            }
        };

        // and return our pair of numbers
        Some([first_num, second_num])
    });

    let output = valid_pairs.fold(0, |acc, [a, b]| acc + (a * b));

    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    // instructions to look for
    enum Instruction {
        Do,
        Dont,
        Mul,
    }

    // find muls, dos and donts to narrow our search
    let muls: Vec<(usize, Instruction)> = input
        .chars()
        .enumerate()
        .tuple_windows()
        .filter_map(
            |((ind, ch_0), (_, ch_1), (_, ch_2))| match (ch_0, ch_1, ch_2) {
                ('m', 'u', 'l') => Some((ind, Instruction::Mul)),
                _ => None,
            },
        )
        .collect();
    let dos: Vec<(usize, Instruction)> = input
        .chars()
        .enumerate()
        .tuple_windows()
        .filter_map(|((ind, ch_0), (_, ch_1), (_, ch_2), (_, ch_3))| {
            match (ch_0, ch_1, ch_2, ch_3) {
                ('d', 'o', '(', ')') => Some((ind, Instruction::Do)),
                _ => None,
            }
        })
        .collect();
    let donts: Vec<(usize, Instruction)> =
        input
            .chars()
            .enumerate()
            .tuple_windows()
            .filter_map(
                |(
                    (ind, ch_0),
                    (_, ch_1),
                    (_, ch_2),
                    (_, ch_3),
                    (_, ch_4),
                    (_, ch_5),
                    (_, ch_6),
                )| match (ch_0, ch_1, ch_2, ch_3, ch_4, ch_5, ch_6) {
                    ('d', 'o', 'n', '\'', 't', '(', ')') => Some((ind, Instruction::Dont)),
                    _ => None,
                },
            )
            .collect();

    // now combine to big list
    let mut all_instructions: Vec<(usize, Instruction)> =
        Vec::with_capacity(muls.len() + dos.len() + donts.len());
    all_instructions.extend(muls);
    all_instructions.extend(dos);
    all_instructions.extend(donts);
    all_instructions.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
    
    // then assemble each mul with its proceeding characters for checking
    // remembering to check the do_dont_state
    let mut do_dont_state = true;
    let all_chars: Vec<char> = input.chars().collect();
    let valid_pairs = all_instructions.into_iter().filter_map(|(ind, inst)| {
        match inst {
            Instruction::Do => {
                do_dont_state = true;
                None
            }
            Instruction::Dont => {
                do_dont_state = false;
                None
            }
            Instruction::Mul => {
                if do_dont_state {
                    let max = match ind + 12 < all_chars.len() {
                        true => ind + 12,
                        false => all_chars.len(),
                    };
                    let mut vec = Vec::with_capacity(12);
                    vec.extend_from_slice(&all_chars[ind..max]);
                    // we know from previous filtering that the first 3 characters are already 'm', 'u', 'l'
                    // then we need an opening bracket
                    let _ = match vec.get(3) {
                        Some(ch) if is_opening_bracket(ch) => ch,
                        _ => {
                            return None;
                        }
                    };

                    // at least one number char
                    let mut count = 0;
                    while let Some(ch) = vec.get(4 + count) {
                        if is_number(ch) {
                            count += 1
                        } else {
                            break;
                        }
                    }

                    // need at least one
                    if count == 0 {
                        return None;
                    }
                    // but no more than 3
                    if count > 3 {
                        return None;
                    }

                    // parse the number we just collected
                    let first_num_string = vec
                        .get(4..4 + count)
                        .expect("must exist")
                        .into_iter()
                        .join("");
                    let first_num = u64::from_str_radix(first_num_string.as_str(), 10)
                        .expect("made from number chars");

                    // test for a comma next
                    let _ = match vec.get(4 + count) {
                        Some(ch) if is_comma(ch) => ch,
                        _ => {
                            return None;
                        }
                    };

                    // stash count in a new starting index, then search for the second number
                    let starting_index = 4 + count + 1;
                    // at least one number char to make the second number
                    let mut count = 0;
                    while let Some(ch) = vec.get(starting_index + count) {
                        if is_number(ch) {
                            count += 1
                        } else {
                            break;
                        }
                    }

                    // need at least one
                    if count == 0 {
                        return None;
                    }
                    // but no more than 3
                    if count > 3 {
                        return None;
                    }

                    // parse the number we just collected
                    let second_num_string = vec
                        .get(starting_index..starting_index + count)
                        .expect("must exist")
                        .into_iter()
                        .join("");
                    let second_num = u64::from_str_radix(second_num_string.as_str(), 10)
                        .expect("made from number chars");

                    // finally check for the closing bracket
                    let _ = match vec.get(starting_index + count) {
                        Some(ch) if is_closing_bracket(ch) => ch,
                        _ => {
                            return None;
                        }
                    };

                    // and return our pair of numbers
                    Some([first_num, second_num])
                } else {
                    None
                }
            }
        }
    });

    let output = valid_pairs.fold(0, |acc, [a, b]| acc + (a * b));

    Some(output)
}

fn is_mul(mul: &[char; 3]) -> bool {
    mul[0] == 'm' && mul[1] == 'u' && mul[2] == 'l'
}

fn is_opening_bracket(ch: &char) -> bool {
    *ch == '('
}

fn is_closing_bracket(ch: &char) -> bool {
    *ch == ')'
}

fn is_comma(ch: &char) -> bool {
    *ch == ','
}

fn is_number(ch: &char) -> bool {
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
