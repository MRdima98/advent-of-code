use core::time;
use std::{collections::HashMap, thread, vec};

pub fn run() {
    let input = include_str!("../input");
    let mut stones: Vec<(u64, bool)> = vec![];

    for line in input.lines() {
        let tmp = line.split_whitespace();
        for num in tmp {
            stones.push((num.parse().unwrap(), false));
        }
    }

    let mut cache: HashMap<Vec<u64>, u64> = HashMap::new();
    cache.insert(vec![1], 0);

    for i in 0..25 {
        println!("Blink: {i}");
        for stone in stones.iter_mut() {
            stone.1 = false;
        }

        stones = blink(&mut stones.clone(), &mut cache);
    }

    print!("Stones part 2: {}\n", stones.iter().count());
}

fn blink(stones: &mut Vec<(u64, bool)>, cache: &mut HashMap<Vec<u64>, u64>) -> Vec<(u64, bool)> {
    if !stones[0].1 {
        let stone = stones.remove(0);
        stones.extend(calc_stone(stone.0).iter());
        return blink(stones, cache);
    }

    stones.to_vec()
}

fn calc_stone(stone: u64) -> Vec<(u64, bool)> {
    if stone == 0 {
        return vec![(1, true)];
    };

    let tmp = stone.to_string();
    if tmp.len() % 2 == 0 {
        let first_half: u64 = tmp[..tmp.len() / 2].parse().unwrap();
        let second_half: u64 = tmp[(tmp.len() / 2)..].parse().unwrap();

        return vec![(first_half, true), (second_half, true)];
    } else {
        return vec![(stone * 2024, true)];
    }
}
