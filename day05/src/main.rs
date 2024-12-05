use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let content = fs::read_to_string("resources/input.txt").unwrap();
    // TODO here and below, find a cleaner way to split a string in two (can I unpack
    //   into a tuple, and avoid allocating a vec?)
    let split_content = content.split("\n\n").collect::<Vec<&str>>();
    let unparsed_orderings = split_content[0];
    let unparsed_updates = split_content[1];

    let mut blockers_by_page_num: HashMap<i32, HashSet<i32>> = HashMap::new();
    for ordering in unparsed_orderings.split("\n") {
        let split_orderings = ordering.split("|").collect::<Vec<&str>>();
        let blocker = split_orderings[0].parse::<i32>().unwrap();
        let blocked = split_orderings[1].parse::<i32>().unwrap();
        blockers_by_page_num.entry(blocked).or_insert_with(HashSet::new).insert(blocker);
    }

    let mut updates: Vec<Vec<i32>> = Vec::new();
    for unparsed_update in unparsed_updates.split("\n") {
        if unparsed_update.is_empty() {
            continue
        }
        let update = unparsed_update.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        updates.push(update);
    }

    solve_part_1(&blockers_by_page_num, &updates);
}

fn solve_part_1(blockers_by_page_num: &HashMap<i32, HashSet<i32>>, updates: &Vec<Vec<i32>>) {
    let mut score = 0;
    for update_seq in updates {
        if are_updates_valid(blockers_by_page_num, update_seq) {
            score += get_score(update_seq);
        }
    }
    println!("Part 1 solution: {score}");
}

fn are_updates_valid(blockers_by_page_num: &HashMap<i32, HashSet<i32>>, updates: &Vec<i32>) -> bool {
    for i in 0..updates.len() {
        // do any of this page's blockers appear after it in this list of updates?
        let remaining_pages: HashSet<i32> = HashSet::from_iter(updates[i..].iter().cloned());
        // TODO clean this up, ideally shouldn't need "empty default"
        let empty_default_hashset = HashSet::new();
        let blockers_for_page = &blockers_by_page_num.get(&updates[i]).unwrap_or(&empty_default_hashset);
        let has_blockers = !remaining_pages.is_disjoint(blockers_for_page);
        if has_blockers {
            return false;
        }
    }
    true
}

fn get_score(updates: &Vec<i32>) -> i32 {
    // score is just middle item
    updates[updates.len() / 2]
}