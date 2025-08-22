use std::{
    collections::HashMap,
    ops::{AddAssign, BitOr},
};

use advent_of_code::shared::{
    CardinalCoord, CardinalDirection, CardinalShift, Grid, GridBounds, RawIndex,
};

advent_of_code::solution!(12);

const DEBUG: bool = false;

pub fn part_one(input: &str) -> Option<u64> {
    let bounds = GridBounds::from_input(input);
    let grid_of_crops = assign_unique_ids_to_crops_and_return_grid(input, &bounds);

    let crops_collected = count_external_faces(&grid_of_crops, &bounds)
        // count each id into area and the number of external faces into perimeter
        .fold(
            HashMap::<usize, (u64, u64)>::new(),
            |mut acc, (id, perimeter_add)| {
                // using entry gives us the mutable references to the values, initalising as 0 if needed
                let (prev_area, prev_perimeter) = acc.entry(id).or_default();
                // equivalent of +=, but works on mut ref
                prev_area.add_assign(1);
                prev_perimeter.add_assign(perimeter_add);
                // then return the hashmap for the next iteration
                acc
            },
        );

    let answer = crops_collected
        .into_iter()
        .map(|(_, (area, perimeter))| area * perimeter)
        .sum();

    Some(answer)
}

/// Iterates over the grid and counts the number of external faces for each crop
fn count_external_faces<'a>(
    grid_of_crops: &Grid<'_, usize>,
    bounds: &'a GridBounds,
) -> impl Iterator<Item = (usize, u64)> {
    grid_of_crops
        .iter()
        .enumerate()
        // convert our raw_index from enumerate into a coordinate
        .map(move |(raw_index, &id)| {
            (
                CardinalCoord::from_raw_ind(raw_index, &bounds).expect("must be in range"),
                id,
            )
        })
        // add in the number of external faces for each tile
        .map(|(coord, id)| {
            let external_faces = grid_of_crops
                .get_map_neighbors_from_coord(coord)
                .iter()
                .filter(|&contents| {
                    // don't forget that the edges of the grid are None,
                    // but still count towards the perimeter
                    contents.is_none_or(|(_, &other_id)| id != other_id)
                })
                .count() as u64;
            (id, external_faces)
        })
}

pub fn part_two(input: &str) -> Option<u64> {
    let bounds = GridBounds::from_input(input);
    let crops = assign_unique_ids_to_crops_and_return_grid(input, &bounds);
    let max_id = crops
        .iter()
        .max()
        .copied()
        .expect("Will already have panicked if input is empty");

    let external_faces_per_shape = crops
        .grid_point_iter()
        .map(|grid_point| {
            let coord = grid_point.loc;
            let id = grid_point.point;
            (coord, id)
        })
        .map(|(coord, id)| {
            let external_faces = get_sides_bitmask(&crops, coord, id);
            let vec_ind = id - 1;
            (
                vec_ind,
                ShapePoint {
                    coord,
                    external_faces,
                },
            )
        })
        // initialise our vec and then slot all the shape points into it
        .fold(
            vec![Vec::<ShapePoint>::new(); max_id],
            |mut acc, (vec_ind, shape_point)| {
                acc[vec_ind].push(shape_point);
                acc
            },
        );

    let score = external_faces_per_shape
        .into_iter()
        .map(shape_vec_to_score)
        .sum();

    Some(score)
}

/// Returns a bitmask of the sides that are external to the crop at the given coordinate
fn get_sides_bitmask(crops: &Grid<'_, usize>, coord: CardinalCoord<'_>, crop_id: usize) -> u64 {
    CardinalDirection::ALL
        .into_iter()
        .flat_map(|dir| {
            let next_coord = coord.shift(dir);
            if next_coord.is_none() {
                // if it is a grid boundary, this direction is an external face
                // therefore part of a side
                return Some(direction_to_key(dir));
            }
            next_coord
                // we know it is some valid coord, so this will map to a crop id
                .and_then(|coord| crops.get_from_coord(coord))
                // which we can then compare to our current crop id
                .and_then(|&other_id| {
                    if other_id != crop_id {
                        // if it is not the same crop id, then this is an external face
                        Some(direction_to_key(dir))
                    } else {
                        None
                    }
                })
        })
        // then we can fold our bits into a bitmask and return
        .fold(0, |acc, next| BitOr::bitor(acc, next))
}

/// Converts a vector of shape points into a score based on the number of sides and area
fn shape_vec_to_score(mut shape_vec: Vec<ShapePoint<'_>>) -> u64 {
    // make a counter for the number of sides we have seen
    let mut side_counter = 0;
    // make a quick lookup for (coordinate in the shape) -> vec index
    let in_shape: HashMap<CardinalCoord, usize> = shape_vec
        .iter()
        .enumerate()
        .map(|(ind, shape_point)| (shape_point.coord, ind))
        .collect();
    // now work our way through each point in the shape, and count all sides it is part of
    for point_ind in 0..shape_vec.len() {
        // because shape_vec is mutable, we have to be careful with references
        let point_coord = shape_vec[point_ind].coord;
        let point_external_faces = shape_vec[point_ind].external_faces;
        if point_external_faces & KEY_BITMASK != 0 {
            // it has external sides that we have not looked at yet
            for (dir, key) in DIR_KEY_PAIRS {
                if point_external_faces & key != 0 {
                    // we have a side in this direction
                    side_counter += 1;
                    // then we need to remove all the parts of this side
                    remove_side(&mut shape_vec, &in_shape, point_ind, point_coord, dir, key);
                }
            }
        }
    }
    // score is number of sides * area
    side_counter * shape_vec.len() as u64
}

