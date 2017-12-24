extern crate time;

use std::collections::HashSet;
use std::io::Read;
use std::fs::File;
use time::now;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Component {
    left: u64,
    right: u64,
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
    {
        let before = now();
        let mut all = parse_input(&get_input());
        let result = iter_components(0, &vec![], &mut all, false);
        println!("part1: {}\ttook: {}", result.0, now() - before);
    }
    {
        let before = now();
        let mut all = parse_input(&get_input());
        let result = iter_components(0, &vec![], &mut all, true);
        println!("part2: {}\ttook: {}", result.0, now() - before);
    }
}

fn parse_input(input: &str) -> HashSet<Component> {
    let mut all = HashSet::new();
    for line in input.lines() {
        let mut it = line.split('/').map(|s| s.parse::<u64>().unwrap());
        all.insert(Component {
            left: it.next().unwrap(),
            right: it.next().unwrap(),
        });
    }
    all
}

fn iter_components(
    start: u64,
    path: &Vec<Component>,
    components: &mut HashSet<Component>,
    only_accept_longest: bool,
) -> (u64, usize) {
    let mut result = path.iter().map(|c| c.left + c.right).sum::<u64>();
    let mut length = path.len();
    for c in components.iter() {
        if c.left == start || c.right == start {
            let mut new_components = components.clone();
            new_components.remove(c);
            let mut new_path = path.clone();
            new_path.push(*c);
            // Recurse, looking for the best result.
            let (new_result, new_length) = iter_components(
                if c.left == start { c.right } else { c.left },
                &new_path,
                &mut new_components,
                only_accept_longest,
            );
            if only_accept_longest {
                // Part 2
                if new_length > length {
                    // Always use longer lengths, even if the result is lower.
                    result = new_result;
                    length = new_length;
                } else if new_length == length && new_result >= result {
                    // If the length is equal but the result is higher, use it.
                    result = new_result;
                    length = new_length;
                }
            } else if new_result > result {
                // Part 1
                // Result is king, ignore the length.
                result = new_result;
                length = new_length;
            }
        }
    }
    (result, length)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

    #[test]
    fn test_examples1() {
        let mut all = parse_input(TEST_INPUT);
        assert_eq!(iter_components(0, &vec![], &mut all, false).0, 31);
    }

    #[test]
    fn test_result1() {
        let mut all = parse_input(&get_input());
        assert_eq!(iter_components(0, &vec![], &mut all, false).0, 1868);
    }

    #[test]
    fn test_examples2() {
        let mut all = parse_input(TEST_INPUT);
        let result = iter_components(0, &vec![], &mut all, true);
        assert_eq!(result.1, 4);
        assert_eq!(result.0, 19);
    }

    #[test]
    fn test_result2() {
        let mut all = parse_input(&get_input());
        let result = iter_components(0, &vec![], &mut all, true);
        assert_eq!(result.1, 40); // Max length is 40
        assert_eq!(result.0, 1841); // Max value of length 40
    }
}
