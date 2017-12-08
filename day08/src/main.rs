extern crate time;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use time::now;

type Registers = HashMap<String, i32>;

fn execute(instructions: &str, part2: bool) -> Registers {
    let mut registers = Registers::new();
    let mut highest_value = 0;

    // Iterate on each instruction, splitting the lines into the separate fields.
    for line in instructions.lines() {
        let line = line.split_whitespace().collect::<Vec<_>>();

        // Determine whether the condition is valid.
        let valid = {
            let left_cond = registers.get(line[4]).unwrap_or(&0);
            let right_cond = line[6].parse::<i32>().unwrap();

            match line[5] {
                ">" => left_cond > &right_cond,
                "<" => left_cond < &right_cond,
                ">=" => left_cond >= &right_cond,
                "==" => left_cond == &right_cond,
                "<=" => left_cond <= &right_cond,
                "!=" => left_cond != &right_cond,
                _ => panic!(format!("Missing condition parsing: {}", line[5])),
            }
        };

        if !valid {
            continue;
        }

        // Fetch a mutable borrow of the value of the register.
        let reg = {
            if !registers.contains_key(line[0]) {
                registers.insert(line[0].to_owned(), 0);
            }
            registers.get_mut(line[0]).unwrap()
        };

        // Handle the operation and mutate the registry value.
        let op_count = line[2].parse::<i32>().unwrap();
        *reg = if line[1] == "inc" {
            *reg + op_count
        } else if line[1] == "dec" {
            *reg - op_count
        } else {
            panic!(format!("missing operation parsing: {}", line[1]));
        };

        // If we're doing part2, register a new highest value, if we've encountered it yet.
        if part2 && *reg > highest_value {
            highest_value = *reg;
        }
    }
    // If we're doing part2, register the highest value in the result.
    if part2 {
        registers.insert("highest".to_owned(), highest_value);
    }
    registers
}

fn main() {
    let input = get_input();
    {
        let before = now();
        let result = execute(&input, false);
        let max = result.values().max_by_key(|&e| e).unwrap();
        println!("part1: {}\ttook: {}", max, now() - before);
    }
    {
        let before = now();
        let result = execute(&input, true);
        let highest = result.get("highest").unwrap();
        println!("part2: {}\ttook: {}", highest, now() - before);
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";


    #[test]
    fn part1_example() {
        let registers = execute(TEST_INPUT, false);
        assert_eq!(registers.values().max_by_key(|&e| e).unwrap(), &1); // a is 1.
    }

    #[test]
    fn part1_result() {
        let registers = execute(&get_input(), false);
        assert_eq!(registers.values().max_by_key(|&e| e).unwrap(), &5075);
    }

    #[test]
    fn part2_example() {
        let registers = execute(TEST_INPUT, true);
        assert_eq!(registers.get("highest").unwrap(), &10);
    }

    #[test]
    fn part2_result() {
        let registers = execute(&get_input(), true);
        assert_eq!(registers.get("highest").unwrap(), &7310);
    }
}
