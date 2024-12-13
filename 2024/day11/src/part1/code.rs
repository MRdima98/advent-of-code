use core::time;
use std::thread;

pub fn run() {
    let input = include_str!("../input");
    let mut stones: Vec<u64> = vec![];

    for line in input.lines() {
        let tmp = line.split_whitespace();
        for num in tmp {
            stones.push(num.parse().unwrap());
        }
    }

    for i in 0..25 {
        //println!("{:?}", stones);
        //if i == 5 {
        //    break;
        //}
        //blink(&mut stones);
    }

    print!("Stones part 1: {}\n", stones.iter().count());
}

fn blink(stones: &mut Vec<u64>) {
    let mut i = 0;
    let mut moded = false;
    loop {
        let Some(stone) = stones.get_mut(i) else {
            break;
        };

        if *stone == 0 {
            *stone = 1;
            moded = true;
            continue;
        }

        let tmp = stone.to_string();
        thread::sleep(time::Duration::from_millis(300));
        if tmp.len() % 2 == 0 {
            stones.remove(i);
            let first_half: u64 = tmp[..tmp.len() / 2].parse().unwrap();
            let second_half: u64 = tmp[(tmp.len() / 2)..].parse().unwrap();

            stones.insert(i, first_half);
            stones.insert(i + 1, second_half);
            i = i + 1;
        } else {
            if !moded {
                *stone = *stone * 2024;
            }
        }
        i += 1;
        moded = false;
    }
}
