extern crate time;

use std::fs::File;
use std::io::Read;
use time::now;

fn shortest_path(path: &str, part2: bool) -> u32 {
    let (mut x, mut y, mut max_steps) = (0, 0, 0);

    // Start by calculating the position from spawn.
    for p in path.split(",") {
        match p {
            "n" => y -= 2,
            "ne" => {
                y -= 1;
                x += 1;
            }
            "nw" => {
                y -= 1;
                x -= 1;
            }
            "s" => y += 2,
            "se" => {
                y += 1;
                x += 1;
            }
            "sw" => {
                y += 1;
                x -= 1;
            }
            _ => panic!(format!("Unknown path: {}", p)),
        }
        // Probably not that efficient, due to calculating steps after each
        // path change, but fast it enough it seems :)
        if part2 {
            let new_max_steps = calculate_least_amount_of_movex(x, y);
            if new_max_steps > max_steps {
                max_steps = new_max_steps;
            }
        }
    }

    if part2 {
        return max_steps;
    }
    calculate_least_amount_of_movex(x, y)
}

fn calculate_least_amount_of_movex(x: i32, y: i32) -> u32 {
    let (mut x, mut y, mut steps) = (x.to_owned(), y.to_owned(), 0);

    // Based on the position, move back towards spawn.
    while x != 0 || y != 0 {
        // Start by incrementing the move counter.
        steps += 1;

        if x == 0 {
            // If we're done moving to the sides, move up and down.
            if y > 0 {
                // Move: s
                y -= 2;
            } else {
                // Move: n
                y += 2;
            }
        } else if x < 0 && y < 0 {
            // Move: se
            x += 1;
            y += 1;
        } else if x > 0 && y < 0 {
            // Move: sw
            x -= 1;
            y += 1;
        } else if x > 0 && y >= 0 {
            // Move: nw
            x -= 1;
            y -= 1;
        } else if x < 0 && y >= 0 {
            // Move: ne
            x += 1;
            y -= 1;
        } else {
            panic!(format!("X: {}, Y: {}", x, y));
        }
    }

    steps
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
        let result = shortest_path(&input, false);
        println!("part1: {}\ttook: {}", result, now() - before);
    }
    {
        let before = now();
        let result = shortest_path(&input, true);
        println!("part2: {}\ttook: {}", result, now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examles() {
        assert_eq!(shortest_path("ne,ne,ne", false), 3);
        assert_eq!(shortest_path("ne,ne,sw,sw", false), 0);
        assert_eq!(shortest_path("ne,ne,s,s", false), 2);
        assert_eq!(shortest_path("se,sw,se,sw,sw", false), 3);
    }

    #[test]
    fn part1_result() {
        assert_eq!(shortest_path(&get_input(), false), 747);
    }

    #[test]
    fn part2_result() {
        assert_eq!(shortest_path(&get_input(), true), 1544)
    }
}
