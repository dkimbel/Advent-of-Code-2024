use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum BlockID {
    FREE,
    FILE_ID(usize)
}

fn main() {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut blocks: Vec<BlockID> = Vec::new();

    let mut is_file = true;
    let mut file_id: usize = 0;
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
                file_id += 1;
            }
            is_file = !is_file;
        });
    }

    let mut original_free_indices_reversed: Vec<usize> = Vec::new();
    for i in 0..blocks.len() {
        if blocks[i] == BlockID::FREE {
            original_free_indices_reversed.push(i);
        }
    }
    original_free_indices_reversed.reverse();

    let mut compacted_blocks = blocks.clone();
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
        compacted_blocks[earliest_free_index] = block;
        compacted_blocks[i] = BlockID::FREE;
    }

    let mut checksum = 0;
    for (i, block) in compacted_blocks.iter().enumerate() {
        match block {
            BlockID::FREE => { break; },
            BlockID::FILE_ID(id) => { checksum += i * id }
        }
    }

    println!("Part 1 solution: {checksum}");
}
