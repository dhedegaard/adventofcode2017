extern crate time;

use std::fs::File;
use std::io::Read;
use time::now;

type Memory = Vec<i64>;

fn parse_value(val: &str, memory: &Memory) -> i64 {
    match val.parse::<i64>() {
        Ok(num) => num,
        _ => memory[val.chars().next().unwrap() as usize - 'a' as usize],
    }
}

fn execute(instructions: &str, memory: &mut Memory) -> i64 {
    let instructions = instructions
        .lines()
        .map(|e| e.to_owned())
        .collect::<Vec<_>>();
    let mut result = 0;
    let mut pc = 0;
    while pc >= 0 && (pc as usize) < instructions.len() {
        let inst = instructions[pc as usize].split(" ").collect::<Vec<_>>();
        match inst[0] {
            "set" => {
                let inst2 = parse_value(inst[2], &memory);
                memory[inst[1].chars().next().unwrap() as usize - 'a' as usize] = inst2;
            }
            "sub" => {
                let inst2 = parse_value(inst[2], &memory);
                let mem_val = memory
                    .get_mut(inst[1].chars().next().unwrap() as usize - 'a' as usize)
                    .unwrap();
                *mem_val -= inst2;
            }
            "mul" => {
                let inst2 = parse_value(inst[2], &memory);
                let mem_val = memory
                    .get_mut(inst[1].chars().next().unwrap() as usize - 'a' as usize)
                    .unwrap();
                *mem_val *= inst2;
                result += 1;
            }
            "jnz" => if parse_value(inst[1], &memory) != 0 {
                pc += parse_value(inst[2], &memory) - 1;
            },
            _ => panic!("Unknown instruction: {}", inst[0]),
        }
        pc += 1;
    }
    result
}

fn part2() -> i64 {
    // The assembly translated into code, with all the useless instructions and
    // registers remove, to make it run really really fast.
    let mut b = 93 * 100 + 100000;
    let mut h = 0;
    let c = b + 17_000;
    loop {
        let mut f = 1;
        let mut d = 2;
        // e and g is never used for anything, so skip it.
        loop {
            if b % d == 0 {
                f = 0;
            }
            d += 1;
            if d != b {
                continue;
            }
            if f == 0 {
                h += 1;
            }
            if b == c {
                return h;
            }
            b += 17;
            break;
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
        let mut memory = vec![0; 8];
        let result = execute(&get_input(), &mut memory);
        println!("part1: {}\ttook: {}", result, now() - before);
    }
    {
        let before = now();
        let result = part2();
        println!("part2: {}\ttook: {}", result, now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result1() {
        let mut memory = vec![0; 8];
        assert_eq!(execute(&get_input(), &mut memory), 8281);
    }

    #[test]
    fn test_result2() {
        assert_eq!(part2(), 911);
    }
}
