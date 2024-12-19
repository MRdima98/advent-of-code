use std::{
    fmt::Display,
    fs,
    ops::{Add, AddAssign, Sub},
};

use regex::Regex;

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
struct Coord(i64, i64);

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Add for Coord {
    type Output = Coord;
    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Coord {
    type Output = Coord;
    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub fn run(path: &str) {
    let input: String = fs::read_to_string(path).unwrap();
    let mut positions = vec![];
    let mut speeds = vec![];

    for line in input.lines() {
        let tmp = get_nums(line);
        positions.push(tmp.0);
        speeds.push(tmp.1);
    }

    let res = move_robot(&mut positions, &speeds);

    println!("Sum is: {res}");
}

fn move_robot(positions: &mut [Coord], speeds: &[Coord]) -> u32 {
    const ROWS: i64 = 103;
    const COLS: i64 = 101;

    for _ in 0..100 {
        for (i, pos) in positions.iter_mut().enumerate() {
            *pos += speeds[i];

            if pos.0 <= 0 {
                pos.0 += ROWS;
            }

            if pos.1 <= 0 {
                pos.1 += COLS;
            }

            if pos.0 >= ROWS {
                pos.0 -= ROWS;
            }

            if pos.1 >= COLS {
                pos.1 -= COLS;
            }
        }
    }

    let mid_row = ROWS / 2;
    let cols_row = COLS / 2;
    let mut quadrant = vec![0; 4];

    for pos in positions {
        if pos.0 < mid_row && pos.1 < cols_row {
            println!("Quad 1: {}, {}", pos.1, pos.0);
            quadrant[0] += 1;
        }

        if pos.0 < mid_row && pos.1 > cols_row {
            println!("Quad 2: {}, {}", pos.1, pos.0);
            quadrant[1] += 1;
        }

        if pos.0 > mid_row && pos.1 < cols_row {
            println!("Quad 3: {}, {}", pos.1, pos.0);
            quadrant[2] += 1;
        }

        if pos.0 > mid_row && pos.1 > cols_row {
            println!("Quad 4: {}, {}", pos.1, pos.0);
            quadrant[3] += 1;
        }
    }

    quadrant.into_iter().reduce(|acc, el| acc * el).unwrap()
}

fn get_nums(line: &str) -> (Coord, Coord) {
    let num_re = Regex::new(r"-*\d+").expect("This is a balls regex");
    let mut matches = num_re.captures_iter(line);
    let y: i64 = matches
        .next()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let x: i64 = matches
        .next()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let y_m: i64 = matches
        .next()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let x_m: i64 = matches
        .next()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    (Coord(x, y), Coord(x_m, y_m))
}
