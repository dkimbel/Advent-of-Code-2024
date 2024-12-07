use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
enum Operation {
    ADD,
    CONCAT,
    MULTIPLY,
}

#[derive(Clone, Debug)]
struct Equation {
    result: i64,
    terms: Vec<i64>
}

impl Equation {
    fn from_line(line: &str) -> Self {
        // TODO find a way to avoid this `collect` call
        let split = line.split(":").collect::<Vec<&str>>();
        let result = split[0].parse::<i64>().unwrap();
        let terms = split[1].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        Self { result, terms }
    }

    fn can_be_solved_with_operations(&self, operations: &[Operation]) -> bool {
        fn inner(remaining_terms: &[i64], result: i64, curr: i64, curr_op: Operation, available_ops: &[Operation]) -> bool {
            if remaining_terms.is_empty() {
                return curr == result;
            } else if curr > result {
                // short-circuit -- at least so far, every possible operation can only increase value
                return false;
            }

            let new_term = remaining_terms[0];
            let new_curr = match curr_op {
                Operation::ADD => curr + new_term,
                Operation::CONCAT => format!("{curr}{new_term}").parse::<i64>().unwrap(),
                Operation::MULTIPLY => curr * new_term,
            };

            available_ops.iter().any(|&op| inner(&remaining_terms[1..], result, new_curr, op, available_ops))
        }

        operations.iter().any(|&op| inner(&self.terms[1..], self.result, self.terms[0], op, operations))
    }
}

fn main() {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut equations: Vec<Equation> = Vec::new();

    for line in reader.lines() {
        equations.push(Equation::from_line(&line.unwrap()));
    }

    let score = equations.iter()
        .filter(|eq| eq.can_be_solved_with_operations(&[Operation::ADD, Operation::MULTIPLY]))
        .map(|eq| eq.result)
        .sum::<i64>();

    println!("Part 1 solution: {score}");

    let alt_score = equations.iter()
        .filter(|eq| eq.can_be_solved_with_operations(&[Operation::ADD, Operation::MULTIPLY, Operation::CONCAT]))
        .map(|eq| eq.result)
        .sum::<i64>();

    println!("Part 2 solution: {alt_score}");
}
