use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let debug = false;
    let (lab_width, lab_hieght) = get_lab_dimensions(input);

    let (lab, guard) = parse_input(input, &lab_width, &lab_hieght);
    let all_visited_locs = get_finite_path_information(guard, &lab, debug);
    Some(all_visited_locs.len() as u64)
}

fn parse_input<'a>(
    input: &str,
    lab_width: &'a usize,
    lab_height: &'a usize,
) -> (LabGrid<'a>, Guard<'a>) {
    // use helper function to generate our lab
    let lab = build_lab_grid_from_str(input, &lab_width, &lab_height);
    // find the guard in the lab, and then initialise a Guard instance
    let (guard_starting_position, _) = input
        .lines()
        .flat_map(|line| line.chars())
        .enumerate()
        .find(|(_, ch)| *ch == '^')
        .expect("valid input must have a guard");
    let guard = Guard::new(Coord::from_raw_ind(
        guard_starting_position,
        &lab_width,
        &lab_height,
    ));
    (lab, guard)
}

fn get_lab_dimensions(input: &str) -> (usize, usize) {
    // collect our static grid size references
    let lab_height = input.lines().count();
    let lab_width = input
        .lines()
        .next()
        .expect("input should not be empty")
        .chars()
        .count();
    (lab_height, lab_width)
}

fn get_finite_path_information(
    guard: Guard<'_>,
    lab: &LabGrid<'_>,
    debug: bool,
) -> HashMap<(usize, usize), LocationHistory> {
    // initialise next_obstacle
    let mut next_obstacle = get_next_obst(&lab, &guard);
    let mut guard: Result<Guard<'_>, HashMap<(usize, usize), LocationHistory>> = Ok(guard);
    loop {
        guard = match guard {
            Err(e) => {
                // if guard has left the bounds, then we can stop
                // and pass the err on for future processing
                Err(e)
            }
            Ok(mut inner_guard) => {
                // otherwise we should make the guard do something
                let mut path_clear = false;
                while !path_clear {
                    if let Some(obst) = next_obstacle {
                        if inner_guard.is_blocked_by(obst) {
                            if debug {
                                println!("guard has hit obstacle");
                                display_grid(
                                    &inner_guard,
                                    &lab,
                                    inner_guard.location.col_height,
                                    inner_guard.location.row_width,
                                );
                            }
                            inner_guard.rotate();
                            next_obstacle = get_next_obst(&lab, &inner_guard);
                        } else {
                            // path not yet blocked, still some distant object
                            path_clear = true;
                        }
                    } else {
                        // path not blocked, no more objects
                        path_clear = true;
                    }
                }
                // then we march forwards
                inner_guard.move_one_step()
            }
        };

        if guard.is_err() {
            break;
        }
    }
    guard.expect_err("only breaks from loop if err")
}

fn display_grid(guard: &Guard, lab: &LabGrid, lab_height: &usize, lab_width: &usize) {
    // initialise grid
    let mut grid = vec!['.'; lab_height * lab_width];
    // populate with obstacles
    lab.layout.iter().for_each(|obst_coord| {
        grid[obst_coord.to_raw_ind()] = '#';
    });
    // show all the places the guard has been
    guard.visited.iter().for_each(|((row, col), _)| {
        let raw_ind = row * lab_width + col;
        match grid.get(raw_ind) {
            Some('#') => {
                panic!("Guard has collided with an obstacle at ({}, {})", row, col);
            }
            Some('.') => grid[raw_ind] = 'X',
            None => {
                panic!(
                    "Guard visited a position out of bounds at ({}, {})",
                    row, col
                );
            }
            _ => {}
        }
    });
    // display the guard themselves
    let guard_char = match guard.direction {
        Direction::Left => 'L',
        Direction::Right => 'R',
        Direction::Up => 'U',
        Direction::Down => 'D',
    };
    let guard_raw_ind = guard.location.to_raw_ind();
    let (guard_row, guard_col) = (guard.location.row, guard.location.col);
    match grid.get(guard_raw_ind) {
        Some('.') | Some('X') => grid[guard_raw_ind] = guard_char,
        Some('#') => {
            panic!(
                "Guard has collided with an obstacle at ({}, {})",
                guard_row, guard_col
            );
        }
        None => {
            panic!(
                "Guard visited a position out of bounds at ({}, {})",
                guard_row, guard_col
            );
        }
        _ => {}
    }
    // finally display the grid
    (0..*lab_height).for_each(|row_ind| {
        let row = (row_ind * *lab_width..(row_ind + 1) * *lab_width)
            .map(|raw_ind| grid[raw_ind])
            .join(" ");
        println!("{}", row)
    });
    println!("")
}

