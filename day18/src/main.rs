extern crate time;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use time::now;

type Memory = HashMap<char, i64>;

#[derive(Debug)]
enum Value {
    Char(char),
    Number(i64),
}

fn parse_value(input: &str) -> Value {
    match input.parse::<i64>() {
        Ok(e) => Value::Number(e),
        _ => Value::Char(input.chars().next().unwrap()),
    }
}

fn value_to_int(value: &Value, memory: &Memory) -> i64 {
    match value {
        &Value::Char(c) => return *memory.get(&c).unwrap_or(&0),
        &Value::Number(i) => return i,
    }
}

fn unwrap_char(value: &Value) -> char {
    match value {
        &Value::Char(c) => c,
        _ => panic!("Value is not a Char: {:?}", value),
    }
}

fn execute(instructions: &str, memory: &mut Memory) -> Option<i64> {
    let instructions = instructions.lines().collect::<Vec<_>>();
    let mut pc: i64 = 0;
    let mut last_sound = 0;
    while pc >= 0 && pc < instructions.len() as i64 {
        let values = instructions[pc as usize]
            .split(" ")
            .skip(1)
            .map(|e| parse_value(e))
            .collect::<Vec<_>>();
        let inst = instructions[pc as usize].split(" ").next().unwrap();
        match inst {
            "set" => {
                let val1 = value_to_int(&values[1], &memory);
                memory.insert(unwrap_char(&values[0]), val1);
            }
            "add" => {
                let val1 = value_to_int(&values[1], &memory);
                let val = memory.get_mut(&unwrap_char(&values[0])).unwrap();
                *val = *val + val1;
            }
            "mul" => {
                let val0 = value_to_int(&values[0], &memory);
                let val1 = value_to_int(&values[1], &memory);
                memory.insert(unwrap_char(&values[0]), val0 * val1);
            }
            "mod" => {
                let val1 = value_to_int(&values[1], &memory);
                let val = memory.get_mut(&unwrap_char(&values[0])).unwrap();
                *val = *val % val1;
            }
            "snd" => {
                last_sound = value_to_int(&values[0], &memory);
            }
            "jgz" => if value_to_int(&values[0], &memory) > 0 {
                pc += value_to_int(&values[1], &memory) - 1;
            },
            "rcv" => {
                // Part 1
                if last_sound != 0 {
                    return Some(last_sound);
                }
            }
            _ => panic!("Unknown instruction: {:?}", inst),
        }
        pc += 1;
    }
    None
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
        let mut memory = Memory::new();
        let result = execute(&get_input(), &mut memory).unwrap();
        println!("part1: {}\ttook: {}", result, now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    #[test]
    fn test_examples1() {
        let mut memory = Memory::new();
        assert_eq!(execute(TEST_INPUT, &mut memory), Some(4));
    }

    #[test]
    fn test_result1() {
        let mut memory = Memory::new();
        assert_eq!(execute(&get_input(), &mut memory), Some(3188));
    }
}
