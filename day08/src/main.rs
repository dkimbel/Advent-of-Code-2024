use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coords = (usize, usize);

enum X_POSITION {
    LEFT,
    RIGHT
}

enum Y_POSITION {
    TOP,
    BOTTOM
}

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

    let mut part_1_antinodes: HashSet<Coords> = HashSet::new();
    let mut part_2_antinodes: HashSet<Coords> = HashSet::new();
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
                    let (x1_pos, x2_pos) = if x1 > x2 {
                        (X_POSITION::RIGHT, X_POSITION::LEFT)
                    } else {
                        (X_POSITION::LEFT, X_POSITION::RIGHT)
                    };
                    let (y1_pos, y2_pos) = if y1 > y2 {
                        (Y_POSITION::BOTTOM, Y_POSITION::TOP)
                    } else {
                        (Y_POSITION::TOP, Y_POSITION::BOTTOM)
                    };
                    add_coords(x1, y1, max_x, max_y, diff_x.abs(), diff_y.abs(), x1_pos, y1_pos, &mut part_1_antinodes, &mut part_2_antinodes);
                    add_coords(x2, y2, max_x, max_y, diff_x.abs(), diff_y.abs(), x2_pos, y2_pos, &mut part_1_antinodes, &mut part_2_antinodes);
                }
            }
        }
    }

    let num_part_1_antinodes = part_1_antinodes.len();
    println!("Part 1 solution: {num_part_1_antinodes}");

    let num_part_2_antinodes = part_2_antinodes.len();
    println!("Part 2 solution: {num_part_2_antinodes}");
}

fn add_coords(x: i32, y: i32, max_x: i32, max_y: i32, diff_x_abs: i32, diff_y_abs: i32, x_pos: X_POSITION, y_pos: Y_POSITION, part_1_antinodes: &mut HashSet<Coords>, part_2_antinodes: &mut HashSet<Coords>) -> () {
    let mut iters = 0;
    let mut new_x = x;
    let mut new_y = y;
    while (new_x >= 0 && new_x <= max_x && new_y >= 0 && new_y <= max_y) {
        part_2_antinodes.insert((new_x as usize, new_y as usize));
        if (iters == 1) {
            part_1_antinodes.insert((new_x as usize, new_y as usize));
        }
        iters += 1;
        new_x = match x_pos {
            X_POSITION::RIGHT => new_x + diff_x_abs,
            X_POSITION::LEFT => new_x - diff_x_abs,
        };
        new_y = match y_pos {
            Y_POSITION::TOP => new_y - diff_y_abs,
            Y_POSITION::BOTTOM => new_y + diff_y_abs,
        }
    }
}