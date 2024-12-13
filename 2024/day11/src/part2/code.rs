use core::time;
use std::{collections::HashMap, thread};

pub fn run() {
    let input = include_str!("../input");
    let mut stones: HashMap<usize, usize> = HashMap::new();

    for line in input.lines() {
        let tmp = line.split_whitespace();
        for num in tmp {
            stones.entry(num.parse().unwrap()).or_insert(1);
        }
    }

    for i in 0..25 {
        println!("Blink {i}, {:?}", stones);
        blink(&mut stones);
        //if i == 7 {
        //    break;
        //}
    }

    let mut sum: usize = stones.values().sum();

    //for el in stones.values() {
    //    sum += *el;
    //}

    print!("\nStones part 2: {}\n", sum);
}

fn blink(stones: &mut HashMap<usize, usize>) {
    let mut keys: Vec<_> = stones.clone().into_keys().collect();
    //println!("{:?}", keys);
    //println!("{:?}", stones.clone().values());
    //println!();
    //thread::sleep(time::Duration::from_millis(300));
    keys.sort();
    for key in keys {
        //println!("{:?}", stones);
        //thread::sleep(time::Duration::from_millis(300));
        //let val = stones.get(&key).unwrap().clone();
        let mut val = 1;
        if let Some(tmp) = stones.remove(&key) {
            val = tmp;
        }

        if key == 0 {
            stones.entry(1).and_modify(|el| *el += val).or_insert(val);
            continue;
        }

        let tmp = key.to_string();
        if tmp.len() % 2 == 0 {
            let first_half = tmp[..tmp.len() / 2].parse().unwrap();
            let second_half = tmp[(tmp.len() / 2)..].parse().unwrap();

            stones
                .entry(first_half)
                .and_modify(|el| *el += val)
                .or_insert(val);
            stones
                .entry(second_half)
                .and_modify(|el| *el += val)
                .or_insert(val);
        } else {
            stones
                .entry(key * 2024)
                .and_modify(|el| *el += val)
                .or_insert(val);
        }
    }
}
