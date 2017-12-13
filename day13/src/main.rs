extern crate time;

use std::fs::File;
use std::io::Read;
use time::now;

type Firewall = Vec<u32>;

fn parse(input: &str) -> Firewall {

    let mut result = Firewall::new();
    for line in input.lines() {
        let splitted = line.split(": ")
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        while result.len() < splitted[0] as usize {
            result.push(0);
        }
        result.insert(splitted[0].to_owned() as usize, splitted[1]);
    }
    result
}

fn calculate_severity(firewall: &Firewall, offset: u32, break_on_caught: bool) -> (u32, bool) {
    let mut severity = 0;
    let mut caught = false;
    for (depth, range) in firewall.iter().enumerate() {
        if *range == 0 {
            continue;
        }
        let depth = depth as u32;
        if (depth + offset) % (2 * range - 2) == 0 {
            if break_on_caught {
                return (depth as u32 * range, true)
            }
            severity += depth * range;
            if !caught {
                caught = true;
            }
        }
    }
    (severity, caught)
}

fn get_input() -> String {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input
}

fn determine_delay(firewall: &Firewall) -> u32 {
    let mut delay = 0;
    while calculate_severity(firewall, delay, true).1 {
        delay += 1;
    }
    delay
}

fn main() {
    let input = parse(&get_input());
    {
        let before = now();
        let result = calculate_severity(&input, 0, false).0;
        println!("part1: {:?}\ttook: {}", result, now() - before);
    }
    {
        let before = now();
        let result = determine_delay(&input);
        println!("part2: {:?}\ttook: {}", result, now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn part1_examles() {
        let input = parse(TEST_INPUT);
        assert_eq!(calculate_severity(&input, 0, false).0, 24);
    }

    #[test]
    fn part1_result() {
        let input = parse(&get_input());
        assert_eq!(calculate_severity(&input, 0, false).0, 648);
    }

    #[test]
    fn part2_examles() {
        let input = parse(TEST_INPUT);
        assert_eq!(determine_delay(&input), 10);
    }

    #[test]
    fn part2_results() {
        let input = parse(&get_input());
        assert_eq!(calculate_severity(&input, 3933124, true).0, 0);
    }
}
