extern crate time;

use std::collections::HashMap;
use time::now;

fn debug_steps(memory_banks: Vec<u32>, find_loop_length: bool) -> u32 {
    let mut memory = memory_banks.clone();
    let mut steps = 0;
    let mut seen = HashMap::new();
    seen.insert(memory_banks.clone(), 0);

    loop {
        // Find the index and memory value of the first highest memory
        let (mut hm_index, mut hm_value): (usize, u32);
        {
            let hm_tuple = memory
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|&(_, e)| e)
                .unwrap();
            hm_index = hm_tuple.0.to_owned();
            hm_value = hm_tuple.1.to_owned();
        }

        // Reset the memory value of the high memory index.
        memory[hm_index] = 0;

        // Push memory values onto the rest of the memory bank until the old value is 0.
        while hm_value > 0 {
            // Push the index pointer.
            hm_index = (hm_index + 1) % memory.len();
            // Increase the memory value.
            memory[hm_index] += 1;
            // Subtract from the high memory value.
            hm_value -= 1;
        }

        // Increment the step counter.
        steps += 1;

        // If we're back at a previously observed memory state, then stop.
        if seen.contains_key(&memory) {
            break;
        }
        // Otherwise add the memory to the seen set.
        seen.insert(memory.clone(), steps);
    }

    if find_loop_length {
        // Find the loop size (part2).
        return steps - seen.get(&memory).unwrap();
    }

    // find the step count (part1).
    steps
}

fn main() {
    let input = INPUT
        .split_whitespace()
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    {
        let before = now();
        let result = debug_steps(input.clone(), false);
        println!("part1: {}\ttook: {}", result, now() - before);
    }
    {
        let before = now();
        let result = debug_steps(input.clone(), true);
        println!("part2: {}\ttook: {}", result, now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(debug_steps(vec![0, 2, 7, 0], false), 5);
    }

    #[test]
    fn part1_result() {
        let input = INPUT
            .split_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(debug_steps(input, false), 6681);
    }

    #[test]
    fn part2_example() {
        assert_eq!(debug_steps(vec![0, 2, 7, 0], true), 4);
    }

    #[test]
    fn part2_result() {
        let input = INPUT
            .split_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(debug_steps(input, true), 2392);
    }
}

const INPUT: &'static str = "4	1	15	12	0	9	9	5	5	8	7	3	14	5	12	3";
