use core::time;
use std::{fmt::Display, thread, time::SystemTime, usize, vec};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Going {
    pos: Position,
    dir: Direction,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Right => write!(f, "Right"),
            Direction::Left => write!(f, "Left"),
        }
    }
}

pub fn run() {
    let input = include_str!("../input");
    let mut map = vec![];
    let mut row: Vec<char>;
    let mut guard_coord = Position { x: 0, y: 0 };

    for (i, line) in input.lines().enumerate() {
        if let Some(guard) = line.find("^") {
            guard_coord = Position { x: i, y: guard };
        }
        row = line.chars().collect();
        map.push(row);
    }

    let backup = guard_coord.clone();

    let mut direction = Direction::Up;
    let mut count = 0;
    let mut distinct_pos: Vec<Position> = vec![];

    loop {
        if !distinct_pos.contains(&guard_coord) {
            distinct_pos.push(guard_coord.clone());
        }

        if !valid_block(&mut guard_coord, &map, &mut direction) {
            break;
        }
    }

    println!("Distinct pos: {:?}", distinct_pos.iter().count());

    for (i, g) in distinct_pos.iter().enumerate() {
        println!("Iter {i}");
        guard_coord = backup.clone();

        let mut loop_detector: Vec<Going> = vec![];
        direction = Direction::Up;

        if let Some(pos) = distinct_pos.get(i + 1) {
            map[pos.x][pos.y] = '#';
        };

        loop {
            let curr = Going {
                pos: guard_coord,
                dir: direction,
            };

            if !loop_detector.contains(&curr) {
                loop_detector.push(curr);
            } else {
                count += 1;
                break;
            }

            if !valid_block(&mut guard_coord, &map, &mut direction) {
                break;
            }
        }
        map[g.x][g.y] = '.';
    }

    print!("\nNum of steps: {}\n\n", count);
}

fn pretty_print(fake_map: &Vec<Vec<char>>) {
    for row in fake_map {
        let row: String = row.clone().iter().collect();
        println!("{row}");
    }
    println!();
}

fn change_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Right => Direction::Down,
        Direction::Left => Direction::Up,
    }
}

fn valid_block(pos: &mut Position, map: &Vec<Vec<char>>, dir: &mut Direction) -> bool {
    let mut next_y = pos.y;
    let mut next_x = pos.x;

    match dir {
        Direction::Up => {
            if next_x == 0 {
                return false;
            }
            next_x -= 1;
        }
        Direction::Down => {
            next_x += 1;
            if next_x == map.len() {
                return false;
            }
        }
        Direction::Right => {
            next_y += 1;
            if next_y == map[0].len() {
                return false;
            }
        }
        Direction::Left => {
            if next_y == 0 {
                return false;
            }
            next_y -= 1;
        }
    }

    if map[next_x][next_y] == '#' {
        *dir = change_direction(*dir);
        return true;
    }

    pos.x = next_x;
    pos.y = next_y;

    return true;
}

fn valid_bar(pos: &mut Position, map: &Vec<Vec<char>>, dir: &mut Direction) -> bool {
    let mut next_y = pos.y;
    let mut next_x = pos.x;

    match dir {
        Direction::Up => {
            if next_x == 0 {
                return false;
            }
            next_x -= 1;
        }
        Direction::Down => {
            next_x += 1;
            if next_x == map.len() {
                return false;
            }
        }
        Direction::Right => {
            next_y += 1;
            if next_y == map[0].len() {
                return false;
            }
        }
        Direction::Left => {
            if next_y == 0 {
                return false;
            }
            next_y -= 1;
        }
    }

    if map[next_x][next_y] == '#' {
        *dir = change_direction(*dir);
        pos.x = next_x;
        pos.y = next_y;
        return true;
    }

    pos.x = next_x;
    pos.y = next_y;

    return true;
}
