use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coords = (usize, usize);

struct SearchParams {
    x: usize,
    y: usize,
    target: u32,
    path: Vec<Coords>
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

    let mut sum_part_1_scores = 0;
    let mut sum_part_2_scores = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, digit) in row.iter().enumerate() {
            if *digit == 0 {
                let paths_to_nines = get_unique_paths_to_nines(x, y, &grid);
                let part_1_start_ends = paths_to_nines.iter().map(|path| (path[0], path[path.len() - 1])).collect::<Vec<(Coords, Coords)>>();
                let part_1_uniques: HashSet<&(Coords, Coords)> = HashSet::from_iter(part_1_start_ends.iter());
                sum_part_1_scores += part_1_uniques.len();
                sum_part_2_scores += paths_to_nines.len()
            }
        }
    }

    println!("Part 1 solution: {sum_part_1_scores}");
    println!("Part 2 solution: {sum_part_2_scores}");
}

fn get_unique_paths_to_nines(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> Vec<Vec<Coords>> {
    let mut found_nines_paths: Vec<Vec<Coords>> = Vec::new();

    let max_y = grid.len() - 1;
    let max_x = grid[0].len() - 1;
    let mut surrounding_coords = get_surrounding_coords(x, y, max_x, max_y);
    let mut search_params: Vec<SearchParams> = surrounding_coords.iter().map(|(x, y)| SearchParams {
        x: *x,
        y: *y,
        target: 1,
        path: vec![(0, 0)]
    }).collect::<Vec<SearchParams>>();

    // doing depth-first search; it shouldn't matter at all
    while let Some(SearchParams { x, y, target, path }) = search_params.pop() {
        if grid[y][x] != target {
            continue;
        }
        if target == 9 {
            // base case
            let mut final_path = path.clone();
            final_path.push((x, y));
            found_nines_paths.push(final_path);
        } else {
            let surrounding_coords = get_surrounding_coords(x, y, max_x, max_y);
            for (new_x, new_y) in surrounding_coords {
                let mut new_path = path.clone();
                new_path.push((x, y));
                search_params.push(SearchParams { x: new_x, y: new_y, target: target + 1, path: new_path })
            }
        }
    }
    found_nines_paths
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