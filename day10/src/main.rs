use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct SearchParams {
    x: usize,
    y: usize,
    target: u32
}

fn main() {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        grid.push(
            line.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
        )
    }

    let mut sum_scores = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, digit) in row.iter().enumerate() {
            if *digit == 0 {
                sum_scores += get_num_destination_nines(x, y, &grid);
            }
        }
    }

    println!("Part 1 solution: {sum_scores}");
}

fn get_num_destination_nines(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let mut found_nines_coords: HashSet<(usize, usize)> = HashSet::new();

    let max_y = grid.len() - 1;
    let max_x = grid[0].len() - 1;
    let mut surrounding_coords = get_surrounding_coords(x, y, max_x, max_y);
    let mut search_params: Vec<SearchParams> = surrounding_coords.iter().map(|(x, y)| SearchParams {
        x: *x,
        y: *y,
        target: 1
    }).collect::<Vec<SearchParams>>();

    // doing depth-first search; it shouldn't matter at all
    while let Some(SearchParams { x, y, target }) = search_params.pop() {
        if grid[y][x] != target {
            continue;
        }
        if target == 9 {
            // base case
            found_nines_coords.insert((x, y));
        } else {
            let surrounding_coords = get_surrounding_coords(x, y, max_x, max_y);
            for (new_x, new_y) in surrounding_coords {
                search_params.push(SearchParams { x: new_x, y: new_y, target: target + 1 })
            }
        }
    }
    found_nines_coords.len() as u32
}

fn get_surrounding_coords(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    // no diagonals allowed!
    let mut surrounding_coords: Vec<(usize, usize)> = Vec::new();
    if x > 0 {
        surrounding_coords.push((x - 1, y)); // left
    }
    if y > 0 {
        surrounding_coords.push((x, y - 1)); // up
    }
    if y < max_y {
        surrounding_coords.push((x, y + 1)); // down
    }
    if x < max_x {
        surrounding_coords.push((x + 1, y)); // right
    }
    surrounding_coords
}