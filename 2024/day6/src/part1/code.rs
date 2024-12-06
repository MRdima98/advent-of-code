use core::time;
use std::{thread, usize};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

enum Block {
    Valid,
    Obstacle,
    Exit,
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

    print!("Starte move\n");
    let mut direction = Direction::Up;
    let mut block = Block::Valid;
    let mut distinct_pos: Vec<Position> = vec![];
    loop {
        match block {
            Block::Exit => break,
            Block::Obstacle => {
                direction = check_direction(direction, &map, &guard_coord);
            }
            _ => {}
        }

        if !distinct_pos.contains(&guard_coord) {
            distinct_pos.push(guard_coord.clone());
        }

        block = move_guard(&mut guard_coord, &direction, &map);
        //thread::sleep(time::Duration::from_millis(200));
    }

    print!("\nNum of steps: {}\n\n", distinct_pos.iter().count());
}

fn move_guard(position: &mut Position, direction: &Direction, map: &Vec<Vec<char>>) -> Block {
    match direction {
        Direction::Up => {
            let block = check_block(*position, map, -1, 0);
            match block {
                Block::Valid => {
                    position.x -= 1;
                    Block::Valid
                }
                Block::Obstacle => Block::Obstacle,
                _ => Block::Exit,
            }
        }
        Direction::Down => {
            let block = check_block(*position, map, 1, 0);
            match block {
                Block::Valid => {
                    position.x += 1;
                    Block::Valid
                }
                Block::Obstacle => Block::Obstacle,
                _ => Block::Exit,
            }
        }
        Direction::Right => {
            let block = check_block(*position, map, 0, 1);
            match block {
                Block::Valid => {
                    position.y += 1;
                    Block::Valid
                }
                Block::Obstacle => Block::Obstacle,
                _ => Block::Exit,
            }
        }
        Direction::Left => {
            let block = check_block(*position, map, 0, -1);
            match block {
                Block::Valid => {
                    position.y -= 1;
                    Block::Valid
                }
                Block::Obstacle => Block::Obstacle,
                _ => Block::Exit,
            }
        }
    }
}

fn check_direction(direction: Direction, map: &Vec<Vec<char>>, position: &Position) -> Direction {
    if map[position.x][position.y] == '^' {
        return direction;
    }

    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Right => Direction::Down,
        Direction::Left => Direction::Up,
    }
}

fn check_block(pos: Position, map: &Vec<Vec<char>>, xmove: i32, ymove: i32) -> Block {
    if xmove < 0 && pos.x == 0 {
        return Block::Exit;
    }

    if ymove < 0 && pos.y == 0 {
        return Block::Exit;
    }

    let next_x = (pos.x as i32 + xmove) as usize;
    if (pos.x as i32 + xmove) as usize == map.len() {
        return Block::Exit;
    }

    let next_y = (pos.y as i32 + ymove) as usize;
    if (pos.y as i32 + ymove) as usize == map[0].len() {
        return Block::Exit;
    }

    if map[next_x][next_y] == '#' {
        return Block::Obstacle;
    }

    return Block::Valid;
}