fn get_next_obst<'a>(lab: &'a LabGrid, guard: &Guard) -> Option<&'a Coord<'a>> {
    use Direction::*;
    let (view_dir, ind) = guard.get_view();
    let (curr_row, curr_col) = guard.get_pos();
    // get the relevant objects for the row or column (depending on view direction)
    let row_col_obsts = match view_dir {
        Left | Right => {
            lab.row_look_up.get(&ind).map(|obsts| {
                obsts
                    .iter()
                    // this only gives us indexes into the lab layout, so we need to get the actual obstacles
                    .filter_map(|&ind| lab.layout.get(ind))
                    .collect::<Vec<_>>()
            })
        }
        Up | Down => {
            lab.col_look_up.get(&ind).map(|obsts| {
                obsts
                    .iter()
                    // this only gives us indexes into the lab layout, so we need to get the actual obstacles
                    .filter_map(|&ind| lab.layout.get(ind))
                    .collect::<Vec<_>>()
            })
        }
    }?;
    let obsts_in_view: Vec<&Coord> = row_col_obsts
        .into_iter()
        .filter(|obst| match view_dir {
            Left => obst.col < curr_col,
            Right => obst.col > curr_col,
            Up => obst.row < curr_row,
            Down => obst.row > curr_row,
        })
        .collect();
    let closest_obj = obsts_in_view
        .into_iter()
        .min_by_key(|obst| match view_dir {
            Left => curr_col - obst.col,
            Right => obst.col - curr_col,
            Up => curr_row - obst.row,
            Down => obst.row - curr_row,
        })?;
    Some(closest_obj)
}

pub fn part_two(input: &str) -> Option<u64> {
    // TODO: make this more performant
    let (lab_width, lab_height) = get_lab_dimensions(input);
    let (mut original_lab, original_guard) = parse_input(input, &lab_width, &lab_height);
    let original_guard_loc: (usize, usize) = original_guard.location.clone().into();
    let potential_object_locations: Vec<(usize, usize)> =
        get_finite_path_information(original_guard.clone(), &original_lab, false)
            .into_keys()
            // filter out the guard's starting location, since we cannot put an object there
            // due to paradoxes
            .filter(|&loc| loc != original_guard_loc)
            .collect();
    let object_locations = potential_object_locations
        .into_iter()
        .filter(|loc| {
            // create a new guard for this timeline
            let guard = original_guard.clone();
            let new_obst = Coord {
                row: loc.0,
                col: loc.1,
                row_width: &lab_width,
                col_height: &lab_height,
            };
            // now we need to calculate the path, returning true if we detect a loop
            // and false if the guard exits the lab before a loop is detected
            with_inserted_obstacle(&mut original_lab, new_obst, |lab| {
                check_for_path_loop(guard, lab)
            })
        })
        .count();
    Some(object_locations as u64)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord<'a> {
    row: usize,
    col: usize,
    row_width: &'a usize,
    col_height: &'a usize,
}

impl<'a> Coord<'a> {
    fn shift_left(self) -> Option<Self> {
        if self.col > 0 {
            Some(Self {
                row: self.row,
                col: self.col - 1,
                row_width: self.row_width,
                col_height: self.col_height,
            })
        } else {
            None
        }
    }

    fn shift_right(self) -> Option<Self> {
        if self.col < self.row_width - 1 {
            Some(Self {
                row: self.row,
                col: self.col + 1,
                row_width: self.row_width,
                col_height: self.col_height,
            })
        } else {
            None
        }
    }

    fn shift_up(self) -> Option<Self> {
        if self.row > 0 {
            Some(Self {
                row: self.row - 1,
                col: self.col,
                row_width: self.row_width,
                col_height: self.col_height,
            })
        } else {
            None
        }
    }

    fn shift_down(self) -> Option<Self> {
        if self.row < self.col_height - 1 {
            Some(Self {
                row: self.row + 1,
                col: self.col,
                row_width: self.row_width,
                col_height: self.col_height,
            })
        } else {
            None
        }
    }

    /// Shifts the coordinate 1 step in the given direction.
    /// Returns an option where None represents leaving the bounds of the grid.
    pub fn shift(self, dir: Direction) -> Option<Self> {
        use Direction::*;
        match dir {
            Left => self.shift_left(),
            Right => self.shift_right(),
            Up => self.shift_up(),
            Down => self.shift_down(),
        }
    }

    pub fn from_raw_ind(raw_ind: usize, grid_width: &'a usize, grid_height: &'a usize) -> Self {
        Self {
            row: raw_ind / *grid_width,
            col: raw_ind % *grid_width,
            row_width: grid_width,
            col_height: grid_height,
        }
    }

