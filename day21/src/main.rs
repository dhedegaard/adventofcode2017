extern crate time;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use time::now;

type Grid = Vec<Vec<u8>>;
// Prev grid -> [Grid after mapping, number of pixels]
type Rules = HashMap<Grid, (Grid, usize)>;

// Rotate a given grid clockwise.
fn rotate_clockwise(v: &[Vec<u8>]) -> Grid {
    let len = v.len();
    (0..len)
        .map(|l| (0..len).map(|c| v[len - 1 - c][l]).collect())
        .collect()
}

// Generate all possible combinations of a pattern (flip and rotate
// combinations).
fn generate_grid_combinations(v: &[Vec<u8>]) -> Vec<Grid> {
    let mut result = Vec::with_capacity(8);
    // Iterate on normal and flipped grid.
    for mut flip_vec in vec![v.to_vec(), v.iter().rev().cloned().collect()] {
        result.push(flip_vec.clone());
        // Iterate on the 4 possible rotations.
        for _ in 0..3 {
            flip_vec = rotate_clockwise(&flip_vec);
            result.push(flip_vec.clone());
        }
    }
    result
}

// Converts a string-based grid to a grid vector.
fn grid_to_vec(i: &str) -> Grid {
    i.split('/').map(|w| w.as_bytes().to_vec()).collect()
}

// Parse the rules (and all combinations of these) into a hashmap, for fast
// translation.
fn parse_rules(input: &str) -> Rules {
    input
        .lines()
        .map(|line| {
            let mut it = line.trim().split(" => ");
            (it.next().unwrap(), it.next().unwrap())
        })
        .flat_map(|(k, v)| {
            let sharps = v.chars().filter(|&c| c == '#').count();
            let vv = (grid_to_vec(v), sharps);
            generate_grid_combinations(&grid_to_vec(k))
                .into_iter()
                .map(move |kk| (kk, vv.clone()))
        })
        .collect()
}

fn get_input() -> String {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input
}

fn initial_grid() -> Grid {
    grid_to_vec(".#./..#/###")
}

fn calculate_pixels(iterations: u32, rules: &Rules) -> usize {
    let mut grid = initial_grid();
    let mut pixels = 0;
    for _ in 0..iterations {
        pixels = 0;

        // Look for evenly divisible by 2 or 3.
        let rule = 2 + (grid.len() % 2);
        let len_div_rule = grid.len() / rule;

        // Extend the grid vertically and horizontally.
        grid.iter_mut()
            .for_each(|l| l.extend(vec![b' '; len_div_rule]));
        grid.extend(vec![vec![b' '; (rule + 1) * len_div_rule]; len_div_rule]);

        // Iterate on each "block" of the grid, in each direction.
        for c in (0..len_div_rule).rev() {
            for l in (0..len_div_rule).rev() {
                // Match the current "block" against a rule.
                let pattern = (l * rule..l * rule + rule)
                    .map(|line| grid[line][c * rule..c * rule + rule].to_vec())
                    .collect::<Vec<_>>();
                let (ref new, s) = rules[&pattern];

                // Modify the grid to match the new rule.
                for ll in (0..rule + 1).rev() {
                    for cc in (0..rule + 1).rev() {
                        grid[l * (rule + 1) + ll][c * (rule + 1) + cc] = new[ll][cc];
                    }
                }

                // Add the number of pixels added from the current block.
                pixels += s;
            }
        }
    }
    pixels
}

fn main() {
    let rules = parse_rules(&get_input());
    {
        let before = now();
        let result = calculate_pixels(5, &rules);
        println!("part1: {}\ttook: {}", result, now() - before);
    }
    {
        let before = now();
        let result = calculate_pixels(18, &rules);
        println!("part2: {}\ttook: {}", result, now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

    #[test]
    fn test_examples1() {
        let rules = parse_rules(TEST_INPUT);
        let result = calculate_pixels(2, &rules);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_result1() {
        let rules = parse_rules(&get_input());
        let result = calculate_pixels(5, &rules);
        assert_eq!(result, 160);
    }

    #[test]
    fn test_result2() {
        let rules = parse_rules(&get_input());
        let result = calculate_pixels(18, &rules);
        assert_eq!(result, 2271537);
    }
}
