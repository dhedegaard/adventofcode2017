extern crate time;

use time::now;

#[derive(Debug)]
struct Spinlock {
    buffer: Box<Vec<i32>>,
    pos: usize,
}

impl Spinlock {
    fn new(capacity: usize) -> Spinlock {
        // Allocate a buffer with the capacity required.
        let mut vec = Vec::with_capacity(capacity);
        vec.push(0);
        // Create the struct and return in.
        Spinlock {
            buffer: Box::new(vec),
            pos: 0,
        }
    }

    fn step_forward(&mut self, stepping: usize, value: i32) {
        // Move the position.
        self.pos = (self.pos + stepping) % self.buffer.len() + 1;
        // Insert the new value.
        self.buffer.insert(self.pos, value);
    }

    fn result1(&self) -> i32 {
        self.buffer[self.pos + 1]
    }

    fn result2(&self) -> i32 {
        let zeroes_position = self.buffer.iter().position(|e| *e == 0).unwrap();
        self.buffer[zeroes_position + 1]
    }
}

fn main() {
    {
        let before = now();
        let mut spinlock = Spinlock::new(2018);
        for value in 1..2018 {
            spinlock.step_forward(INPUT, value);
        }
        println!("part1: {}\ttook: {}", spinlock.result1(), now() - before);
    }
    {
        return;
        let before = now();
        let mut spinlock = Spinlock::new(50_000_001);
        for value in 1..50_000_001 {
            spinlock.step_forward(INPUT, value);
        }
        println!("part1: {}\ttook: {}", spinlock.result2(), now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_example1() {
        let mut spinlock = Spinlock::new(10);
        assert_eq!(spinlock.buffer, Box::new(vec![0]));
        assert_eq!(spinlock.pos, 0);
        spinlock.step_forward(3, 1);
        assert_eq!(spinlock.buffer, Box::new(vec![0, 1]));
        assert_eq!(spinlock.pos, 1);
        spinlock.step_forward(3, 2);
        assert_eq!(spinlock.buffer, Box::new(vec![0, 2, 1]));
        assert_eq!(spinlock.pos, 1);
        spinlock.step_forward(3, 3);
        assert_eq!(spinlock.buffer, Box::new(vec![0, 2, 3, 1]));
        assert_eq!(spinlock.pos, 2);
    }

    #[test]
    fn fn_examples1() {
        let mut spinlock = Spinlock::new(2018);
        for value in 1..2018 {
            spinlock.step_forward(3, value);
        }
        assert_eq!(spinlock.buffer[spinlock.pos], 2017);
        assert_eq!(spinlock.result1(), 638);
    }
}

const INPUT: usize = 377;
