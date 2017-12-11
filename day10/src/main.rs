extern crate time;

use time::now;
use std::fmt::Write;

fn rev_sublist(input: &mut Vec<i32>, index: usize, len: usize) {
    let input_len = input.len();
    let mut sublist = Vec::with_capacity(len);
    {
        for i in index..index + len {
            sublist.push(input[i % input_len] % 256);
        }
    }
    let sublist = sublist.iter().rev().collect::<Vec<_>>();
    for i in index..index + len {
        input[i % input_len] = sublist[i - index].to_owned() % 256;
    }
}

fn hash(input: Vec<i32>, input_lengths: Vec<usize>, runs: usize) -> Vec<i32> {
    let mut elems = input.iter().map(|e| *e).collect::<Vec<i32>>();
    let mut cur_pos = 0;
    let mut skip_size = 0;

    for _ in 0..runs {
        for length in input_lengths.to_owned() {
            rev_sublist(&mut elems, cur_pos, length);

            cur_pos = (cur_pos + length + skip_size) % input.len();
            skip_size += 1;
        }
    }
    elems
}

fn calculate_result(hash: &Vec<i32>) -> i32 {
    return hash[0] * hash[1];
}

fn main() {
    {
        let input = INPUT
            .split(",")
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let before = now();
        let result = calculate_result(&hash((0..256).collect::<Vec<_>>(), input, 1));
        println!("part1: {}\ttook: {}", result, now() - before);
    }
    {
        let input = get_input_part2(INPUT);
        let before = now();
        let result = part2(&input);
        println!("part2: {}\ttook: {}", result, now() - before);
    }
}

fn part2(input: &Vec<u8>) -> String {
    // Do the hashing rounds.
    let result = hash(
        (0..256).collect::<Vec<_>>(),
        input.iter().map(|e| *e as usize).collect::<Vec<_>>(),
        64,
    ).iter()
        .map(|e| *e as u8)
        .collect::<Vec<_>>();
    // Build the dense hash
    let mut dense_result = Vec::with_capacity(result.len() / 16);
    for i in 0..result.len() / 16 {
        let slice = result
            .iter()
            .skip(i * 16)
            .take(16)
            .map(|e| *e)
            .collect::<Vec<_>>();
        dense_result.push(dense_hash(&slice));
    }
    // Convert to hex
    vec_to_hex(&dense_result)
}

fn dense_hash(sparse_hash: &Vec<u8>) -> u8 {
    let mut result = 0_u8;
    for e in sparse_hash {
        result ^= e;
    }
    result
}

fn vec_to_hex(input: &Vec<u8>) -> String {
    let mut result = String::with_capacity(input.len() * 2);
    for b in input {
        write!(&mut result, "{:02x}", b).unwrap();
    }
    result
}

fn get_input_part2(input: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(input.len() + 5);
    result.extend(input.as_bytes());
    result.extend(&[17, 31, 73, 47, 23]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rev_list() {
        {
            let mut input = vec![0, 1, 2, 3, 4];
            rev_sublist(&mut input, 0, 3);
            assert_eq!(input, vec![2, 1, 0, 3, 4]);
        }
        {
            let mut input = vec![2, 1, 0, 3, 4];
            rev_sublist(&mut input, 3, 4);
            assert_eq!(input, vec![4, 3, 0, 1, 2]);
        }
    }

    #[test]
    fn test_examples1() {
        assert_eq!(
            hash(vec![0, 1, 2, 3, 4], vec![3, 4, 1, 5], 1),
            vec![3, 4, 2, 1, 0]
        );
    }

    #[test]
    fn test_result1() {
        let input = INPUT
            .split(",")
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let calculated_hash = hash((0..256).collect::<Vec<_>>(), input, 1);
        assert_eq!(calculate_result(&calculated_hash), 212);
    }

    #[test]
    fn test_parse2() {
        let input = "1,2,3";
        assert_eq!(
            get_input_part2(&input),
            vec![49, 44, 50, 44, 51, 17, 31, 73, 47, 23]
        );
    }

    #[test]
    fn test_vec_to_hex() {
        assert_eq!(vec_to_hex(&vec![64, 7, 255]), "4007ff");
    }

    #[test]
    fn test_dense_hash() {
        let input = vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
        assert_eq!(dense_hash(&input), 64);
    }

    #[test]
    fn test_examples2() {
        assert_eq!(
            part2(&get_input_part2("")),
            "a2582a3a0e66e6e86e3812dcb672a272"
        );
        assert_eq!(
            part2(&get_input_part2("AoC 2017")),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(
            part2(&get_input_part2("1,2,3")),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
        assert_eq!(
            part2(&get_input_part2("1,2,4")),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }

    #[test]
    fn test_result2() {
        let input = get_input_part2(INPUT);
        assert_eq!(part2(&input), "96de9657665675b51cd03f0b3528ba26");
    }
}

const INPUT: &'static str = "212,254,178,237,2,0,1,54,167,92,117,125,255,61,159,164";
