extern crate time;

use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, VecDeque};
use time::now;

type Memory = HashMap<char, i64>;

#[derive(Debug)]
struct Process {
    pub queue: RefCell<VecDeque<i64>>,
    memory: Box<Memory>,
    pub pc: i64,
    instructions: Box<Vec<String>>,
    count_send: u64,
    last_sound: i64,
    pub other_process: Option<Rc<RefCell<Process>>>,
}

impl Process {
    fn new(instructions: &str) -> Process {
        let instructions = instructions
            .lines()
            .map(|e| e.to_owned())
            .collect::<Vec<_>>();
        Process {
            queue: RefCell::new(VecDeque::new()),
            memory: Box::new(Memory::new()),
            pc: 0,
            instructions: Box::new(instructions),
            last_sound: 0,
            count_send: 0,
            other_process: None,
        }
    }

    fn is_done(&self, old_program_counter: i64) -> bool {
        let queue = self.queue.borrow();
        queue.is_empty()
            && (self.pc < 0 || self.pc >= self.instructions.len() as i64
                || self.pc == old_program_counter)
    }

    fn execute(&mut self, part2: bool) {
        while self.pc >= 0 && self.pc < self.instructions.len() as i64 {
            let values = self.instructions[self.pc as usize]
                .split(" ")
                .skip(1)
                .map(|e| parse_value(e))
                .collect::<Vec<_>>();
            let inst = self.instructions[self.pc as usize]
                .split(" ")
                .next()
                .unwrap();
            match inst {
                "set" => {
                    let val1 = value_to_int(&values[1], &self.memory);
                    self.memory.insert(values[0].unwrap_char(), val1);
                }
                "add" => {
                    let val0 = value_to_int(&values[0], &self.memory);
                    let val1 = value_to_int(&values[1], &self.memory);
                    self.memory.insert(values[0].unwrap_char(), val0 + val1);
                }
                "mul" => {
                    let val0 = value_to_int(&values[0], &self.memory);
                    let val1 = value_to_int(&values[1], &self.memory);
                    self.memory.insert(values[0].unwrap_char(), val0 * val1);
                }
                "mod" => {
                    let val0 = value_to_int(&values[0], &self.memory);
                    let val1 = value_to_int(&values[1], &self.memory);
                    self.memory.insert(values[0].unwrap_char(), val0 % val1);
                }
                "snd" => if part2 {
                    let val0 = value_to_int(&values[0], &self.memory);
                    self.count_send += 1;
                    match self.other_process.as_ref() {
                        Some(process) => {
                            let process = process.borrow();
                            let mut queue = process.queue.borrow_mut();
                            queue.push_back(val0);
                        }
                        None => panic!("Missing reference to other_process"),
                    }
                } else {
                    self.last_sound = value_to_int(&values[0], &self.memory);
                },
                "rcv" => {
                    if part2 {
                        // Pop the queue, if applicable.
                        if self.queue.borrow().is_empty() {
                            // If we're empty, stop and await for the next
                            // execute().
                            return;
                        } else {
                            // Otherwise, pop the queue and store the value on
                            // the memory location.
                            self.memory.insert(
                                values[0].unwrap_char(),
                                self.queue.borrow_mut().pop_front().unwrap(),
                            );
                        }
                    } else if self.last_sound != 0 {
                        // If we've received after last_sound being set, then
                        // part1 is complete.
                        return;
                    }
                }
                "jgz" => if value_to_int(&values[0], &self.memory) > 0 {
                    self.pc += value_to_int(&values[1], &self.memory) - 1;
                },
                _ => panic!("Unknown instruction: {:?}", inst),
            }
            self.pc += 1;
            // If we're running part2, break between executions.
            if part2 {
                break;
            }
        }
    }
}

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

impl Value {
    fn unwrap_char(&self) -> char {
        match self {
            &Value::Char(c) => c,
            _ => panic!("Value is not a Char: {:?}", self),
        }
    }
}

fn part2(input: &str) -> u64 {
    // Create the processes.
    let process1 = Rc::new(RefCell::new(Process::new(input)));
    let process2 = Rc::new(RefCell::new(Process::new(input)));
    // Set the "p" register and relate the processes to each other.
    {
        let mut process1 = process1.as_ref().borrow_mut();
        process1.other_process = Some(Rc::clone(&process2));
        process1.memory.insert('p', 0);
    }
    {
        let mut process2 = process2.as_ref().borrow_mut();
        process2.other_process = Some(Rc::clone(&process1));
        process2.memory.insert('p', 1);
    }
    // Execute on each process, until they're both done and both have an empty queue.
    loop {
        // Register the values of the program counters before execution.
        let program_counters = { (process1.as_ref().borrow().pc, process2.as_ref().borrow().pc) };

        // Execute on both processes.
        {
            let mut process1 = process1.as_ref().borrow_mut();
            process1.execute(true);
        }
        {
            let mut process2 = process2.as_ref().borrow_mut();
            process2.execute(true);
        }

        // If both queues are empty, we just executed and the program counters
        // are stuch, then both programs ended up in deadlock.
        let process1_done = {
            let process = process1.as_ref().borrow();
            process.is_done(program_counters.0)
        };
        let process2_done = {
            let process = process2.as_ref().borrow();
            process.is_done(program_counters.1)
        };
        if process1_done && process2_done {
            break;
        }
    }

    // Fetch the send count from process with ID: 1 and return it
    {
        let process2 = process2.as_ref().borrow();
        return process2.count_send;
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
        let mut process = Process::new(&get_input());
        process.execute(false);
        println!("part1: {}\ttook: {}", process.last_sound, now() - before);
    }
    {
        let before = now();
        let result = part2(&get_input());
        println!("part2: {}\ttook: {}", result, now() - before);
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
        let mut process = Process::new(TEST_INPUT);
        process.execute(false);
        assert_eq!(process.last_sound, 4);
    }

    #[test]
    fn test_result1() {
        let mut process = Process::new(&get_input());
        process.execute(false);
        assert_eq!(process.last_sound, 3188);
    }

    const TEST_INPUT2: &'static str = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

    #[test]
    fn test_examples2() {
        let result = part2(TEST_INPUT2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_result2() {
        let result = part2(&get_input());
        assert_eq!(result, 7112);
    }
}
