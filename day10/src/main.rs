extern crate time;

use time::now;

fn rev_sublist(input: &mut Vec<i32>, index: usize, len: usize) {
    let input_len = input.len();
    let mut sublist = Vec::with_capacity(len);
    {
        let len = if len > input_len {
            // This probably never happens :)
            len % input_len
        } else {
            len
        };

        for i in index..index + len {
            sublist.push(input[i % input_len]);
        }
    }
    let sublist = sublist.iter().rev().collect::<Vec<_>>();
    for i in index..index + len {
        input[i % input_len] = sublist[i - index].to_owned();
    }
}

fn hash(input: Vec<i32>, input_lengths: Vec<i32>, runs: usize) -> Vec<i32> {
    let mut elems = input.iter().map(|e| *e).collect::<Vec<i32>>();
    let mut cur_pos = 0;
    let mut skip_size = 0;

    for _ in 0..runs {
        for length in input_lengths.to_owned() {
            rev_sublist(&mut elems, cur_pos, length as usize);

            cur_pos = (cur_pos + length as usize + skip_size) % input.len();
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
            .map(|e| e.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let before = now();
        let result = calculate_result(&hash((0..256).collect::<Vec<_>>(), input, 1));
        println!("part1: {}\ttook: {}", result, now() - before);
    }
    {
        let input = get_input_part2(INPUT);
        // TODO: let result = hash((0..256).collect::<Vec<_>>(), input, 64);
        // Build the dense hash
        // Convert to hex
    }
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
            .map(|e| e.parse::<i32>().unwrap())
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
}

const INPUT: &'static str = "212,254,178,237,2,0,1,54,167,92,117,125,255,61,159,164";
