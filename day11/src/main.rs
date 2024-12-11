use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut stones: Vec<usize> = Vec::new();

    for line in reader.lines() {
        line.unwrap().split_whitespace().for_each(|word|
            stones.push(word.parse::<usize>().unwrap())
        )
    }

    let part_1_stones = blink(stones, 25);
    let part_1_solution = part_1_stones.len();
    println!("Part 1 solution: {part_1_solution}");
}

fn blink(stones: Vec<usize>, num_iterations: u32) -> Vec<usize> {
    if num_iterations == 0 {
        // base case
        return stones;
    }
    let mut new_stones: Vec<usize> = Vec::new();
    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else if let Some((left, right)) = maybe_split_stone(stone) {
            new_stones.push(left);
            new_stones.push(right);
        } else {
            new_stones.push(stone * 2024);
        }

    }
    blink(new_stones, num_iterations - 1)
}

fn maybe_split_stone(stone: usize) -> Option<(usize, usize)> {
    let mut num_digits = 0;
    let mut n = stone;
    while n > 0 {
        num_digits += 1;
        n /= 10;
    }
    if num_digits % 2 != 0 {
        return None
    }
    let left = stone / 10usize.pow(num_digits / 2);
    let right = stone % 10usize.pow(num_digits / 2);
    Some((left, right))
}
