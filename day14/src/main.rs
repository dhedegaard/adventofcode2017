extern crate time;

use std::fmt::Write;
use time::now;

type Grid = Vec<Vec<char>>;

fn count_squares(grid: &Grid) -> u32 {
    let mut result = 0;
    for row in grid {
        result += row.iter().filter(|e| **e == '#').count() as u32;
    }
    result
}

fn count_regions(grid: &mut Grid) -> u32 {
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if iterate_region(grid, y as i32, x as i32) {
                result += 1;
            }
        }
    }
    result
}

fn iterate_region(grid: &mut Grid, y: i32, x: i32) -> bool {
    if y < 0 || x < 0 {
        return false;
    }
    let should_proceed = {
        let (x, y) = (x as usize, y as usize);
        if y >= grid.len() || x >= grid[y].len() {
            return false;
        }
        grid[y as usize][x as usize] == '#'
    };

    if should_proceed {
        grid[y as usize][x as usize] = '.';
        // Recurse in all directions, clearing elements as we move along.
        iterate_region(grid, y, x - 1);
        iterate_region(grid, y, x + 1);
        iterate_region(grid, y + 1, x);
        iterate_region(grid, y - 1, x);
        return true;
    }
    false
}

fn rev_sublist(input: &mut Vec<i32>, index: usize, len: usize) {
    let input_len = input.len();
    let mut sublist = Vec::with_capacity(len);
    {
        for i in index..index + len {
            sublist.push(input[i % input_len] % 256);
        }
    }
    let sublist = sublist.iter().rev().collect::<Vec<_>>();
    for i in index..index + len {
        input[i % input_len] = sublist[i - index] % 256;
    }
}

fn hash(input: Vec<i32>, input_lengths: Vec<usize>, runs: usize) -> Vec<i32> {
    let mut elems = input.iter().map(|e| *e).collect::<Vec<i32>>();
    let mut cur_pos = 0;
    let mut skip_size = 0;

    for _ in 0..runs {
        for length in input_lengths.iter() {
            rev_sublist(&mut elems, cur_pos, *length);

            cur_pos = (cur_pos + length + skip_size) % input.len();
            skip_size += 1;
        }
    }
    elems
}

fn part2(input: &Vec<u8>) -> String {
    // Do the hashing rounds.
    let result = hash(
        (0..256).collect::<Vec<_>>(),
        input.iter().map(|e| *e as usize).collect::<Vec<_>>(),
        64,
    ).iter()
        .map(|e| *e as u8)
        .collect::<Vec<_>>();
    // Build the dense hash
    let mut dense_result = Vec::with_capacity(result.len() / 16);
    for i in 0..result.len() / 16 {
        let slice = result
            .iter()
            .skip(i * 16)
            .take(16)
            .map(|e| *e)
            .collect::<Vec<_>>();
        dense_result.push(dense_hash(&slice));
    }
    // Convert to hex
    vec_to_hex(&dense_result)
}

fn dense_hash(sparse_hash: &Vec<u8>) -> u8 {
    let mut result = 0_u8;
    for e in sparse_hash {
        result ^= e;
    }
    result
}

fn vec_to_hex(input: &Vec<u8>) -> String {
    let mut result = String::with_capacity(input.len() * 2);
    for b in input {
        write!(&mut result, "{:02x}", b).unwrap();
    }
    result
}

fn generate_grid(input: &str) -> Grid {
    let mut result = Grid::with_capacity(128);
    for i in 0..128 {
        // Determine the input
        let mut input = format!("{}-{}", input, i)
            .as_bytes()
            .iter()
            .map(|e| *e as u8)
            .collect::<Vec<_>>();
        input.extend(&[17, 31, 73, 47, 23]);
        // Do the dance.
        let output = part2(&input);
        // Convert it to binary.
        result.push(
            output
                .chars()
                .map(|c| {
                    format!("{:04b}", u8::from_str_radix(&c.to_string(), 16).unwrap())
                })
                .collect::<String>()
                .chars()
                .map(|c| if c == '1' { '#' } else { '.' })
                .collect::<Vec<_>>(),
        );
    }
    result
}

fn main() {
    let mut grid = generate_grid(INPUT);
    {
        let before = now();
        let result = count_squares(&grid);
        println!("part1: {}\ttook: {}", result, now() - before);
    }
    {
        let before = now();
        let result = count_regions(&mut grid);
        println!("part2: {}\ttook: {}", result, now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples1_initial_grid() {
        let grid = generate_grid("flqrgnkx");
        assert_eq!(grid[0].iter().take(8).collect::<String>(), "##.#.#..");
        assert_eq!(grid[1].iter().take(8).collect::<String>(), ".#.#.#.#");
        assert_eq!(grid[2].iter().take(8).collect::<String>(), "....#.#.");
        assert_eq!(grid[3].iter().take(8).collect::<String>(), "#.#.##.#");
        assert_eq!(grid[4].iter().take(8).collect::<String>(), ".##.#...");
        assert_eq!(grid[5].iter().take(8).collect::<String>(), "##..#..#");
        assert_eq!(grid[6].iter().take(8).collect::<String>(), ".#...#..");
        assert_eq!(grid[7].iter().take(8).collect::<String>(), "##.#.##.");
    }

    #[test]
    fn test_examples1_count_squares() {
        let grid = generate_grid("flqrgnkx");
        assert_eq!(count_squares(&grid), 8108);
    }

    #[test]
    fn test_examples2() {
        let mut grid = generate_grid("flqrgnkx");
        assert_eq!(count_regions(&mut grid), 1242);
    }
}

const INPUT: &'static str = "vbqugkhl";
