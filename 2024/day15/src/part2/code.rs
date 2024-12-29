use core::{panic, time};
use std::{
    fmt::Display,
    fs,
    ops::{Add, AddAssign, Sub, SubAssign},
    thread, usize,
};

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

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn run(path: &str) {
    let input: String = fs::read_to_string(path).unwrap();
    let mut map = vec![];
    let mut moves = vec![];
    let mut robot_pos = Coord(0, 0);

    for (i, line) in input.lines().enumerate() {
        if line == "" {
            continue;
        }
        let mut tmp = vec![];

        for (j, c) in line.chars().enumerate() {
            if c == '@' {
                robot_pos = Coord(i as i64, j as i64);
            }

            if c == '#' || c == '.' || c == 'O' || c == '@' {
                tmp.push(c);
            } else {
                moves.push(c);
            }
        }

        if !tmp.is_empty() {
            map.push(tmp);
        }
    }

    move_robot(&mut map, &moves, &mut robot_pos);

    let mut sum = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'O' {
                sum += j + i * 100;
            }
        }
    }

    print!("Res is: {sum}");
}

fn move_robot(map: &mut [Vec<char>], moves: &[char], robot_pos: &mut Coord) {
    //pretty_print(map);
    for dir in moves {
        map[robot_pos.0 as usize][robot_pos.1 as usize] = '.';
        match dir {
            '^' => {
                valid_move(map, robot_pos, Direction::Up);
                map[robot_pos.0 as usize][robot_pos.1 as usize] = '^';
            }
            '>' => {
                valid_move(map, robot_pos, Direction::Right);
                map[robot_pos.0 as usize][robot_pos.1 as usize] = '>';
            }
            '<' => {
                valid_move(map, robot_pos, Direction::Left);
                map[robot_pos.0 as usize][robot_pos.1 as usize] = '<';
            }
            _ => {
                valid_move(map, robot_pos, Direction::Down);
                map[robot_pos.0 as usize][robot_pos.1 as usize] = 'v';
            }
        }
        //pretty_print(map);
        //map[robot_pos.0 as usize][robot_pos.1 as usize] = '@';
        //pretty_print(map);
        //thread::sleep(time::Duration::from_millis(250));
        //thread::sleep(time::Duration::from_millis(500));
        //print!(" {}[2J", 27 as char);
    }
}

fn valid_move(map: &mut [Vec<char>], robot_pos: &mut Coord, dir: Direction) {
    //println!("Robot before: {}", robot_pos);
    let next_move: Coord;
    match dir {
        Direction::Up => {
            println!("Move up");
            next_move = Coord(-1, 0)
        }
        Direction::Down => {
            println!("Move down");
            next_move = Coord(1, 0)
        }
        Direction::Left => {
            println!("Move left");
            next_move = Coord(0, -1)
        }
        Direction::Right => {
            println!("Move right");
            next_move = Coord(0, 1)
        }
    }

    let mut potential_move = *robot_pos + next_move;
    let mut tile = map[potential_move.0 as usize][potential_move.1 as usize];

    if !(tile == '#' || tile == 'O') {
        *robot_pos = potential_move;
    }

    let mut boxes = vec![];
    if tile == 'O' {
        boxes.push(potential_move);
        loop {
            potential_move = potential_move + next_move;
            tile = map[potential_move.0 as usize][potential_move.1 as usize];

            if tile == 'O' {
                boxes.push(potential_move);
            }

            if tile == '#' {
                boxes = vec![];
                break;
            }

            if tile == '.' {
                break;
            }
        }

        //println!("Boxes: {:?}", boxes);
        for el in boxes.iter() {
            //map[el.0 as usize][el.1 as usize] = '.';
            let tmp = *el + next_move;
            potential_move -= next_move;
            map[tmp.0 as usize][tmp.1 as usize] = 'O';
            //println!("tmp: {:?}", tmp);
        }

        if !boxes.is_empty() {
            *robot_pos = potential_move;
        }
    }
    //println!("Robot after: {}", robot_pos);
}

fn pretty_print(map: &[Vec<char>]) {
    for row in map {
        for c in row {
            print!("{c}");
        }
        println!();
    }
    println!();
}
