use std::fs;
use regex::Regex;

fn main() {
    let content = fs::read_to_string("resources/input.txt").unwrap();
    solve_part_1(&content);
    solve_part_2(&content);
}

fn solve_part_1(content: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re.captures_iter(&content);

    let mut sum = 0;
    for capture in re.captures_iter(&content) {
        let product = capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap();
        sum += product;
    }

    println!("Part 1 solution: {sum}");
}

fn solve_part_2(content: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let matches = re.captures_iter(&content);

    let mut sum = 0;
    let mut enabled = true;
    for capture in re.captures_iter(&content) {
        let command = capture[0].to_string();
        if command == "do()" {
            enabled = true;
        } else if command == "don't()" {
            enabled = false;
        } else if enabled {
            let product = capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap();
            sum += product;
        }
    }

    println!("Part 2 solution: {sum}");
}
