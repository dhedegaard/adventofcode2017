extern crate time;

use std::collections::HashSet;
use time::now;

// Since the value can only be 1, use a hashset to tell us whether
// the value is 1.
type Tape = HashSet<i64>;

struct BlueprintResult {
    position: i64,
    state: char,
}

// Implemented my blueprint input as a function.
fn blueprint(tape: &mut Tape, position: i64, state: char) -> BlueprintResult {
    match state {
        'a' => if !tape.contains(&position) {
            tape.insert(position);
            BlueprintResult {
                position: position + 1,
                state: 'b',
            }
        } else {
            tape.remove(&position);
            BlueprintResult {
                position: position + 1,
                state: 'f',
            }
        },
        'b' => if !tape.contains(&position) {
            BlueprintResult {
                position: position - 1,
                state: 'b',
            }
        } else {
            BlueprintResult {
                position: position - 1,
                state: 'c',
            }
        },
        'c' => if !tape.contains(&position) {
            tape.insert(position);
            BlueprintResult {
                position: position - 1,
                state: 'd',
            }
        } else {
            tape.remove(&position);
            BlueprintResult {
                position: position + 1,
                state: 'c',
            }
        },
        'd' => if !tape.contains(&position) {
            tape.insert(position);
            BlueprintResult {
                position: position - 1,
                state: 'e',
            }
        } else {
            BlueprintResult {
                position: position + 1,
                state: 'a',
            }
        },
        'e' => if !tape.contains(&position) {
            tape.insert(position);
            BlueprintResult {
                position: position - 1,
                state: 'f',
            }
        } else {
            tape.remove(&position);
            BlueprintResult {
                position: position - 1,
                state: 'd',
            }
        },
        'f' => if !tape.contains(&position) {
            tape.insert(position);
            BlueprintResult {
                position: position + 1,
                state: 'a',
            }
        } else {
            tape.remove(&position);
            BlueprintResult {
                position: position - 1,
                state: 'e',
            }
        },
        _ => panic!("Unknown state: {}", state),
    }
}

// Executes a given blueprint `steps` number of times, returns the number of
// enabled bits on the tape.
fn execute(blueprint: &Fn(&mut Tape, i64, char) -> BlueprintResult, steps: usize) -> usize {
    let mut tape = Tape::new();
    let mut position = 0;
    let mut state = 'a';
    for _ in 0..steps {
        let res = blueprint(&mut tape, position, state);
        position = res.position;
        state = res.state;
    }
    tape.len()
}

fn main() {
    let before = now();
    let result = execute(&blueprint, 12_425_180);
    println!("part1: {}\ttook: {}", result, now() - before);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Executes a single step, modifying the tape and returning the new
    // position and state.
    fn test_blueprint(tape: &mut Tape, position: i64, state: char) -> BlueprintResult {
        match state {
            'a' => if !tape.contains(&position) {
                tape.insert(position);
                BlueprintResult {
                    position: position + 1,
                    state: 'b',
                }
            } else {
                tape.remove(&position);
                BlueprintResult {
                    position: position - 1,
                    state: 'b',
                }
            },
            'b' => if !tape.contains(&position) {
                tape.insert(position);
                BlueprintResult {
                    position: position - 1,
                    state: 'a',
                }
            } else {
                BlueprintResult {
                    position: position + 1,
                    state: 'a',
                }
            },
            _ => panic!("Invalid state: {}", state),
        }
    }

    #[test]
    fn test_examples1() {
        assert_eq!(execute(&test_blueprint, 6), 3);
    }

    #[test]
    fn test_result1() {
        assert_eq!(execute(&blueprint, 12_425_180), 3099);
    }
}
