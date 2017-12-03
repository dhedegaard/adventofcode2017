extern crate time;

use std::collections::HashMap;
use time::now;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

type Grid = HashMap<Point, u32>;

fn calculate_manhattan_distance(input: u32) -> u32 {
    let mut dir: Direction = Direction::Right;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for _ in 1..input {
        // Determine if we should change direction.
        dir = if dir == Direction::Right && x == y + 1 {
            Direction::Up
        } else if dir == Direction::Up && x == y * -1 {
            Direction::Left
        } else if dir == Direction::Left && x == y {
            Direction::Down
        } else if dir == Direction::Down && x * -1 == y {
            Direction::Right
        } else {
            dir
        };

        // Iterate in whatever direction is current.
        match dir {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
    }
    (x.abs() + y.abs()) as u32
}

fn calculate_manhattan_distance_part2(input: u32) -> u32 {
    let mut grid: Grid = HashMap::new();
    grid.insert(Point { x: 0, y: 0 }, 1);
    let mut dir: Direction = Direction::Right;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    loop {
        // Determine if we should change direction.
        dir = if dir == Direction::Right && x == y + 1 {
            Direction::Up
        } else if dir == Direction::Up && x == y * -1 {
            Direction::Left
        } else if dir == Direction::Left && x == y {
            Direction::Down
        } else if dir == Direction::Down && x * -1 == y {
            Direction::Right
        } else {
            dir
        };

        // Iterate in whatever direction is current.
        match dir {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }

        // Calculate the value for the point.
        let mut value = 0;
        for _x in -1..2 {
            for _y in -1..2 {
                if _x == 0 && _y == 0 {
                    continue;
                }
                value += grid.get(&Point {
                    x: x + _x,
                    y: y + _y,
                }).unwrap_or(&0);
            }
        }

        // Check if we've hit the limit yet.
        if value > input {
            return value as u32;
        }

        // Add the value to the grid.
        grid.insert(Point { x: x, y: y }, value as u32);
    }
}

const INPUT: u32 = 277678;

fn main() {
    {
        let before = now();
        let result = calculate_manhattan_distance(INPUT);
        println!("part1: {} - took: {}", result, now() - before);
    }
    {
        let before = now();
        let result = calculate_manhattan_distance_part2(INPUT);
        println!("part1: {} - took: {}", result, now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(calculate_manhattan_distance(1), 0);
        assert_eq!(calculate_manhattan_distance(12), 3);
        assert_eq!(calculate_manhattan_distance(23), 2);
        assert_eq!(calculate_manhattan_distance(1024), 31);
    }

    #[test]
    fn part1_result() {
        assert_eq!(calculate_manhattan_distance(277678), 475);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(calculate_manhattan_distance_part2(1), 2);
        assert_eq!(calculate_manhattan_distance_part2(2), 4);
        assert_eq!(calculate_manhattan_distance_part2(3), 4);
        assert_eq!(calculate_manhattan_distance_part2(4), 5);
        assert_eq!(calculate_manhattan_distance_part2(5), 10);
    }

    #[test]
    fn part2_result() {
        assert_eq!(calculate_manhattan_distance_part2(277678), 279138);
    }
}
