use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum BlockID {
    FREE,
    FILE_ID(usize)
}

#[derive(Copy, Clone, Debug)]
struct FileBlockRange {
    file_id: usize,
    size:  usize,
    start_index: usize,
}

fn main() {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut blocks: Vec<BlockID> = Vec::new();
    let mut whole_file_blocks: Vec<FileBlockRange> = Vec::new();

    let mut is_file = true;
    let mut file_id: usize = 0;
    let mut curr_index: usize = 0;
    for line in reader.lines() {
        &line.unwrap().chars().for_each(|c| {
            let num_indices = c.to_digit(10).unwrap();
            let id = if is_file {
                BlockID::FILE_ID(file_id)
            } else {
                BlockID::FREE
            };
            for _ in 0..num_indices {
                blocks.push(id);
            }

            if is_file {
                whole_file_blocks.push(FileBlockRange { file_id, size: num_indices as usize, start_index: curr_index });
                file_id += 1;
            }
            is_file = !is_file;
            curr_index += num_indices as usize;
        });
    }

    let mut original_free_indices_reversed: Vec<usize> = Vec::new();
    for i in 0..blocks.len() {
        if blocks[i] == BlockID::FREE {
            original_free_indices_reversed.push(i);
        }
    }
    original_free_indices_reversed.reverse();

    let mut part_1_compacted_blocks = blocks.clone();
    for i in (0..blocks.len()).rev() {
        let block = blocks[i];
        if block == BlockID::FREE {
            continue;
        }

        let earliest_free_index = original_free_indices_reversed.pop().unwrap();
        if i < earliest_free_index {
            // basically our 'base case' -- there are no free indexes before the point we're
            // looking at, meaning there is no 'compaction' left to be done
            break;
        }
        part_1_compacted_blocks[earliest_free_index] = block;
        part_1_compacted_blocks[i] = BlockID::FREE;
    }

    let mut part_1_checksum = 0;
    for (i, block) in part_1_compacted_blocks.iter().enumerate() {
        match block {
            BlockID::FREE => { break; },
            BlockID::FILE_ID(id) => { part_1_checksum += i * id }
        }
    }

    println!("Part 1 solution: {part_1_checksum}");

    // NOTE:  this part 2 solution is not as time-efficient as it could be in O(n) terms. We're
    // iterating through the list of blocks every time we need to find free space, instead of e.g.
    // looking up blocks by size in a hashmap.
    let mut part_2_compacted_blocks = blocks.clone();
    whole_file_blocks.reverse();
    for whole_file_block in whole_file_blocks {
        let maybe_i = get_start_index_of_first_free_block_of_at_least_size(&part_2_compacted_blocks, whole_file_block.size);
        if let Some(free_start_i) = maybe_i {
            if free_start_i > whole_file_block.start_index {
                continue;
            }
            for free_i in free_start_i..(free_start_i + whole_file_block.size) {
                part_2_compacted_blocks[free_i] = BlockID::FILE_ID(whole_file_block.file_id);
            }
            for previous_file_i in whole_file_block.start_index..(whole_file_block.start_index+whole_file_block.size) {
                part_2_compacted_blocks[previous_file_i] = BlockID::FREE;
            }
        }
    }

    let mut part_2_checksum = 0;
    for (i, block) in part_2_compacted_blocks.iter().enumerate() {
        match block {
            BlockID::FREE => (),
            BlockID::FILE_ID(id) => { part_2_checksum += i * id }
        }
    }

    println!("Part 2 solution: {part_2_checksum}");
}

fn get_start_index_of_first_free_block_of_at_least_size(block_ids: &[BlockID], size: usize) -> Option<usize> {
    let mut curr_free_block_start_index: Option<usize> = None;
    let mut curr_free_block_size = 0;
    for (i, block_id) in block_ids.iter().enumerate() {
        if block_id == &BlockID::FREE {
            if curr_free_block_size == 0 {
                curr_free_block_start_index = Some(i);
            }
            curr_free_block_size += 1;
        } else {
            if curr_free_block_size >= size {
                return curr_free_block_start_index;
            } else {
                curr_free_block_size = 0;
                curr_free_block_start_index = None;
            }
        }
    }
    None
}