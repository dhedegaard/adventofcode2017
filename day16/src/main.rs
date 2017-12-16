extern crate time;

use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use time::now;

fn generate_programs(to_char: char) -> Vec<char> {
    (('a' as u8)..(to_char as u8) + 1)
        .map(|c| c as char)
        .collect::<Vec<_>>()
}

fn get_input() -> String {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input
}

#[derive(Copy, Clone, Debug)]
struct InstSpin {
    spin: usize,
}
#[derive(Copy, Clone, Debug)]
struct InstExchange {
    pos1: usize,
    pos2: usize,
}
#[derive(Copy, Clone, Debug)]
struct InstPartner {
    pos1: char,
    pos2: char,
}
#[derive(Debug)]
enum Instruction {
    Spin(InstSpin),
    Exchange(InstExchange),
    Partner(InstPartner),
}

fn execute(instructions: &Vec<Instruction>, programs: &mut Vec<char>) {
    let len = programs.len();
    for inst in instructions {
        match inst {
            &Instruction::Spin(ref inst) => {
                let tmp = Vec::from_iter(programs.iter().map(|c| *c));
                for i in 0..len {
                    programs[(i + inst.spin) % len as usize] = tmp[i];
                }
            }
            &Instruction::Exchange(ref inst) => {
                let tmp = programs[inst.pos1];
                programs[inst.pos1] = programs[inst.pos2];
                programs[inst.pos2] = tmp;
            }
            &Instruction::Partner(ref inst) => {
                // Slow :(
                let pos1 = programs.iter().position(|e| *e == inst.pos1).unwrap();
                let pos2 = programs.iter().position(|e| *e == inst.pos2).unwrap();
                let tmp = programs[pos1];
                programs[pos1] = programs[pos2];
                programs[pos2] = tmp;
            }
        }
    }
}

fn parse_instructions(instructions: &str, program_len: usize) -> Vec<Instruction> {
    let mut result: Vec<Instruction> = Vec::new();
    for inst in instructions.split(",") {
        match &inst[0..1] {
            "s" => {
                let spin = inst[1..].parse::<i32>().unwrap();
                let spin = ((spin + program_len as i32) % program_len as i32) as usize;
                result.push(Instruction::Spin(InstSpin { spin: spin }));
            }
            "x" => {
                let inst = inst[1..]
                    .split("/")
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                result.push(Instruction::Exchange(InstExchange {
                    pos1: inst[0],
                    pos2: inst[1],
                }));
            }
            "p" => {
                let inst = inst[1..]
                    .split("/")
                    .map(|e| e.parse::<char>().unwrap())
                    .collect::<Vec<_>>();
                result.push(Instruction::Partner(InstPartner {
                    pos1: inst[0],
                    pos2: inst[1],
                }));
            }
            _ => panic!("Don't know how to handle: {:?}", &inst[0..1]),
        }
    }
    result
}

fn main() {
    {
        let mut input = generate_programs('p');
        let instructions = parse_instructions(&get_input(), input.len());
        let before = now();
        execute(&instructions, &mut input);
        println!(
            "part1: {}\ttook: {}",
            input.iter().collect::<String>(),
            now() - before
        );
    }
    {
        let initial_input = generate_programs('p');
        let mut input = generate_programs('p');
        let instructions = parse_instructions(&get_input(), input.len());
        let before = now();
        let mut iterations = 1_000_000_000;
        let mut iteration = 0;
        while iteration < iterations {
            execute(&instructions, &mut input);
            iteration += 1;
            // When we find the loop length, reduce the number of iterations
            // and reset the iteration counter.
            if input == initial_input {
                iterations = (iterations / instructions.len()) % iteration;
                iteration = 0;
            }
        }
        println!(
            "part2: {}\ttook: {}",
            input.iter().collect::<String>(),
            now() - before
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let input = generate_programs('e');
        assert_eq!(input, vec!['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn test_examples1() {
        let mut input = generate_programs('e');
        let instructions = parse_instructions(TEST_INPUT, input.len());
        execute(&instructions, &mut input);
        assert_eq!(input, vec!['b', 'a', 'e', 'd', 'c']);
    }

    #[test]
    fn test_result1() {
        let mut input = generate_programs('p');
        let instructions = parse_instructions(&get_input(), input.len());
        execute(&instructions, &mut input);
        assert_eq!(input.iter().collect::<String>(), "namdgkbhifpceloj");
    }

    #[test]
    fn test_examples2() {
        let mut input = generate_programs('e');
        let instructions = parse_instructions(TEST_INPUT, input.len());
        execute(&instructions, &mut input);
        execute(&instructions, &mut input);
        assert_eq!(input, vec!['c', 'e', 'a', 'd', 'b']);
    }

    #[test]
    fn test_result2() {
        let initial_input = generate_programs('p');
        let mut input = generate_programs('p');
        let instructions = parse_instructions(&get_input(), input.len());
        let mut iterations = 1_000_000_000;
        let mut iteration = 0;
        while iteration < iterations {
            execute(&instructions, &mut input);
            iteration += 1;
            // When we find the loop length, reduce the number of iterations
            // and reset the iteration counter.
            if input == initial_input {
                iterations = (iterations / instructions.len()) % iteration;
                iteration = 0;
            }
        }
        assert_eq!(input.iter().collect::<String>(), "ibmchklnofjpdeag");
    }

    const TEST_INPUT: &'static str = "s1,x3/4,pe/b";
}
