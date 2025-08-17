use std::collections::HashMap;

use advent_of_code::shared::{CardinalCoord, CardinalDirection, CardinalShift, Grid, GridBounds, RawIndex};

advent_of_code::solution!(12);

const DEBUG: bool = false;

pub fn part_one(input: &str) -> Option<u64> {
    let bounds = GridBounds::from_input(input);
    let grid_of_crops = assign_unique_ids_to_crops_and_return_grid(input, &bounds);

    let crops_collected = count_external_faces(&grid_of_crops, &bounds)
        // count each id into area and the number of external faces into perimeter
        .fold(HashMap::<usize, (u64, u64)>::new(), |mut acc, (id, perimeter_add)| {
            let (prev_area, prev_perimeter) = acc.entry(id)
                .or_default();
            *prev_area += 1;
            *prev_perimeter += perimeter_add;
            acc
        });

    let answer = crops_collected.into_iter()
        .map(|(id, (area, perimeter))| {
            let score = area * perimeter;
            if DEBUG {
                println!("id {}: area of {}, perimeter of {}, total score {}",
                    id,
                    area,
                    perimeter,
                    score
                );
            }
            score
        })
        .sum();

    Some(answer)
}

fn count_external_faces<'a>(grid_of_crops: &Grid<'_, usize>, bounds: &'a GridBounds) -> impl Iterator<Item = (usize, u64)> {
    grid_of_crops.iter()
        .enumerate()
        // convert our raw_index from enumerate into a coordinate
        .map(move |(raw_index, &id)| {
            (CardinalCoord::from_raw_ind(raw_index, &bounds).expect("must be in range"), id)
        })
        // add in the number of external faces for each tile
        .map(|(coord, id)| {
            let external_faces = grid_of_crops.get_map_neighbors_from_coord(coord)
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
    let max_id = crops.iter()
        .max()
        .copied()
        .expect("Will already have panicked if input is empty");

    let external_faces_per_shape = crops.iter()
        .enumerate()
        .map(|(raw_index, id)| {
            let coord = CardinalCoord::from_raw_ind(raw_index, &bounds)
                .expect("came from in bounds, so must be in bounds");
            let vec_ind = id - 1;
            (coord, vec_ind)
        })
        .map(|(coord, vec_ind)| {
            let external_faces = CardinalDirection::ALL
                .into_iter()
                .flat_map(|dir| {
                    let new_coord = match coord.shift(dir) {
                        Some(coord) => coord,
                        None => return Some(direction_to_key(dir))
                    };
                    if let Some(id) = crops.get_from_coord(new_coord) {
                        if id - 1 != vec_ind {
                            return Some(direction_to_key(dir))
                        }
                    }
                    return None
                })
                .fold(0, |acc, next| acc | next);
            (vec_ind, ShapePoint { coord, external_faces })
        })
        .fold(vec![Vec::<ShapePoint>::new(); max_id],
            |mut acc, (vec_ind, shape_point)| {
                acc[vec_ind].push(shape_point);
                acc
            }
        );

    let score = external_faces_per_shape.into_iter()
        .map(|mut shape_vec| {
            // because shape_vec is mutable, we have to be careful with references
            // make a counter for the number of sides we have seen
            let mut side_counter = 0;
            // make a quick lookup for if a coordinate is in the shape
            let in_shape: HashMap<CardinalCoord, usize> = shape_vec.iter()
                .enumerate()
                .map(|(ind, shape_point)| (shape_point.coord, ind))
                .collect();
            // now work our way through each point in the shape, and count all sides it is part of
            for point_ind in 0 .. shape_vec.len() {
                let point_coord = shape_vec[point_ind].coord;
                let point_external_faces = shape_vec[point_ind].external_faces;
                if shape_vec[point_ind].external_faces & KEY_BITMASK != 0 {
                    // it has external sides that we have not looked at yet
                    for (dir, key) in CardinalDirection::ALL.map(|dir| (dir, direction_to_key(dir))) {
                        if point_external_faces & key != 0 {
                            // we have a side in this direction
                            side_counter += 1;
                            // then we need to remove all the parts of this side length
                            let left_dir = dir.turn_anti_clockwise();
                            let right_dir = dir.turn_clockwise();
                            // first left
                            let mut pointer_coord = Some(point_coord);
                            while let Some(coord) = pointer_coord.take() {
                                // make sure that the next coordinate we check is in bounds
                                let Some(next_coord) = coord.shift(left_dir) else { break };

                                // make sure that the next coordinate we check is contained within this shape
                                let Some(&shape_vec_ind) = in_shape.get(&next_coord) else { break };

                                // ensure next point does continues the side
                                let next_ext_faces = shape_vec[shape_vec_ind].external_faces;
                                if next_ext_faces & key == 0 { break }
                                
                                // set pointer and then unset the current point
                                pointer_coord = Some(next_coord);
                                shape_vec[shape_vec_ind].external_faces &= !key;
                            }
                            // then right
                            let mut pointer_coord = Some(point_coord);
                            while let Some(coord) = pointer_coord.take() {
                                // make sure that the next coordinate we check is in bounds
                                let Some(next_coord) = coord.shift(right_dir) else { break };

                                // make sure that the next coordinate we check is contained within this shape
                                let Some(&shape_vec_ind) = in_shape.get(&next_coord) else { break };

                                // ensure next point does continues the side
                                let next_ext_faces = shape_vec[shape_vec_ind].external_faces;
                                if next_ext_faces & key == 0 { break }
                                
                                // set pointer and then unset the current point
                                pointer_coord = Some(next_coord);
                                shape_vec[shape_vec_ind].external_faces &= !key;
                            }
                            // then finally our starting coord
                            shape_vec[point_ind].external_faces &= !key;
                        }
                    }
                }
            }
            // score is number of sides * area
            side_counter * shape_vec.len() as u64
        })
        .sum();
        
    Some(score)
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

#[derive(Debug, Clone, Copy)]
struct ShapePoint<'a> {
    coord: CardinalCoord<'a>,
    external_faces: u64
}

fn parse_input(input: &str) -> Vec<char> {
    input.lines()
        .flat_map(|line| line.chars())
        .collect()
}

fn assign_unique_ids_to_crops_and_return_grid<'a>(input: &str, bounds: &'a GridBounds) -> Grid<'a, usize> {
    let mut counter = 0;
    let initial_grid = Grid::new(parse_input(input), &bounds);
    let mut with_unique_ids = Grid::new(vec![0_usize; bounds.max_col * bounds.max_row], &bounds);
    for (raw_ind, &current_char) in initial_grid.iter().enumerate() {
        let current_coord = CardinalCoord::from_raw_ind(raw_ind, &bounds)
            .expect("must be valid coord");
        let current_id = with_unique_ids.get_from_coord(current_coord)
            .copied()
            .expect("coordinate is in range");
        if current_id == 0 {
            // we have not set this square before, therefore we must be seeing a new char
            counter += 1;
            // find all the positions we need to update
            let mut crop_queue = vec![current_coord];
            let mut queue_pointer = 0;
            while queue_pointer < crop_queue.len() {
                let coord_to_check = crop_queue[queue_pointer];
                let new_neighbors = initial_grid.get_map_neighbors_from_coord(coord_to_check);
                let matching_neighbors = new_neighbors.iter()
                    .filter_map(|inner| {
                        inner.map(|(coord, &n_char)| {
                            if current_char == n_char {
                                Some(coord)
                            } else {
                                None
                            }
                        })
                        .flatten()
                    });
                for new_coord in matching_neighbors {
                    if !crop_queue.contains(&new_coord) {
                        crop_queue.push(new_coord);
                    }
                }
                queue_pointer += 1;
            }
            // now we can update them all
            for new_coordinate in crop_queue {
                with_unique_ids.set_at_coord(new_coordinate, counter);
            }
            if DEBUG {
                // print the changes for debugging
                for ind in 0 .. 10 {
                    let lower = ind * 10;
                    let upper = (ind + 1) * 10;
                    println!("{:?}", with_unique_ids.contents.get(lower .. upper).expect("afobsgoiubdsg"));
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
