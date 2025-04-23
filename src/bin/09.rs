use advent_of_code::shared::match_numeric;

advent_of_code::solution!(9);
const DEBUG: bool = false;

pub fn part_one(input: &str) -> Option<u64> {
    let drive_blocks = parse_input_to_disk(input);
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

    if DEBUG {
        println!(
            "{}",
            new_arrangement
                .iter()
                .map(|num| num.to_string())
                .collect::<String>()
        );
    }

    let final_drive: Vec<Option<u64>> = new_arrangement.into_iter().map(|id| Some(id)).collect();

    let checksum = calculate_checksum(final_drive);
    Some(checksum)
}

fn calculate_checksum(file_blocks: Vec<Option<u64>>) -> u64 {
    file_blocks
        .into_iter()
        .enumerate()
        .fold(0u64, |acc, (block_ind, block_contents)| {
            let block_value = match block_contents {
                Some(id) => id,
                None => 0
            };
            acc + (block_value * block_ind as u64)
        })
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk: Vec<Option<u64>> = parse_input_to_disk(input);
    let disk_blocks: Vec<FileBlock> = parse_input_to_file_blocks(input);

    // make an iterator of the files, running backwards, so we can search for somewhere to put them
    let file_iter_reversed = disk_blocks.into_iter().filter_map(|block| {
        use FileBlock::*;
        match block {
            File(file_block_info) => Some(file_block_info),
            Empty => None,
        }
    }).rev();

    // now starting from the end file, we can move them into the first available free space
    file_iter_reversed.for_each(|file| {
        // check for free space *before the start of the file*
        if let Some(free_space) = find_next_free_space(&disk[0..file.start_index], file.length) {
            // to move the file into that free space,
            // we reassign the necessary blocks from the free space as the file,
            // remembering that the free space could be longer than the file
            (free_space.start_index .. free_space.start_index + file.length).for_each(|ind| {
                disk[ind] = Some(file.file_id)
            });
            // then we erase the file from its previous position
            (file.start_index .. file.start_index + file.length).for_each(|ind| {
                disk[ind] = None
            });
        }
        // if there is no free space to fit the file, we just move on to the next file
    });

    if DEBUG {
        let for_display: String = disk.iter().map(|mem| {
            match mem {
                Some(id) => id.to_string(),
                None => ".".to_string(),
            }
        }).collect();
        print!("{}", for_display)
    }

    // now we have our disk rearranged, we can calculate the checksum and return it as our output
    let output = calculate_checksum(disk);
    Some(output)
}

enum FileBlock {
    File(FileBlockInfo),
    Empty,
}

struct FileBlockInfo {
    file_id: u64,
    start_index: usize,
    length: usize
}

struct EmptyBlockInfo {
    start_index: usize,
}

fn find_next_free_space(
    disk: &[Option<u64>],
    minimum_length: usize,
) -> Option<EmptyBlockInfo> {
    let mut length_counter = 0;
    let mut start_index = 0;
    for (ind, mem) in disk.iter().enumerate() {
        if mem.is_none() && length_counter == 0 {
            // start of a new empty block
            length_counter += 1;
            start_index = ind;
        } else if mem.is_none() && length_counter > 0 {
            // empty block continues
            length_counter += 1;
        } else if mem.is_some() && length_counter > 0 {
            // end of an empty block
            if length_counter >= minimum_length {
                // we found the first block that fits the bill
                return Some(EmptyBlockInfo { start_index });
            } else {
                // the empty block is not long enough, and we must keep searching
                length_counter = 0;
            }
        }
        // if mem.is_some() && length_counter == 0, we can move on until another empty block
    }
    // if we are in an empty block at the end of the slice, we should check if it is a valid empty block
    // think for example of `[...]`, minimum length 2 - since there is no end to the empty block,
    // it won't be returned in the for loop

    // this was an evil edge case
    if length_counter >= minimum_length {
        return Some(EmptyBlockInfo { start_index });
    }
    // otherwise, there are no empty blocks with sufficient free space, and we can return None
    None
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

fn parse_input_to_disk(input: &str) -> Vec<Option<u64>> {
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

fn parse_input_to_file_blocks(input: &str) -> Vec<FileBlock> {
    // convert input to numbers
    let input: Vec<u64> = input
        .chars()
        .map(|ch| match_numeric(ch).expect("input should all be valid numerics"))
        .collect();
    
    let mut file_type = FileState::File;
    let mut index_counter = 0;
    let mut file_id_counter = 0;

    input.into_iter().map(|block_length| {
        use FileState::*;
        let block = match file_type {
            File => {
                // calculate our FileInfo
                let output = FileBlock::File(
                    FileBlockInfo { 
                        file_id: file_id_counter.clone(),
                        start_index: index_counter.clone(),
                        length: block_length as usize
                    }
                );
                // increment the file_id
                file_id_counter += 1;
                output
            },
            FreeSpace => FileBlock::Empty,
        };
        // and finally, move our index along
        index_counter += block_length as usize;
        // and flip the file type in preparation of the next block
        file_type = file_type.get_toggle();

        block
    }).collect()
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
        assert_eq!(result, Some(2858));
    }
}
