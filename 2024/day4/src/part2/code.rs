use std::{usize, vec};

pub fn run() {
    let input = include_str!("../input.txt");
    let mut xmas = vec![];

    for line in input.lines() {
        let mut row: Vec<&str> = line.split("").collect();
        row.pop();
        row.remove(0);
        xmas.push(row);
    }

    let mut sum = 0;
    for (x, row) in xmas.iter().enumerate() {
        for (y, el) in row.iter().enumerate() {
            sum += check_cross(&xmas, x, y, el);
        }
    }
    sum += 0;

    println!("\n\nPart 2 sum!: {sum} ");
}

fn check_cross(xmas: &Vec<Vec<&str>>, x: usize, y: usize, curr: &str) -> usize {
    if x == 0 || y == 0 {
        return 0;
    }

    if curr != "A" {
        return 0;
    }

    let Some(tmp) = xmas.get(x - 1) else {
        return 0;
    };

    let Some(top_left) = tmp.get(y - 1) else {
        return 0;
    };

    let Some(tmp) = xmas.get(x + 1) else {
        return 0;
    };

    let Some(bottom_left) = tmp.get(y - 1) else {
        return 0;
    };

    let Some(tmp) = xmas.get(x - 1) else {
        return 0;
    };

    let Some(top_right) = tmp.get(y + 1) else {
        return 0;
    };

    let Some(tmp) = xmas.get(x + 1) else {
        return 0;
    };

    let Some(bottom_right) = tmp.get(y + 1) else {
        return 0;
    };

    let mut is_cross = false;
    match (*top_right, *bottom_left) {
        ("M", "S") => {
            if is_cross {
                return 1;
            }
            is_cross = true;
        }
        ("S", "M") => {
            if is_cross {
                return 1;
            }
            is_cross = true;
        }
        (_, _) => {}
    }
    match (*top_left, *bottom_right) {
        ("M", "S") => {
            if is_cross {
                return 1;
            }
            is_cross = true;
        }
        ("S", "M") => {
            if is_cross {
                return 1;
            }
            is_cross = true;
        }
        (_, _) => {}
    }

    0
}
