extern crate time;

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;
use time::now;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn distance(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug, Eq)]
struct Particle {
    position: Box<Position>,
    velocity: Box<Position>,
    acceleration: Box<Position>,
    absolute_vector: i64,
    current_distance: i64,
    closets_distance: i64,
    moving_closer: bool,
    id: usize,
}

impl Particle {
    fn tick(&mut self) {
        // Add the acceleration to the velocity.
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;

        // Change position based on the velocity.
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;

        self.absolute_vector = self.velocity.distance();

        let prev_distance = self.current_distance;
        self.current_distance = self.position.distance();
        self.moving_closer = self.current_distance <= prev_distance;
    }
}

impl Ord for Particle {
    fn cmp(&self, other: &Particle) -> Ordering {
        self.current_distance.cmp(&other.current_distance)
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Particle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Particle) -> bool {
        self.id == other.id
    }
}

fn parse_position(input: &str) -> Position {
    let input = input
        .chars()
        .skip(3)
        .filter(|c| *c != '>')
        .collect::<String>();
    let coords = input
        .split(",")
        .map(|e| e.parse().unwrap())
        .collect::<Vec<_>>();
    Position {
        x: coords[0],
        y: coords[1],
        z: coords[2],
    }
}

fn parse(input: &str) -> Vec<Particle> {
    let mut result = vec![];
    for (id, line) in input.lines().enumerate() {
        let parts = line.split(", ")
            .map(|part| parse_position(part))
            .collect::<Vec<_>>();
        let p = Particle {
            position: Box::new(parts[0]),
            velocity: Box::new(parts[1]),
            acceleration: Box::new(parts[2]),
            absolute_vector: std::i64::MAX,
            current_distance: std::i64::MAX,
            closets_distance: std::i64::MAX,
            id: id,
            moving_closer: true,
        };
        result.push(p);
    }
    result
}

fn part1(particles: &mut Vec<Particle>) -> usize {
    loop {
        // Check whether all the particles are on their way away from the
        // center and whether the first element has the lowest velocity.
        let first_id = particles[0].id;
        if particles
            .iter()
            .filter(|p| p.moving_closer)
            .next()
            .is_none()
            && first_id
                == particles
                    .iter()
                    .min_by_key(|p| p.absolute_vector)
                    .unwrap()
                    .id
            && first_id
                == particles
                    .iter()
                    .min_by_key(|p| p.acceleration.distance())
                    .unwrap()
                    .id
        {
            break;
        }

        // Iterate everything
        for particle in particles.iter_mut() {
            particle.tick();
        }

        // Sort the particles based on distance.
        particles.sort();
    }

    // Just return the first one, being closest and with the lowest velocity.
    particles
        .iter()
        .min_by_key(|p| p.current_distance)
        .unwrap()
        .id
}

fn part2(particles: &mut Vec<Particle>) -> usize {
    for _ in 0..5_000 {
        // Check for collisions.
        let collision_positions = {
            let mut result = HashSet::new();
            for i in 0..particles.len() {
                let pos1 = &particles[i].position;
                for j in i + 1..particles.len() {
                    let pos2 = &particles[j].position;
                    if pos1 == pos2 {
                        // Mark the position.
                        result.insert(pos1.clone());
                    }
                }
            }
            result
        };

        // Remove all particles with positions in the hashset.
        if !collision_positions.is_empty() {
            for i in (0..particles.len()).rev() {
                if collision_positions.contains(&particles[i].position) {
                    particles.remove(i);
                }
            }
        }

        // Iterate everything
        for particle in particles.iter_mut() {
            particle.tick();
        }
    }

    // Return the particle count.
    particles.len()
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
        let mut particles = parse(&get_input());
        println!("part1: {}\ttook: {}", part1(&mut particles), now() - before);
    }
    {
        let before = now();
        let mut particles = parse(&get_input());
        println!("part2: {}\ttook: {}", part2(&mut particles), now() - before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";

    #[test]
    fn test_examples1() {
        let mut particles = parse(TEST_INPUT);
        assert_eq!(part1(&mut particles), 0);
    }

    #[test]
    fn test_result1() {
        let mut particles = parse(&get_input());
        assert_eq!(part1(&mut particles), 243);
    }

    #[test]
    fn test_result1() {
        let mut particles = parse(&get_input());
        assert_eq!(part2(&mut particles), 648);
    }
}