    pub fn to_raw_ind(&self) -> usize {
        let row_len = self.row_width.clone();
        self.row * row_len + self.col
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<Coord<'_>> for (usize, usize) {
    fn from(value: Coord<'_>) -> Self {
        (value.row, value.col)
    }
}

#[derive(Debug, Clone)]
struct Guard<'a> {
    location: Coord<'a>,
    direction: Direction,
    visited: HashMap<(usize, usize), LocationHistory>,
}

impl<'a> Guard<'a> {
    pub fn new(starting_location: Coord<'a>) -> Self {
        let mut visited = HashMap::new();
        visited.increment_visit_count(starting_location.into());
        Self {
            location: starting_location,
            direction: Direction::Up,
            visited,
        }
    }

    /// Naively moves one position forwards. Returns Ok(self) unless it has exited the grid,
    /// in which case it returns Err(positions) for further analysis.
    pub fn move_one_step(mut self) -> Result<Self, HashMap<(usize, usize), LocationHistory>> {
        let new_location = match self.location.shift(self.direction) {
            Some(x) => x,
            None => return Err(self.visited),
        };
        self.visited.increment_visit_count(new_location.into());
        Ok(Self {
            location: new_location,
            direction: self.direction,
            visited: self.visited,
        })
    }

    pub fn rotate(&mut self) {
        use Direction::*;
        let new_dir = match self.direction {
            Left => Up,
            Right => Down,
            Up => Right,
            Down => Left,
        };
        self.visited.increment_turning_point(self.get_pos());
        self.direction = new_dir;
    }

    pub fn get_view(&self) -> (Direction, usize) {
        use Direction::*;
        (
            self.direction,
            match self.direction {
                Left | Right => self.location.row,
                Up | Down => self.location.col,
            },
        )
    }

    pub fn get_pos(&self) -> (usize, usize) {
        self.location.into()
    }

