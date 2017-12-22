extern crate time;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use time::now;

type Grid = HashMap<Position, Flag>;

#[derive(Debug, Eq, Copy, Clone, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Flag {
    // Clean: Means the position doesn't exist in the grid.
    Weakened,
    Infected,
    Flagged,
}

fn turn_left(direction: &Direction) -> Direction {
    match *direction {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
    }
}

fn turn_right(direction: &Direction) -> Direction {
    match *direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn reverse_direction(direction: &Direction) -> Direction {
    match *direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Right => Direction::Left,
        Direction::Left => Direction::Right,
    }
}

fn parse(input: &str) -> Grid {
    // Determine the middle of the grid.
    let half_width = (input.lines().next().unwrap().chars().count() / 2) as i32;
    let half_height = (input.lines().count() / 2) as i32;
    let mut result = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for x in line.chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .map(|(x, _)| x)
        {
            result.insert(
                Position {
                    x: x as i32 - half_width,
                    y: y as i32 - half_height,
                },
                Flag::Infected,
            );
        }
    }
    result
}

fn move_position(position: &mut Position, direction: &Direction) {
    match direction {
        &Direction::Up => position.y -= 1,
        &Direction::Down => position.y += 1,
        &Direction::Left => position.x -= 1,
        &Direction::Right => position.x += 1,
    }
}

// Handles a tick, modifies the grid, position and direction. Returns true
// when infecting, false when cleansing.
fn tick(grid: &mut Grid, position: &mut Position, direction: &mut Direction, part2: bool) -> bool {
    if !part2 {
        if !grid.contains_key(position) {
            // Clean -> Infected
            grid.insert(position.clone(), Flag::Infected);
            *direction = turn_left(direction);
            move_position(position, direction);
            true
        } else {
            // Infected -> Clean
            grid.remove(position);
            *direction = turn_right(direction);
            move_position(position, direction);
            false
        }
    } else {
        if !grid.contains_key(position) {
            // Clean -> Weakened
            *direction = turn_left(direction);
            grid.insert(position.clone(), Flag::Weakened);
            move_position(position, direction);
            false
        } else {
            // Change direction and determine the new flag for the position.
            let new_flag_value = match grid[position] {
                Flag::Weakened => {
                    // Weakened -> Infected
                    Some(Flag::Infected)
                }
                Flag::Infected => {
                    // Infected -> Flagged
                    *direction = turn_right(direction);
                    Some(Flag::Flagged)
                }
                Flag::Flagged => {
                    // Flagged -> Clean
                    *direction = reverse_direction(direction);
                    None
                }
            };
            // Update the grid.
            match new_flag_value {
                Some(new_flag) => grid.insert(position.clone(), new_flag),
                None => grid.remove(position),
            };
            // Move the position.
            move_position(position, direction);
            // If we ended up in Infected state, return true.
            match new_flag_value {
                Some(e) => e == Flag::Infected,
                _ => false,
            }
        }
    }
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
        let mut grid = parse(&get_input());
        let mut position = Position { x: 0, y: 0 };
        let mut direction = Direction::Up;
        let mut result = 0;
        for _ in 0..10_000 {
            if tick(&mut grid, &mut position, &mut direction, false) {
                result += 1;
            }
        }
        println!("part1: {}\ttook: {}", result, now() - before);
    }
    {
        let before = now();
        let mut grid = parse(&get_input());
        let mut position = Position { x: 0, y: 0 };
        let mut direction = Direction::Up;
        let mut result = 0;
        for _ in 0..10_000_000 {
            if tick(&mut grid, &mut position, &mut direction, true) {
                result += 1;
            }
        }
        println!("part2: {}\ttook: {}", result, now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = "..#
#..
...";

    #[test]
    fn test_parse() {
        let grid = parse(TEST_INPUT);
        assert_eq!(grid.len(), 2);
        assert!(grid.contains_key(&Position { x: 1, y: -1 }));
        assert!(grid.contains_key(&Position { x: -1, y: 0 }));
    }

    #[test]
    fn test_examples1_first_few_moves() {
        let mut grid = parse(TEST_INPUT);
        let mut position = Position { x: 0, y: 0 };
        let mut direction = Direction::Up;
        assert!(!grid.contains_key(&position));

        // Move left and infect
        assert!(tick(&mut grid, &mut position, &mut direction, false));
        assert_eq!(direction, Direction::Left);
        assert_eq!(position, Position { x: -1, y: 0 });
        assert_eq!(grid.len(), 3);
        assert!(grid.contains_key(&position));

        // Move up and clean infection.
        assert!(!tick(&mut grid, &mut position, &mut direction, false));
        assert_eq!(direction, Direction::Up);
        assert_eq!(position, Position { x: -1, y: -1 });
        assert_eq!(grid.len(), 2);
    }

    #[test]
    fn test_examples1_70_movex() {
        let mut grid = parse(TEST_INPUT);
        let mut result = 0;
        let mut position = Position { x: 0, y: 0 };
        let mut direction = Direction::Up;
        for _ in 0..70 {
            if tick(&mut grid, &mut position, &mut direction, false) {
                result += 1;
            }
        }
        assert_eq!(direction, Direction::Up);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_examples1_10_000_moves() {
        let mut grid = parse(TEST_INPUT);
        let mut result = 0;
        let mut position = Position { x: 0, y: 0 };
        let mut direction = Direction::Up;
        for _ in 0..10_000 {
            if tick(&mut grid, &mut position, &mut direction, false) {
                result += 1;
            }
        }
        assert_eq!(result, 5587);
    }

    #[test]
    fn test_result1() {
        let mut grid = parse(&get_input());
        let mut position = Position { x: 0, y: 0 };
        let mut direction = Direction::Up;
        let mut result = 0;
        for _ in 0..10_000 {
            if tick(&mut grid, &mut position, &mut direction, false) {
                result += 1;
            }
        }
        assert_eq!(result, 5406);
    }

    #[test]
    fn test_examples2_first_few_moves() {
        let mut grid = parse(TEST_INPUT);
        let mut position = Position { x: 0, y: 0 };
        let mut direction = Direction::Up;
        assert!(!grid.contains_key(&position));

        // Move left and infect
        assert!(!tick(&mut grid, &mut position, &mut direction, true));
        assert_eq!(direction, Direction::Left);
        assert_eq!(position, Position { x: -1, y: 0 });
        assert_eq!(grid.len(), 3);
        assert_eq!(grid[&Position { x: 0, y: 0 }], Flag::Weakened);

        // Move up and clean infection.
        assert!(!tick(&mut grid, &mut position, &mut direction, true));
        assert_eq!(direction, Direction::Up);
        assert_eq!(position, Position { x: -1, y: -1 });
        assert_eq!(grid[&Position { x: -1, y: 0 }], Flag::Flagged);
        assert_eq!(grid.len(), 3);
    }

    #[test]
    fn test_examples2() {
        let mut grid = parse(TEST_INPUT);
        let mut result = 0;
        let mut position = Position { x: 0, y: 0 };
        let mut direction = Direction::Up;
        for _ in 0..10_000_000 {
            if tick(&mut grid, &mut position, &mut direction, true) {
                result += 1;
            }
        }
        assert_eq!(result, 2_511_944);
    }

    #[test]
    fn test_result2() {
        let mut grid = parse(&get_input());
        let mut position = Position { x: 0, y: 0 };
        let mut direction = Direction::Up;
        let mut result = 0;
        for _ in 0..10_000_000 {
            if tick(&mut grid, &mut position, &mut direction, true) {
                result += 1;
            }
        }
        assert_eq!(result, 2511640);
    }
}
