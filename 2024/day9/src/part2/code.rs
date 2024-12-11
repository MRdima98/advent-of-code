use core::{num, time};
use std::{fmt::Display, fs::copy, ops::RangeBounds, thread};

use regex::{Match, Regex};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Disk {
    Val(usize),
    Dot,
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Disk::Val(val) => write!(f, "{val}"),
            Disk::Dot => write!(f, "."),
        }
    }
}

pub fn run() {
    let input = include_str!("../input");
    let mut disk = vec![];
    let mut unpacked_disk = vec![];

    for line in input.lines() {
        for el in line.chars() {
            disk.push(el.to_digit(10).unwrap());
        }
    }

    let mut id = 0;
    let mut checked_id: Vec<(usize, bool)> = vec![];
    let mut num_string = vec![];
    for (i, el) in disk.iter().enumerate() {
        let tmp: Disk;
        if i % 2 == 0 {
            tmp = Disk::Val(id);
            checked_id.push((id, false));
            num_string.push("".to_string());
            id += 1;
        } else {
            tmp = Disk::Dot;
        }
        for _ in 0..*el {
            if let Disk::Val(el) = tmp {
                num_string[el] += (el).to_string().as_str();
            }
            unpacked_disk.push(tmp);
        }
    }

    let mut string_disk = "".to_string();
    for el in unpacked_disk.iter() {
        string_disk += el.to_string().as_str();
    }

    println!("{string_disk}");
    let mut last_end = string_disk.len();
    for num in num_string.iter().rev() {
        let clone = string_disk.clone();
        println!("\n\nRE: {num}");
        //println!("{string_disk}");
        let num_re = Regex::new(num).expect("This is a balls regex");
        let num_match = num_re
            .captures(&clone[0..last_end])
            .unwrap()
            .iter()
            .last()
            .unwrap()
            .unwrap();

        println!("match: {}", num_match.as_str());
        println!("{}", string_disk.chars().nth(num_match.start()).unwrap());
        for i in 1..120 {
            print!(
                "{}",
                string_disk
                    .chars()
                    .nth((string_disk.len() - 120) + i)
                    .unwrap()
            );
        }
        last_end = num_match.start();
        //println!("{}", num_match.as_str());

        //if num_match.start() != 0 {
        //    if let Some(prefix) = clone.chars().nth(num_match.start() - 1) {
        //        if prefix != '.' {
        //            continue;
        //        }
        //    }
        //}

        //if let Some(suffix) = clone.chars().nth(num_match.end() + 1) {
        //    if suffix != '.' {
        //        continue;
        //    }
        //}

        let Some(space) = get_space(&clone, num_match.len(), num_match.start()) else {
            continue;
        };

        if space.end() >= num_match.start() {
            break;
        }

        //println!("{}\n", space.as_str());
        let end = space.end() - (space.len() - num_match.len());
        string_disk.replace_range(space.start()..end, num_match.as_str());
        string_disk.replace_range(
            num_match.start()..num_match.end(),
            &space.as_str()[0..num_match.len()],
        );

        //thread::sleep(time::Duration::from_millis(500));
    }

    let mut sum = 0;
    for (i, el) in string_disk.chars().enumerate() {
        if let Some(num) = el.to_digit(10) {
            sum += num * i as u32;
        }
    }

    println!("The res is: {sum}");
}

fn get_space(string_disk: &str, len: usize, start: usize) -> Option<Match<'_>> {
    let space_re = Regex::new(r"\.+").expect("This is a balls regex");
    for cap in space_re.captures_iter(&string_disk) {
        for c in cap.iter() {
            let Some(space) = c else {
                continue;
            };
            if space.len() >= len && space.start() < start {
                return Some(space);
            }
        }
    }

    None
}

fn get_id_coord(string_disk: &String, id: usize) -> (usize, usize) {
    let string_id = id.to_string();
    let start = string_disk.find(&string_id).unwrap();
    let end = start + string_id.len() - 1;

    (start, end)
}

fn get_space_coord(unpacked_disk: &[Disk], size: usize, limit: usize) -> Option<(usize, usize)> {
    let mut space = vec![];
    for (i, el) in unpacked_disk.iter().enumerate() {
        if i == limit {
            return None;
        }

        match el {
            Disk::Dot => {
                space.push(i);
            }
            _ => {
                if space.len() >= size {
                    println!("Space size: {}. Num size: {}", space.len(), size);
                    return Some((space[0], space[space.len() - 1]));
                }

                space = vec![];
            }
        }
    }
    None
}

//fn get_id_coord(unpacked_disk: &[Disk], id: usize) -> (usize, usize) {
//    let mut coord = (0, unpacked_disk.len() - 1);
//    let mut hit = false;
//    for (i, el) in unpacked_disk.iter().enumerate() {
//        let tmp;
//        match el {
//            Disk::Val(tmp2) => {
//                tmp = tmp2;
//            }
//            Disk::Dot => {
//                if hit {
//                    println!("Second hit: {i}");
//                    hit = false;
//                    coord.1 = i - 1;
//                }
//                continue;
//            }
//        }
//
//        if *tmp == id && !hit {
//            thread::sleep(time::Duration::from_millis(200));
//            println!("First hit: {i}");
//            coord.0 = i;
//            hit = true;
//        }
//
//        if *tmp != id && hit {
//            println!("Second hit: {i}");
//            hit = false;
//            coord.1 = i - 1;
//        }
//    }
//    coord
//}

fn pretty_print(arr: &[Disk]) {
    for el in arr {
        print!("{el}");
    }
    println!();
}
