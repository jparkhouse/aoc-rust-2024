advent_of_code::solution!(10);

use std::collections::HashSet;

use advent_of_code::shared::{CardinalCoord as Coord, GridBounds, RawIndex};

const ALL_HEIGHTS: [Height; 10] = {
    use Height::*;
    [
        Zero,
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine
    ]
};

pub fn part_one(input: &str) -> Option<u64> {
    let bounds = GridBounds::from_input(input);
    let map = parse_input_to_map(input, &bounds);

    let trail_heads = find_trail_heads(&map);
    let mut all_paths: Option<Vec<HashSet<MapPoint>>> = None;
    // since we start at Zero, we need to skip 1 and start our search from One
    for height in ALL_HEIGHTS.iter().skip(1) {
        match all_paths {
            // this will be our first run, so we need to work from the trail_heads vec and initialise all_paths
            None => {
                all_paths = Some(
                    // for each trail head
                    trail_heads.iter()
                        // we must start a new HashSet of valid paths (since we want to dedupe locations)
                        .map(|trail_head| {
                            find_valid_paths(
                                &map,
                                height,
                                // we must clone the trail_head because rust is silly
                                // and worries we might come back to it later
                                trail_head.clone()
                            )
                            .into_iter()
                            .collect()
                        })
                        .collect()
                )
            }
            // otherwise, we already have some paths, and we need to check for the next height
            Some(paths) => {
                all_paths = Some(
                    paths.into_iter()
                        // for each trail head's possible paths
                        .map(|trail_head_paths| {
                            // check each point
                            trail_head_paths.into_iter()
                                // and flatten all possible paths back into the trail head's HashSet, deduping locations
                                .flat_map(|mp| {
                                    find_valid_paths(&map, height, mp)
                                })
                                .collect()
                        })
                        .collect()
                    )
                }
            }

        if let Some(paths) = all_paths {
            // filter out any trail heads that did not survive the last round
            all_paths = Some(paths.into_iter().filter(|set| !set.is_empty()).collect());
        }
    }
    
    let scores: Vec<u64> = all_paths.expect("must be initialised by now")
        .into_iter()
        // a trail head's score is equal to the number of Nines it can reach
        .map(|set| set.len() as u64)
        .collect();

    // our final answer is the sum of the scores of all trail heads
    Some(scores.into_iter().sum())
}

fn find_valid_paths<'a>(map: &'a Map, target_height: &Height, mp: MapPoint<'a>) -> Vec<MapPoint<'a>> {
    // for each point from the original trail head, we need to get its neighbours
    let possible_paths = map.get_map_neighbors_from_coord(mp.loc);
    // maximum possible paths are 3 (since we must have come from one of them)
    // unless we are starting at a trail head
    let mut output = Vec::with_capacity(4);
    for path in possible_paths.iter() {
        // if it is a valid path, i.e. does not leave the grid
        if let Some(new_mp) = path {
            // and it is exactly one step up in height (from the original iterator)
            if *new_mp.1 == *target_height {
                // add it to the output
                output.push(MapPoint::from(new_mp.0,*new_mp.1));
            }
        }
    }
    // this will then be unpacked for the next run, so that even if there are more than one paths away
    // from a point, we track all of them
    // dead paths are just ignored
    output
}

pub fn part_two(input: &str) -> Option<u64> {
    let bounds = GridBounds::from_input(input);
    let map = parse_input_to_map(input, &bounds);

    let trail_heads = find_trail_heads(&map);
    let mut all_paths: Option<Vec<Vec<MapPoint>>> = None;
    // since we start at Zero, we need to skip 1 and start our search from One
    for height in ALL_HEIGHTS.iter().skip(1) {
        match all_paths {
            // this will be our first run, so we need to work from the trail_heads vec and initialise all_paths
            None => {
                all_paths = Some(
                    // for each trail head
                    trail_heads.iter()
                        // we must start a new Vec of valid paths
                        .map(|trail_head| {
                            find_valid_paths(
                                &map,
                                height,
                                // we must clone the trail_head because rust is silly
                                // and worries we might come back to it later
                                trail_head.clone()
                            )
                        })
                        .collect()
                )
            }
            // otherwise, we already have some paths, and we need to check for the next height
            Some(paths) => {
                all_paths = Some(
                    paths.into_iter()
                        // for each trail head's possible paths
                        .map(|trail_head_paths| {
                            // check each point
                            trail_head_paths.into_iter()
                                // and flatten all possible paths back into the trail head's vec
                                .flat_map(|mp| {
                                    find_valid_paths(&map, height, mp)
                                })
                                .collect()
                        })
                        .collect()
                    )
                }
            }

        if let Some(paths) = all_paths {
            // filter out any trail heads that did not survive the last round
            all_paths = Some(paths.into_iter().filter(|vec| !vec.is_empty()).collect());
        }
    }
    
    let scores: Vec<u64> = all_paths.expect("must be initialised by now")
        .into_iter()
        // a trail head's score is equal to the number of Nines it can reach
        .map(|vec| vec.len() as u64)
        .collect();

    // our final answer is the sum of the scores of all trail heads
    Some(scores.into_iter().sum())
}

fn parse_input_to_map<'a>(input: &str, grid_bounds: &'a GridBounds) -> Map<'a> {
    let grid: Vec<Height> = input.lines()
        .flat_map(|line| line.chars().map(|ch| char_to_height(ch)))
        .collect();
    Map { contents: grid, grid_bounds }
} 

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Height {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

fn char_to_height(ch: char) -> Height {
    use Height::*;
    match ch {
        '0' => Zero,
        '1' => One,
        '2' => Two,
        '3' => Three,
        '4' => Four,
        '5' => Five,
        '6' => Six,
        '7' => Seven,
        '8' => Eight,
        '9' => Nine,
        _ => panic!("invalid input"),
    }
}

type Map<'a> = advent_of_code::shared::Grid<'a, Height>;
type MapPoint<'a> = advent_of_code::shared::GridPoint<'a, Coord<'a>, Height>;


/// Filters the grid to return a `Vec` of `MapPoint`s for all locations in the grid
/// where the height is `Zero`, defined as a trail head in the problem
fn find_trail_heads<'gb>(map: &'gb Map<'gb>) -> Vec<MapPoint<'gb>> {
    map.iter()
        .enumerate()
        .filter_map(|(ind, &height)| {
            (height == Height::Zero)
                .then(|| {
                        MapPoint::from(
                            Coord::from_raw_ind(ind, map.grid_bounds)
                            .expect("usize from iter enumerate must be in range"), 
                            height
                        )
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}