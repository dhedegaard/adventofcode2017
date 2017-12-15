extern crate time;

use std::option::Option;
use time::now;

const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;

// Generates the next value based no the current value, the factor to multiply
// and optionally a modulo check to validate.
fn next_val(val: u64, modulo: &Option<u64>, factor: u64) -> u64 {
    let mut res = val;

    // If the modulo part is none, skip the whole check part and just return
    // the next number.
    if modulo.is_none() {
        return res * factor % 0x7fff_ffff;
    }

    // Otherwise, unwrap() and check until we hit a valid number.
    let modulo = modulo.unwrap();
    loop {
        res = res * factor % 0x7fff_ffff;
        if res % modulo == 0 {
            return res;
        }
    }
}

fn calculate(
    initial_a: u32,
    initial_b: u32,
    modulo_a: &Option<u64>,
    modulo_b: &Option<u64>,
    iterations: u32,
) -> u32 {
    let (mut a, mut b) = (initial_a as u64, initial_b as u64);
    let mut count = 0;
    for _ in 0..iterations {
        a = next_val(a, modulo_a, FACTOR_A);
        b = next_val(b, modulo_b, FACTOR_B);
        if (a ^ b) & 0xffff == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    {
        let before = now();
        let result = calculate(INPUT.0, INPUT.1, &None, &None, 40_000_000);
        println!("part1: {}\ttook: {}", result, now() - before);
    }
    {
        let before = now();
        let result = calculate(INPUT.0, INPUT.1, &Some(4), &Some(8), 5_000_000);
        println!("part2: {}\ttook: {}", result, now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: (u32, u32) = (65, 8921);

    #[test]
    fn test_examples1() {
        let result = calculate(TEST_INPUT.0, TEST_INPUT.1, &None, &None, 40_000_000);
        assert_eq!(result, 588);
    }

    #[test]
    fn test_result1() {
        let result = calculate(INPUT.0, INPUT.1, &None, &None, 40_000_000);
        assert_eq!(result, 619);
    }

    #[test]
    fn test_examples2() {
        let result = calculate(TEST_INPUT.0, TEST_INPUT.1, &Some(4), &Some(8), 5_000_000);
        assert_eq!(result, 309);
    }

    #[test]
    fn test_result2() {
        let result = calculate(INPUT.0, INPUT.1, &Some(4), &Some(8), 5_000_000);
        assert_eq!(result, 290);
    }
}

const INPUT: (u32, u32) = (591, 393);
