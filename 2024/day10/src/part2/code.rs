use core::time;
use std::{thread, usize};

pub fn run() {
    let input = include_str!("../input");
    let mut map = vec![];
    let mut starting_pos = vec![];

    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            let num = c.to_digit(10).unwrap() as usize;
            row.push(num);
            if num == 0 {
                starting_pos.push((i, j));
            }
        }
        map.push(row);
    }

    let mut sum = 0;
    for start in starting_pos {
        sum += get_path_grade(&map, start, start);
    }

    //print!("The res is: {sum}\n\n");
}

fn get_path_grade(map: &[Vec<usize>], curr_coord: (usize, usize), prev: (usize, usize)) -> usize {
    let Some(row) = map.get(curr_coord.0) else {
        return 0;
    };

    let Some(curr) = row.get(curr_coord.1) else {
        return 0;
    };

    let up = (curr_coord.0 - 1, curr_coord.1);
    let down = (curr_coord.0 + 1, curr_coord.1);
    let left = (curr_coord.0, curr_coord.1 - 1);
    let right = (curr_coord.0, curr_coord.1 - 1);

    if curr_coord == prev {
        return get_path_grade(map, up, curr_coord)
            + get_path_grade(map, down, curr_coord)
            + get_path_grade(map, left, curr_coord)
            + get_path_grade(map, right, curr_coord);
    }

    if prev == up {
        return get_path_grade(map, down, curr_coord)
            + get_path_grade(map, left, curr_coord)
            + get_path_grade(map, right, curr_coord);
    }

    if prev == left {
        return get_path_grade(map, up, curr_coord)
            + get_path_grade(map, down, curr_coord)
            + get_path_grade(map, right, curr_coord);
    }

    if prev == right {
        return get_path_grade(map, up, curr_coord)
            + get_path_grade(map, down, curr_coord)
            + get_path_grade(map, left, curr_coord);
    }

    if *curr == 9 {
        return 1;
    }

    return 0;
}
