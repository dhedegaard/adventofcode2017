extern crate time;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use time::now;

type Programs = HashMap<u32, HashSet<u32>>;

fn parse_programs(input: &str) -> Programs {
    let mut result = Programs::new();
    for line in input.lines() {
        let node = line.split(" <-> ").next().unwrap().parse::<u32>().unwrap();
        let children = line.split(" <-> ")
            .skip(1)
            .next()
            .unwrap()
            .split(", ")
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        result.insert(node, children.to_owned());
        for child in children {
            if result.contains_key(&child) {
                result.get_mut(&child).unwrap().insert(node);
            } else {
                let mut value_set = HashSet::new();
                value_set.insert(node);
                result.insert(child, value_set);
            }
        }
    }
    result
}

fn determine_programgroup(programs: &Programs, program_id: u32) -> HashSet<u32> {
    let mut result = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(program_id);
    while !queue.is_empty() {
        let elem = queue.pop_front().unwrap();
        result.insert(elem);
        for child in programs.get(&elem).unwrap() {
            if !result.contains(child) {
                queue.push_back(*child);
                result.insert(*child);
            }
        }
    }
    result
}

fn count_program_groups(programs: &Programs) -> u32 {
    // Put all the nodes in a queue.
    let mut queue: VecDeque<u32> = VecDeque::from_iter(programs.keys().map(|e| *e));
    // Put all the nodes in a set.
    let mut all_programs = HashSet::from_iter(programs.keys().map(|e| *e));
    let mut count = 0;
    while !queue.is_empty() {
        // Fetch a node from the queue, skipping it if it's already been
        // processed from another node linked to it somehow.
        let elem = queue.pop_back().unwrap();
        if !all_programs.contains(&elem) {
            continue;
        }
        // Remove all the nodes linked to the given node, from the set of all
        // the programs.
        all_programs = all_programs
            .difference(&determine_programgroup(programs, elem))
            .map(|e| *e)
            .collect();
        // Increment the program group counter.
        count += 1;
    }
    count
}

fn get_input() -> String {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input
}

fn main() {
    let input = parse_programs(&get_input());
    {
        let before = now();
        let result = determine_programgroup(&input, 0).len();
        println!("part1: {}\ttook: {}", result, now() - before);
    }
    {
        let before = now();
        let result = count_program_groups(&input);
        println!("part2: {}\ttook: {}", result, now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    #[test]
    fn part1_examles() {
        let input = parse_programs(TEST_INPUT);
        assert_eq!(determine_programgroup(&input, 0).len(), 6);
    }

    #[test]
    fn part1_result() {
        let input = parse_programs(&get_input());
        assert_eq!(determine_programgroup(&input, 0).len(), 134);
    }

    #[test]
    fn part2_examples() {
        let input = parse_programs(TEST_INPUT);
        assert_eq!(count_program_groups(&input), 2);
    }

    #[test]
    fn part2_result() {
        let input = parse_programs(&get_input());
        assert_eq!(count_program_groups(&input), 193);
    }
}