    pub fn is_blocked_by(&self, obst: &Coord<'a>) -> bool {
        if let Some(new_pos) = self.location.clone().shift(self.direction) {
            if &new_pos == obst {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct LabGrid<'a> {
    layout: Vec<Coord<'a>>,
    row_look_up: HashMap<usize, Vec<usize>>,
    col_look_up: HashMap<usize, Vec<usize>>,
}

#[derive(Debug, Default, Clone, Copy)]
struct LocationHistory {
    visit_count: u8,
    turning_point: u8,
    direction_hist: [bool; 4],
}

impl LocationHistory {
    pub fn increment_turning_point(&mut self) {
        self.turning_point += 1
    }

    pub fn increment_visit_count(&mut self) {
        self.visit_count += 1
    }

    pub fn add_direction_history(&mut self, dir: Direction) {
        use Direction::*;
        let prev_dir_hist = self.direction_hist;
        let new_dir_hist = [
            if dir == Left { true } else { prev_dir_hist[0] },
            if dir == Right { true } else { prev_dir_hist[1] },
            if dir == Up { true } else { prev_dir_hist[2] },
            if dir == Down { true } else { prev_dir_hist[3] },
        ];
        self.direction_hist = new_dir_hist;
    }

    pub fn check_for_direction(&self, dir: Direction) -> bool {
        use Direction::*;
        match dir {
            Left => self.direction_hist[0],
            Right => self.direction_hist[1],
            Up => self.direction_hist[2],
            Down => self.direction_hist[3],
        }
    }
}

trait IncrementLocationHistory {
    type Key;
    fn increment_turning_point(&mut self, key: Self::Key);
    fn increment_visit_count(&mut self, key: Self::Key);
    fn add_direction_history(&mut self, key: Self::Key, dir: Direction);
}

impl IncrementLocationHistory for HashMap<(usize, usize), LocationHistory> {
    fn increment_turning_point(&mut self, key: Self::Key) {
        self.entry(key).or_default().increment_turning_point()
    }

    fn increment_visit_count(&mut self, key: Self::Key) {
        self.entry(key).or_default().increment_visit_count()
    }

    type Key = (usize, usize);

    fn add_direction_history(&mut self, key: Self::Key, dir: Direction) {
        self.entry(key).or_default().add_direction_history(dir)
    }
}

fn build_lab_grid_from_str<'a>(
    input: &str,
    row_width: &'a usize,
    col_height: &'a usize,
) -> LabGrid<'a> {
    let rows: Vec<&str> = input.lines().collect();
    let layout: Vec<Coord> = rows
        .into_iter()
        .flat_map(|row| row.chars())
        .enumerate()
        .filter_map(|(ind, ch)| {
            if ch == '#' {
                Some(Coord::from_raw_ind(ind, row_width, col_height))
            } else {
                None
            }
        })
        .collect();

    let mut row_look_up: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut col_look_up: HashMap<usize, Vec<usize>> = HashMap::new();

    layout.iter().enumerate().for_each(|(ind, obst)| {
        let row = obst.row;
        let col = obst.col;

        row_look_up.entry(row).or_default().push(ind);
        col_look_up.entry(col).or_default().push(ind);
    });

    LabGrid {
        layout,
        row_look_up,
        col_look_up,
    }
}

fn check_for_path_loop(mut guard: Guard<'_>, lab: &LabGrid<'_>) -> bool {
    // initialise next_obstacle
    let mut next_obstacle = get_next_obst(&lab, &guard);
    loop {
        let mut path_blocked = true;

        while path_blocked {
            if let Some(obst) = next_obstacle {
                if guard.is_blocked_by(obst) {
                    // println!("guard has hit obstacle {:?}", obst);
                    // display_grid(
                    //    &guard,
                    //     &lab,
                    //     guard.location.col_height,
                    //     guard.location.row_width,
                    // );
                    guard.rotate();
                    // println!("Rotated! Now facing {:?}", guard.direction);
                    next_obstacle = get_next_obst(lab, &guard);
                    // println!("calculated {:?} as next obstacle", next_obstacle);
                } else {
                    path_blocked = false;
                }
            } else {
                path_blocked = false;
            }
        }

        guard = match guard.move_one_step() {
            Ok(next_loc) => {
                // println!(
                //     "Guard has moved to ({}, {})",
                //     next_loc.location.row, next_loc.location.col
                // );
                next_loc
            }
            // the guard has left the valid bounds before a loop has been detected
            Err(_) => {
                println!("Guard has left the area!");
                return false;
            }
        };

        // now we check for a loop at the new location
        // we check the loop by seeing if the guard has visited this location before
        // if we have, and the location direction is the same now as it was at our last visit
        // then because guard movement is deterministic, we must be in a loop

        // cache this
        let current_loc = guard.location.into();
        // check to see if we visited this location before
        if let Some(loc) = guard.visited.get(&current_loc) {
            if loc.check_for_direction(guard.direction) {
                // if we have been to this location before, and faced that direction,
                // then our next steps must be the same, because the guard is deterministic
                // therefore we are in a loop
                println!("Loop detected!");
                return true;
            }
        }

        // update the last visit direction for future visits
        guard
            .visited
            .add_direction_history(current_loc, guard.direction);
    }
}

/// A helper function to run some process on a lab layout with a temporary new obstacle.
/// Inserts the obstacle, runs the process, and then resets the obstacle, before returning
/// the result of the process.
fn with_inserted_obstacle<'a, F, R>(lab: &mut LabGrid<'a>, new_obst: Coord<'a>, f: F) -> R
where
    F: FnOnce(&LabGrid) -> R,
{
    let pointer_ind = lab.layout.len();
    lab.layout.push(new_obst);
    lab.row_look_up
        .entry(new_obst.row)
        .or_default()
        .push(pointer_ind);
    lab.col_look_up
        .entry(new_obst.col)
        .or_default()
        .push(pointer_ind);

    let result = f(lab);

    lab.layout.pop();
    lab.row_look_up.entry(new_obst.row).and_modify(|v| {
        let _ = v.pop();
    });
    lab.col_look_up.entry(new_obst.col).and_modify(|v| {
        let _ = v.pop();
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn returns_true_when_finding_a_loop() {
        // arrange
        let input = advent_of_code::template::read_file("examples", DAY);
        let (w, h) = get_lab_dimensions(&input);

        let (mut lab, guard) = parse_input(&input, &w, &h);
        let (lab_copy, _) = parse_input(&input, &w, &h);
        let new_obst = Coord {
            row: 7,
            col: 6,
            row_width: &w,
            col_height: &h,
        };

        // act
        let output = with_inserted_obstacle(&mut lab, new_obst, |l| check_for_path_loop(guard, &l));

        // assert
        assert_eq!(output, true);
        assert_eq!(lab.layout, lab_copy.layout);
        assert_eq!(lab.row_look_up, lab_copy.row_look_up);
        assert_eq!(lab.col_look_up, lab_copy.col_look_up);
    }

    #[test]
    fn returns_false_when_no_loop_found() {
        // arrange
        let input = advent_of_code::template::read_file("examples", DAY);
        let (w, h) = get_lab_dimensions(&input);
        let (lab, guard) = parse_input(&input, &w, &h);

        // act
        let output = check_for_path_loop(guard, &lab);

        // assert
        assert_eq!(output, false)
    }
}
