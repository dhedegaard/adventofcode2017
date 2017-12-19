extern crate time;

use std::fs::File;
use std::io::Read;
use time::now;

type Maze = Vec<Vec<char>>;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> Maze {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn traverse(maze: &Maze) -> (String, usize) {
    let mut result = vec![];
    let mut steps = 0;
    // We always start by going down.
    let mut dir = Direction::Down;
    // Determine the start position.
    let (mut x, mut y) = (maze[0].iter().position(|e| *e == '|').unwrap(), 0);

    loop {
        // Start by moving in the diretion we're facing.
        match dir {
            Direction::Down => {
                if y + 1 >= maze.len() {
                    break;
                }
                y += 1;
            }
            Direction::Up => {
                if y <= 0 {
                    break;
                }
                y -= 1;
            }
            Direction::Left => {
                if x <= 0 {
                    break;
                }
                x -= 1;
            }
            Direction::Right => {
                if x + 1 >= maze[y].len() {
                    break;
                }
                x += 1;
            }
        };
        steps += 1;

        // Fetch the current character from the maze.
        let c = maze[y][x];

        // If we hit a character, add it to the result.
        if c.is_alphabetic() {
            result.push(c);
        }

        // If we're at a crossroads, look for another direction to move to.
        if c == '+' {
            if dir == Direction::Up || dir == Direction::Down {
                if x > 0 && maze[y][x - 1] != ' ' {
                    dir = Direction::Left;
                } else if x + 1 < maze[y].len() && maze[y][x + 1] != ' ' {
                    dir = Direction::Right;
                } else {
                    panic!("Unable to move left or right");
                }
            } else if dir == Direction::Left || dir == Direction::Right {
                if y > 1 && maze[y - 1].len() > x && maze[y - 1][x] != ' ' {
                    dir = Direction::Up;
                } else if y + 1 < maze.len() && maze[y + 1].len() > x && maze[y + 1][x] != ' ' {
                    dir = Direction::Down;
                } else {
                    panic!("Unable to move up or down");
                }
            } else {
                panic!("Hit crossroads, direction is weird: {:?}", dir);
            }
        }

        // If we're outside the track, stop now.
        if c == ' ' {
            break;
        }
    }

    (result.iter().collect(), steps)
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
        let result = traverse(&parse(&get_input()));
        println!("part1: {}\ttook: {}", result.0, now() - before);
    }
    {
        let before = now();
        let result = traverse(&parse(&get_input()));
        println!("part2: {}\t\ttook: {}", result.1, now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+ ";

    #[test]
    fn test_examples1() {
        let maze = parse(TEST_INPUT);
        assert_eq!(traverse(&maze).0, "ABCDEF");
    }

    #[test]
    fn test_result() {
        let maze = parse(&get_input());
        assert_eq!(traverse(&maze).0, "GEPYAWTMLK");
    }

    #[test]
    fn test_examples2() {
        let maze = parse(TEST_INPUT);
        assert_eq!(traverse(&maze).1, 38);
    }
}
