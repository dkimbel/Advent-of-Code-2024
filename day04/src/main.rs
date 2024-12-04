use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
enum Direction {
    UP,
    UP_RIGHT,
    RIGHT,
    DOWN_RIGHT,
    DOWN,
    DOWN_LEFT,
    LEFT,
    UP_LEFT,
}

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_content = &line.unwrap();
        let chars = line_content.chars().collect::<Vec<char>>();
        grid.push(chars);
    }

    let num_rows = grid.len();
    let num_cols = grid[0].len();  // assume all rows have same length

    let mut count = 0;
    for y in 0..num_rows {
        for x in 0..num_cols {
            if grid[y][x] == 'X' {
                count += directional_search('X', x, y, Direction::UP, &grid);
                count += directional_search('X', x, y, Direction::UP_RIGHT, &grid);
                count += directional_search('X', x, y, Direction::RIGHT, &grid);
                count += directional_search('X', x, y, Direction::DOWN_RIGHT, &grid);
                count += directional_search('X', x, y, Direction::DOWN, &grid);
                count += directional_search('X', x, y, Direction::DOWN_LEFT, &grid);
                count += directional_search('X', x, y, Direction::LEFT, &grid);
                count += directional_search('X', x, y, Direction::UP_LEFT, &grid);
            }
        }
    }

    println!("Part 1 solution: {count}");
}

fn directional_search(c: char, x: usize, y: usize, direction: Direction, grid: &Vec<Vec<char>>) -> i32 {
    let maybe_next_char = get_next_char(c);
    if maybe_next_char.is_none() {
        return 1;  // basically our 'base case' -- we found the last letter!
    }
    // TODO try to handle this 'is none or else unwrap' flow more elegantly
    let next_char = maybe_next_char.unwrap();
    let maybe_coords_to_check = get_coords_in_direction(x, y, direction, grid);
    if maybe_coords_to_check.is_none() {
        return 0;
    } else {
        let (next_x, next_y) = maybe_coords_to_check.unwrap();
        let char_at_coords = grid[next_y][next_x];
        return if char_at_coords == next_char {
            directional_search(next_char, next_x, next_y, direction, grid)
        } else {
            0
        }
    }
}

fn get_next_char(c: char) -> Option<char> {
    match c {
        'X' => Some('M'),
        'M' => Some('A'),
        'A' => Some('S'),
        'S' => None,
        // TODO use typechecking to make this case impossible?
        _ => None,
    }
}

fn get_coords_in_direction(x: usize, y: usize, direction: Direction, grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let max_row_index = grid.len() - 1;
    let max_col_index = grid[0].len() - 1;
    return match direction {
        Direction::UP => if y > 0 { Some((x, y - 1))} else { None },
        Direction::UP_RIGHT => if y > 0 && x < max_row_index { Some((x + 1, y - 1))} else { None },
        Direction::RIGHT => if x < max_row_index { Some((x + 1, y))} else { None },
        Direction::DOWN_RIGHT => if y < max_col_index && x < max_row_index { Some((x + 1, y + 1))} else { None },
        Direction::DOWN => if y < max_col_index { Some((x, y + 1))} else { None },
        Direction::DOWN_LEFT => if y < max_col_index && x > 0 { Some((x - 1, y + 1))} else { None },
        Direction::LEFT => if x > 0 { Some((x - 1, y))} else { None },
        Direction::UP_LEFT => if y > 0 && x > 0 { Some((x - 1, y - 1))} else { None },
    }
}