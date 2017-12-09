extern crate time;

use std::fs::File;
use std::io::Read;
use time::now;

fn calc_score(input: &str, part2: bool) -> u32 {
    let chars = input.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut result = 0;
    let mut in_garbage = false;
    let mut depth = 0;
    while i < chars.len() {
        let c = chars[i];

        // If we hit an escape character, skip ahead 2 characters (this one and the next).
        if c == '!' {
            i += 2;
            continue;
        }

        if !in_garbage {
            if c == '<' {
                in_garbage = true;
            } else if c == '>' {
                in_garbage = false;
            } else if c == '{' {
                depth += 1;
            } else if c == '}' {
                if !part2 {
                    result += depth;
                }
                depth -= 1;
            }
        } else {
            if c == '>' {
                in_garbage = false;
            } else if part2 {
                result += 1;
            }
        }

        i += 1;
    }
    result
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
    let input = get_input();
    {
        let before = now();
        let result = calc_score(&input, false);
        println!("part1: {}\ttook: {}", result, now() - before);
    }
    {
        let before = now();
        let result = calc_score(&input, true);
        println!("part2: {}\ttook: {}", result, now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(calc_score("{}", false), 1);
        assert_eq!(calc_score("{{{}}}", false), 6);
        assert_eq!(calc_score("{{}, {}}", false), 5);
        assert_eq!(calc_score("{{{},{},{{}}}}", false), 16);
        assert_eq!(calc_score("{<a>,<a>,<a>,<a>}", false), 1);
        assert_eq!(calc_score("{{<ab>},{<ab>},{<ab>},{<ab>}}", false), 9);
        assert_eq!(calc_score("{{<!!>},{<!!>},{<!!>},{<!!>}}", false), 9);
        assert_eq!(calc_score("{{<a!>},{<a!>},{<a!>},{<ab>}}", false), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(calc_score("<>", true), 0);
        assert_eq!(calc_score("<random characters>", true), 17);
        assert_eq!(calc_score("<<<<>", true), 3);
        assert_eq!(calc_score("<{!>}>", true), 2);
        assert_eq!(calc_score("<!!>", true), 0);
        assert_eq!(calc_score("<!!!>>", true), 0);
        assert_eq!(calc_score("<{o\"i!a,<{i<a>", true), 10);
    }
}
