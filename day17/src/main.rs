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

    fn result(&self) -> i32 {
        self.buffer[self.pos + 1]
    }
}

fn main() {
    {
        let before = now();
        let mut spinlock = Spinlock::new(2018);
        for value in 1..2018 {
            spinlock.step_forward(INPUT, value);
        }
        println!("part1: {}\ttook: {}", spinlock.result(), now() - before);
    }
    {
        let before = now();
        let mut pos = 0;
        let mut result = 0;
        // Using a spinlock in memory is too expensive, just simulate it.
        for value in 1..50_000_001 {
            // Push the position based on the stepping, modulo with the value
            // (which is otherwise the size of the buffer - 1).
            pos = (pos + INPUT) % value + 1;
            // If we're at the beginning, part the value as being a possible result.
            if pos == 1 {
                result = value;
            }
        }
        println!("part1: {}\ttook: {}", result, now() - before);
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
        assert_eq!(spinlock.result(), 638);
    }
}

const INPUT: usize = 377;
