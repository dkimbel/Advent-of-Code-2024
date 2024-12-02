use std::fs::File;
use std::io::{BufRead, BufReader};

fn report_is_safe(report: &Vec<i32>, remaining_item_removals: u32) -> bool {
    let mut first_safe_change: Option<i32> = None;
    for i in 1..report.len() {
        let change = report[i] - report[i - 1];
        if change.abs() > 3 || change.abs() < 1 {
            if remaining_item_removals == 0 {
                return false;
            } else {
                let mut report_without_i = Vec::new();
                let mut report_without_prev = Vec::new();
                for j in 0..report.len() {
                    if j != i {
                        report_without_i.push(report[j]);
                    }
                    if j != i - 1 {
                        report_without_prev.push(report[j]);
                    }
                }
                return report_is_safe(&report_without_i, remaining_item_removals - 1) || report_is_safe(&report_without_prev, remaining_item_removals - 1);
            }
        }
        if first_safe_change.is_some() {
            if (first_safe_change.unwrap() * change) < 0 {
                if remaining_item_removals == 0 {
                    return false;
                } else {
                    let mut report_without_i = Vec::new();
                    let mut report_without_prev = Vec::new();
                    for j in 0..report.len() {
                        if j != i {
                            report_without_i.push(report[j]);
                        }
                        if j != i - 1 {
                            report_without_prev.push(report[j]);
                        }
                    }
                    return report_is_safe(&report_without_i, remaining_item_removals - 1) || report_is_safe(&report_without_prev, remaining_item_removals - 1);
                }
            }
        } else {
            first_safe_change = Some(change);
        }
    }
    true
}

fn main() {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        let line_content = &line.unwrap();
        let line_words = line_content.split_ascii_whitespace();
        let report = line_words.map(|w| w.parse::<i32>().unwrap()).collect();
        reports.push(report);
    }

    let mut num_fully_safe_reports: i32 = 0;
    let mut num_safe_reports: i32 = 0;
    for report in reports {
        let is_fully_safe = report_is_safe(&report, 0);
        if is_fully_safe {
            num_fully_safe_reports += 1;
        }
        let is_safe = report_is_safe(&report, 1);
        if is_safe {
            num_safe_reports += 1;
        }
    }

    println!("Part 1 solution: {}", num_fully_safe_reports);
    println!("Part 2 solution: {}", num_safe_reports);
}
