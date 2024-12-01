use core::str;
use std::{cmp::min_by, fs::read_to_string, i32::MAX};

struct Number {
    num: i32,
    pos: usize,
}

pub fn run() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut example = "src/example_input.txt";
    let mut input = "src/part1/input.txt";

    for line in read_to_string(input).expect("Did not read file").lines() {
        line_to_num(&mut left, &mut right, line);
    }

    let mut result = 0;
    for l in left {
        let mut count = 0;
        for r in right.clone() {
            if l == r {
                count += 1;
            }
        }
        result += count * l;
    }
    println!("Result {result}");
}

fn get_min(mut nums: Vec<i32>) -> (Vec<i32>, i32, usize) {
    let mut pos: usize = 0;
    let mut min: i32 = MAX;
    for (i, el) in nums.iter().enumerate() {
        if *el < min {
            min = *el;
            pos = i;
        }
    }
    nums[pos] = MAX;
    (nums, min, pos)
}

fn line_to_num(left: &mut Vec<i32>, right: &mut Vec<i32>, line: &str) {
    let mut tmp = str::split_ascii_whitespace(line);
    left.push(
        tmp.next()
            .unwrap()
            .trim()
            .parse()
            .expect("Can't parse left"),
    );
    right.push(
        tmp.next()
            .unwrap()
            .trim()
            .parse()
            .expect("Can't parse right"),
    );
}
