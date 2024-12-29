use core::time;
use std::{
    fmt::Display,
    fs,
    ops::{Add, AddAssign, Sub},
    thread, usize,
};

use regex::Regex;

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd)]
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

    let mut iter = 0;
    let res;

    loop {
        iter += 1;
        move_robot(&mut positions, &speeds);
        let frame = next_frame(&positions);
        let tree = check_for_tree(&mut frame.clone(), &positions);
        println!("{iter}");
        //pretty_print2(&frame);
        if tree {
            res = iter;
            pretty_print(&positions);
            break;
        }
    }

    println!("Sum is: {res}");
}

fn check_for_tree(frame: &mut [Vec<String>], positions: &[Coord]) -> bool {
    let mamma_mia = "🤌".to_string();
    for pos in positions {
        let mut queue = vec![];
        if frame[pos.0 as usize][pos.1 as usize] == mamma_mia.clone() {
            continue;
        }
        let mut count = 0;
        frame[pos.0 as usize][pos.1 as usize] = mamma_mia.clone();
        queue.push(*pos);

        while !queue.is_empty() {
            let node = queue.pop().unwrap();
            let neighbours = get_neighbours(frame, node);
            for ghebur in neighbours {
                if frame[ghebur.0 as usize][ghebur.1 as usize] != mamma_mia.clone() {
                    queue.push(ghebur);
                    frame[ghebur.0 as usize][ghebur.1 as usize] = mamma_mia.clone();
                }
            }
            count += 1;
        }

        if count == 229 {
            return true;
        }

        if count > positions.len() / 2 {
            return true;
        }
    }

    false
}

fn get_neighbours(frame: &mut [Vec<String>], node: Coord) -> Vec<Coord> {
    let mut pot_neighbours = vec![];
    pot_neighbours.push(node - Coord(-1, 0));
    pot_neighbours.push(node - Coord(-1, -1));
    pot_neighbours.push(node - Coord(-1, 1));
    pot_neighbours.push(node - Coord(1, 0));
    pot_neighbours.push(node - Coord(1, -1));
    pot_neighbours.push(node - Coord(1, 1));
    pot_neighbours.push(node - Coord(0, -1));
    pot_neighbours.push(node - Coord(0, 1));
    let lower_bound = Coord(0, 0);
    let upper_bound = Coord(103, 101);
    let mut neighbours = vec![];
    let tree = "🌲".to_string();

    for neigh in pot_neighbours {
        if neigh.0 >= lower_bound.0
            && neigh.1 >= lower_bound.1
            && neigh.0 < upper_bound.0
            && neigh.1 < upper_bound.1
            && frame[neigh.0 as usize][neigh.1 as usize] == tree
        {
            neighbours.push(neigh)
        }
    }
    neighbours
}

fn next_frame(positions: &[Coord]) -> Vec<Vec<String>> {
    const ROWS: i64 = 103;
    const COLS: i64 = 101;
    let mut frame: Vec<Vec<String>> = vec![vec!["".to_string(); 101]; 103];

    for i in 0..ROWS {
        for j in 0..COLS {
            if positions.contains(&Coord(i, j)) {
                frame[i as usize][j as usize] = "🌲".to_string();
            } else {
                frame[i as usize][j as usize] = "⬜".to_string();
            }
        }
    }

    frame
}
fn move_robot(positions: &mut [Coord], speeds: &[Coord]) {
    const ROWS: i64 = 103;
    const COLS: i64 = 101;

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

fn pretty_print2(frame: &[Vec<String>]) {
    for row in frame {
        for el in row {
            print!("{el}");
        }
        println!();
    }
    println!();
}

fn pretty_print(positions: &[Coord]) {
    const ROWS: i64 = 103;
    const COLS: i64 = 101;
    for i in 0..ROWS {
        for j in 0..COLS {
            if positions.contains(&Coord(i, j)) {
                print!("🌲");
            } else {
                print!("⬜");
            }
        }
        println!();
    }
    print!(" {}[2J", 27 as char);
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