/// Removes the side of a shape in the given direction, starting from the point at point_ind
fn remove_side(
    shape_vec: &mut Vec<ShapePoint<'_>>,
    shape_vec_map: &HashMap<CardinalCoord<'_>, usize>,
    point_ind: usize,
    point_coord: CardinalCoord<'_>,
    dir: CardinalDirection,
    key: u64,
) {
    let left_dir = dir.turn_anti_clockwise();
    let right_dir = dir.turn_clockwise();
    // first left
    remove_line_in_direction(shape_vec, shape_vec_map, point_coord, key, left_dir);
    // then right
    remove_line_in_direction(shape_vec, shape_vec_map, point_coord, key, right_dir);
    // then finally our starting coord
    shape_vec[point_ind].external_faces &= !key;
}

/// Removes the line in the given direction from the shape vector, starting from the point_coord.
/// Useful helper function to avoid code duplication.
fn remove_line_in_direction(
    shape_vec: &mut Vec<ShapePoint<'_>>,
    shape_vec_map: &HashMap<CardinalCoord<'_>, usize>,
    starting_position: CardinalCoord<'_>,
    key: u64,
    dir: CardinalDirection,
) {
    let mut pointer_coord = starting_position;
    while let Some(next_coord) = pointer_coord.shift(dir) {
        // make sure that the next coordinate we check is contained within this shape
        let Some(&shape_vec_ind) = shape_vec_map.get(&next_coord) else {
            break;
        };

        // ensure next point does continues the side
        let next_ext_faces = shape_vec[shape_vec_ind].external_faces;
        if next_ext_faces & key == 0 {
            break;
        }

        // set pointer and then unset the current point
        pointer_coord = next_coord;
        shape_vec[shape_vec_ind].external_faces &= !key;
    }
}

const fn direction_to_key(direction: CardinalDirection) -> u64 {
    use CardinalDirection::*;
    match direction {
        Up => 1 << 63,
        Down => 1 << 62,
        Left => 1 << 61,
        Right => 1 << 60,
    }
}

const KEY_BITMASK: u64 = 1 << 63 | 1 << 62 | 1 << 61 | 1 << 60;

const DIR_KEY_PAIRS: [(CardinalDirection, u64); 4] = {
    let mut init = [(CardinalDirection::Up, 0); 4];
    let mut i = 0;
    while i < 4 {
        let dir = CardinalDirection::ALL[i];
        init[i] = (dir, direction_to_key(dir));
        i += 1;
    }
    init
};

#[derive(Debug, Clone, Copy)]
struct ShapePoint<'a> {
    coord: CardinalCoord<'a>,
    external_faces: u64,
}

fn parse_input(input: &str) -> Vec<char> {
    input.lines().flat_map(|line| line.chars()).collect()
}

/// Assigns unique IDs to each crop in the grid and returns a new grid with these IDs.
/// Almost a map of crops as chars to crops as usize IDs.
fn assign_unique_ids_to_crops_and_return_grid<'a>(
    input: &str,
    bounds: &'a GridBounds,
) -> Grid<'a, usize> {
    let mut counter = 0;
    let initial_grid = Grid::new(parse_input(input), &bounds);
    let mut with_unique_ids = Grid::new(vec![0_usize; bounds.max_col * bounds.max_row], &bounds);
    for grid_point in initial_grid.grid_point_iter() {
        let current_coord: CardinalCoord = grid_point.loc;
        let current_char = grid_point.point;
        let current_id = with_unique_ids
            .get_from_coord(current_coord)
            .copied()
            .expect("coordinate is in range");
        if current_id == 0 {
            // we have not set this square before, therefore we must be seeing a new char
            counter += 1;
            // find all the positions we need to update
            // side note: would be slightly more efficient to use a HashSet here, but we need ordered iteration
            let mut crop_queue = vec![current_coord];
            let mut queue_pointer = 0;
            while queue_pointer < crop_queue.len() {
                let coord_to_check = crop_queue[queue_pointer];
                let new_neighbors = initial_grid.get_map_neighbors_from_coord(coord_to_check);
                let matching_neighbors = new_neighbors
                    .iter()
                    // filter down to only those of the same crop type
                    .filter_map(|inner| {
                        if inner.is_some_and(|(_, &n_char)| current_char == n_char) {
                            inner.map(|(coord, _)| coord)
                        } else {
                            None
                        }
                    })
                    // filter out any we have found already
                    .filter(|neighbour| !crop_queue.contains(neighbour))
                    // collect into a vector to release the borrow of crop_queue
                    .collect::<Vec<_>>();
                // then add all to the queue
                crop_queue.extend(matching_neighbors);
                // and increment the pointer
                queue_pointer += 1;
            }
            // now we can update them all
            for new_coordinate in crop_queue {
                with_unique_ids.set_at_coord(new_coordinate, counter);
            }
            if DEBUG {
                // print the changes for debugging
                for ind in 0..10 {
                    let lower = ind * 10;
                    let upper = (ind + 1) * 10;
                    println!(
                        "{:?}",
                        with_unique_ids
                            .contents
                            .get(lower..upper)
                            .expect("afobsgoiubdsg")
                    );
                    if ind == 9 {
                        println!();
                    }
                }
            }
        }
    }
    with_unique_ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
