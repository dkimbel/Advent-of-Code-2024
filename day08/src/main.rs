use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coords = (usize, usize);

fn main() {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut antenna_type_to_coords: HashMap<char, Vec<Coords>> = HashMap::new();
    let mut max_y: usize = 0;
    let mut max_x: usize = 0;

    for (y, line) in reader.lines().enumerate() {
        &line.unwrap().chars().enumerate().for_each(|(x, c)| {
            max_x = cmp::max(max_x, x);
            if c != '.' {
                antenna_type_to_coords.entry(c).or_insert(Vec::new()).push((x, y));
            }
        });
        max_y = std::cmp::max(max_y, y);
    }

    let max_x = max_x as i32;
    let max_y = max_y as i32;
    let mut antinodes: HashSet<Coords> = HashSet::new();
    for antenna_type in antenna_type_to_coords.keys() {
        let coords = &antenna_type_to_coords[antenna_type];
        // check every pair of coords for antinodes
        for (x1, y1) in coords {
            for (x2, y2) in coords {
                let x1 = *x1 as i32;
                let x2 = *x2 as i32;
                let y1 = *y1 as i32;
                let y2 = *y2 as i32;
                if (x1, y1) != (x2, y2) {
                    let diff_x = x2 - x1;
                    let diff_y = y2 - y1;
                    let anti_small_x = cmp::min(x1, x2) - diff_x.abs();
                    let anti_large_x = cmp::max(x1, x2) + diff_x.abs();
                    let anti_small_y = cmp::min(y1, y2) - diff_y.abs();
                    let anti_large_y = cmp::max(y1, y2) + diff_y.abs();
                    // figure out pairings of small/large x/y
                    let (anti_1_x, anti_2_x) = if x1 > x2 {
                        (anti_large_x, anti_small_x)
                    } else {
                        (anti_small_x, anti_large_x)
                    };
                    let (anti_1_y, anti_2_y) = if y1 > y2 {
                        (anti_large_y, anti_small_y)
                    } else {
                        (anti_small_y, anti_large_y)
                    };
                    if anti_1_x >= 0 && anti_1_x <= max_x && anti_1_y >= 0 && anti_1_y <= max_y {
                        antinodes.insert((anti_1_x as usize, anti_1_y as usize));
                    }
                    if anti_2_x >= 0 && anti_2_x <= max_x && anti_2_y >= 0 && anti_2_y <= max_y {
                        antinodes.insert((anti_2_x as usize, anti_2_y as usize));
                    }
                }
            }
        }
    }

    let num_antinodes = antinodes.len();
    println!("Part 1 solution: {num_antinodes}");
}
