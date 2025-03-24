use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (rules_map, pages) = parse_input(input);
    Some(
        pages
            .into_iter()
            // the goal is to take a page set, and return Some(the middle value)
            // if it is valid, or None if it is not
            .filter_map(|page_set| {
                if is_valid_page_order(&page_set, &rules_map) {
                    Some(page_set[page_set.len() / 2])
                } else {
                    None
                }
            })
            .sum(),
    )
}

fn is_valid_page_order(page_set: &Vec<u64>, rules_map: &HashMap<u64, HashSet<u64>>) -> bool {
    (1..page_set.len()).fold(true, |valid, ind| {
        // if we have already invalidated, return early
        if !valid {
            return false;
        }
        // for each index, do any of the pages before it violate a rule?
        // since the first page cannot have any pages before it, no need to check
        let the_page = page_set[ind];
        let previous_pages = page_set
            .get(0..ind)
            .expect("cannot exceed bounds due to being constrained by page_set.len()");
        // if there are rules for this page
        if let Some(page_rules) = rules_map.get(&the_page) {
            // if any are invalid, then we update `valid`
            if page_rules.iter().any(|page_no| {
                // true if invalid
                previous_pages.contains(page_no)
            }) {
                return false;
            }
        }
        valid
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules_map, pages) = parse_input(input);
    Some(
        pages
            .into_iter()
            // just get the incorrectly ordered ones
            .filter_map(|page_set| {
                if !is_valid_page_order(&page_set, &rules_map) {
                    Some(page_set)
                } else {
                    None
                }
            })
            // sort them
            .map(|mut invalid_order| {
                invalid_order.sort_by(|a, b| {
                    // check the rules for a
                    if let Some(page_rules) = rules_map.get(a) {
                        // if there are some rules, check if a must go before b
                        if page_rules.contains(b) {
                            return Ordering::Less;
                        }
                    }
                    Ordering::Equal
                });
                // but it is the correct order now
                invalid_order
            })
            // get the middle values
            .map(|valid_order| valid_order[valid_order.len() / 2])
            .sum(),
    )
}

fn parse_input(input: &str) -> (HashMap<u64, HashSet<u64>>, Vec<Vec<u64>>) {
    let mut rule_map: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut pages = Vec::new();
    input.lines().into_iter().for_each(|line| {
        let v_line: Vec<char> = line.chars().collect();
        match v_line.get(2) {
            // a rule
            Some('|') => {
                // get the first number ('aa|bb')
                let num_1 = parse_numeric(v_line[0]).expect("rule format") * 10
                    + parse_numeric(v_line[1]).expect("rule format");
                // get the second number
                let num_2 = parse_numeric(v_line[3]).expect("rule format") * 10
                    + parse_numeric(v_line[4]).expect("rule format");
                // see if we have an entry for that rule yet
                let mut rules_updated = match rule_map.get(&num_1) {
                    // get the current set of pages
                    Some(current_rule) => current_rule.clone(),
                    // initiate a new set of pages
                    None => HashSet::new(),
                };
                // insert the new num
                let _ = rules_updated.insert(num_2);
                // insert the updated hashset
                let _ = rule_map.insert(num_1, rules_updated);
            }
            // a list of pages
            Some(',') => {
                let page_list: Vec<u64> = v_line
                    .into_iter()
                    // filter out the commas
                    .filter(|&ch| ch != ',')
                    // fun new tool for grouping up an iterator
                    .batching(|it| match it.next() {
                        // if we have another character to pass
                        Some(x) if parse_numeric(x).is_some() => match it.next() {
                            // check to ensure we have two
                            Some(y) if parse_numeric(y).is_some() => Some(
                                // return the 2-digit number they make
                                parse_numeric(x).expect("page number format") * 10
                                    + parse_numeric(y).expect("page number format"),
                            ),
                            _ => None,
                        },
                        _ => None,
                        // otherwise return none, ending the iterator
                    })
                    .collect();
                pages.push(page_list);
            }
            // anything else (but probably just the new line separator)
            _ => {}
        }
    });
    (rule_map, pages)
}

fn parse_numeric(ch: char) -> Option<u64> {
    let num: u64 = match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => return None,
    };
    Some(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
