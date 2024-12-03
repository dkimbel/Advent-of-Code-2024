use std::fs;
use regex::Regex;

fn main() {
    let content = fs::read_to_string("resources/input.txt").unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re.captures_iter(&content);

    let mut sum = 0;
    for capture in re.captures_iter(&content) {
        let product = capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap();
        sum += product;
    }

    println!("Part 1 solution: {sum}");
}
