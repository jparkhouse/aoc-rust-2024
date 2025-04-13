use advent_of_code::shared::match_numeric;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let debug = false;

    let drive_blocks = parse_input(input);
    let filled_blocks = drive_blocks.iter().filter(|block| block.is_some()).count();
    let mut new_arrangement: Vec<u64> = vec![0; filled_blocks];
    let mut full_block_index_from_back_iter = drive_blocks
        .iter()
        .enumerate()
        .filter_map(|(block_ind, block_contents)| {
            if let Some(_) = block_contents {
                Some(block_ind)
            } else {
                None
            }
        })
        .rev();

    for ind in 0..filled_blocks {
        match drive_blocks[ind] {
            Some(file_id) => new_arrangement[ind] = file_id,
            None => {
                let block_to_move = full_block_index_from_back_iter
                    .next()
                    .expect("will exaust all blocks");
                new_arrangement[ind] =
                    drive_blocks[block_to_move].expect("indexes in iter are full");
            }
        }
    }

    if debug {
        println!(
            "{}",
            new_arrangement
                .iter()
                .map(|num| num.to_string())
                .collect::<String>()
        );
    }

    let checksum = calculate_checksum(new_arrangement.into_iter().enumerate().collect());
    Some(checksum)
}

fn calculate_checksum(file_blocks: Vec<(usize, u64)>) -> u64 {
    file_blocks
        .into_iter()
        .fold(0u64, |acc, (block_ind, block_contents)| {
            acc + (block_contents * block_ind as u64)
        })
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

struct Node<T> {
    value: T,
    next: Option<usize>,
}

struct LinkedList<T> {
    contents: Vec<Node<T>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { contents: Vec::new() }
    }

    fn get_by_raw_index(&self, index: usize) -> Option<&T> {
        self.contents.get(index).map(|node| &node.value)
    }
}



#[derive(Debug, PartialEq, Clone, Copy)]
enum FileState {
    File,
    FreeSpace,
}

impl FileState {
    pub fn get_toggle(self) -> Self {
        use FileState::*;
        match self {
            File => FreeSpace,
            FreeSpace => File,
        }
    }
}

fn parse_input(input: &str) -> Vec<Option<u64>> {
    // convert input to numbers
    let input: Vec<u64> = input
        .chars()
        .map(|ch| match_numeric(ch).expect("input should all be valid numerics"))
        .collect();
    // calculate and allocate the number of needed blocks
    let total_blocks = input.iter().sum::<u64>() as usize;
    let mut output = vec![None; total_blocks];
    // prepare some state management
    let mut state = FileState::File;
    let mut file_id: u64 = 0;
    let mut block_ind: usize = 0;
    // iterate through the numbers
    input.into_iter().for_each(|num_of_blocks| {
        // if we are looking at a file block, then unpack the file as Some(file_id)
        // if we are looking at free space, then we leave it as the defaulted None
        if state == FileState::File {
            (block_ind..block_ind + num_of_blocks as usize)
                .for_each(|ind| output[ind] = Some(file_id));
            // we have now completed that file, so we can increment file_id
            file_id += 1;
        }
        // move on that number of blocks
        block_ind += num_of_blocks as usize;
        // flip to the next state
        state = state.get_toggle();
    });
    // return the block mapping
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
