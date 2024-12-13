use core::time;
use std::{collections::HashMap, thread, usize};

pub fn run() {
    let input = include_str!("../input");
    let mut stones: HashMap<usize, usize> = HashMap::new();

    for line in input.lines() {
        let tmp = line.split_whitespace();
        for num in tmp {
            stones.entry(num.parse().unwrap()).or_insert(1);
        }
    }

    for i in 0..75 {
        stones = blink(&stones);
    }

    let mut sum: usize = stones.values().sum();

    print!("\nStones part 2: {}\n", sum);
}

fn blink(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut keys = stones.clone().into_keys();
    let mut new_stones: HashMap<usize, usize> = HashMap::new();

    for key in keys {
        let mut val = stones.get(&key).unwrap();

        if key == 0 {
            new_stones
                .entry(1)
                .and_modify(|el| *el += *val)
                .or_insert(*val);
            continue;
        }

        let tmp = key.to_string();
        if tmp.len() % 2 == 0 {
            let first_half = tmp[..tmp.len() / 2].parse().unwrap();
            let second_half = tmp[(tmp.len() / 2)..].parse().unwrap();

            new_stones
                .entry(first_half)
                .and_modify(|el| *el += *val)
                .or_insert(*val);
            new_stones
                .entry(second_half)
                .and_modify(|el| *el += *val)
                .or_insert(*val);
        } else {
            new_stones
                .entry(key * 2024)
                .and_modify(|el| *el += *val)
                .or_insert(*val);
        }
    }

    new_stones
}
