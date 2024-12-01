use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut list_1: Vec<i32> = Vec::new();
    let mut list_2: Vec<i32> = Vec::new();

    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_content = &line.unwrap();
        let mut line_words = line_content.split_ascii_whitespace();
        let first_location_id = line_words.next().unwrap().parse::<i32>().unwrap();
        let second_location_id = line_words.next().unwrap().parse::<i32>().unwrap();
        list_1.push(first_location_id);
        list_2.push(second_location_id);
    }

    list_1.sort();
    list_2.sort();

    let mut sum_distances = 0;
    for (first_location_id, second_location_id) in list_1.iter().zip(list_2.iter()) {
        let distance = (first_location_id - second_location_id).abs();
        sum_distances += distance;
    }

    println!("Part 1 solution: {sum_distances}");

    let mut list_2_item_counts = HashMap::new();
    for location_id in list_2 {
        *list_2_item_counts.entry(location_id).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for location_id in list_1 {
        let num_occurrences_in_list_2 = list_2_item_counts.get(&location_id).unwrap_or(&0);
        similarity_score += location_id * num_occurrences_in_list_2;
    }

    println!("Part 2 solution: {similarity_score}");
}
