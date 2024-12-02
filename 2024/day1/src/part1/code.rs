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

    let mut ordered_left: Vec<Number> = Vec::new();
    let mut ordered_right: Vec<Number> = Vec::new();

    let mut pos = 0;
    let mut num = 0;
    for _ in left.clone() {
        (left, num, pos) = get_min(left.clone());
        ordered_left.push(Number { num, pos });
    }

    for _ in right.clone() {
        (right, num, pos) = get_min(right.clone());
        ordered_right.push(Number { num, pos });
    }

    let mut distances: Vec<i32> = Vec::new();

    for (i, el) in ordered_left.iter().enumerate() {
        distances.push((el.num as i32 - ordered_right[i].num as i32).abs());
    }
    print!("\n");

    let result: i32 = distances.iter().sum();
    println!("Result: {result}");
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
    println!("{}", &line[0..5]);
    println!("{}", &line[8..line.len()]);
    left.push(line[0..5].trim().parse().expect("Can't parse left"));
    right.push(
        line[8..line.len()]
            .trim()
            .parse()
            .expect("Can't parse right"),
    );
}
