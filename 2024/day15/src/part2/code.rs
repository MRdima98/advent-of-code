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

#[derive(Clone, Copy)]
struct FishBox {
    coord: Coord,
    sym: char,
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

    let mut wide_map = vec![];
    for row in map.iter() {
        let mut tmp = vec![];
        for c in row.iter() {
            if *c == '@' {
                tmp.push('@');
                tmp.push('.');
            } else if *c == 'O' {
                tmp.push('[');
                tmp.push(']');
            } else {
                tmp.push(*c);
                tmp.push(*c);
            }
        }
        wide_map.push(tmp);
    }

    for (i, row) in wide_map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '@' {
                robot_pos = Coord(i as i64, j as i64);
            }
        }
    }

    pretty_print(&wide_map);

    move_robot(&mut wide_map, &moves, &mut robot_pos);

    let mut sum = 0;
    for (i, row) in wide_map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '[' {
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
        pretty_print(map);
        //thread::sleep(time::Duration::from_millis(250));
        //thread::sleep(time::Duration::from_millis(500));
        //print!(" {}[2J", 27 as char);
    }
}

fn valid_move(map: &mut [Vec<char>], robot_pos: &mut Coord, dir: Direction) {
    //println!("Robot before: {}", robot_pos);
    let mut next_move: Coord;
    match dir {
        Direction::Up => next_move = Coord(-1, 0),
        Direction::Down => next_move = Coord(1, 0),
        Direction::Left => next_move = Coord(0, -1),
        Direction::Right => next_move = Coord(0, 1),
    }

    let mut potential_move = *robot_pos + next_move;
    let mut tile = map[potential_move.0 as usize][potential_move.1 as usize];

    if !(tile == '#' || tile == '[' || tile == ']') {
        *robot_pos = potential_move;
    }

    let mut boxes = vec![];
    if tile == '[' || tile == ']' {
        boxes = bfs(map, potential_move);

        if next_move.0 == 0 {
            let mut clean_box = vec![];
            for i in 0..boxes.len() {
                if boxes[i].coord.0 == robot_pos.0 {
                    clean_box.push(boxes[i]);
                } else {
                    map[boxes[i].coord.0 as usize][boxes[i].coord.1 as usize] = boxes[i].sym;
                }
            }

            boxes = clean_box;
        } else if next_move.0 == 1 {
            println!("Moving down");
            let mut clean_box = vec![];
            for i in 0..boxes.len() {
                if boxes[i].coord.0 > robot_pos.0 {
                    clean_box.push(boxes[i]);
                } else {
                    map[boxes[i].coord.0 as usize][boxes[i].coord.1 as usize] = boxes[i].sym;
                }
            }

            boxes = clean_box;
        } else if next_move.0 == -1 {
            println!("Moving up");
            println!("Robo pos: {robot_pos}");
            let mut clean_box = vec![];
            for i in 0..boxes.len() {
                if boxes[i].coord.0 < robot_pos.0 {
                    clean_box.push(boxes[i]);
                } else {
                    map[boxes[i].coord.0 as usize][boxes[i].coord.1 as usize] = boxes[i].sym;
                }
            }

            boxes = clean_box;
        }

        for ele in boxes.iter() {
            print!("{} ", ele.coord);
        }
        println!();

        let valid = check_walls(map, boxes.clone(), next_move);

        for el in boxes.iter() {
            if !valid {
                next_move = Coord(0, 0);
            }
            let tmp = el.coord + next_move;
            map[tmp.0 as usize][tmp.1 as usize] = el.sym;
        }

        if !valid {
            boxes = vec![];
        }

        if !boxes.is_empty() {
            println!("Potential move: {potential_move}");
            *robot_pos = potential_move;
        }
    }
    //println!("Robot after: {}", robot_pos);
}

fn check_walls(map: &mut [Vec<char>], boxes: Vec<FishBox>, dir: Coord) -> bool {
    match dir {
        Coord(0, 1) => {
            for el in boxes {
                if map[el.coord.0 as usize][(el.coord.1 + 1) as usize] == '#' {
                    return false;
                }
            }
            true
        }
        Coord(0, -1) => {
            for el in boxes {
                if map[el.coord.0 as usize][(el.coord.1 - 1) as usize] == '#' {
                    return false;
                }
            }
            true
        }
        Coord(1, 0) => {
            for el in boxes {
                if map[(el.coord.0 + 1) as usize][el.coord.1 as usize] == '#' {
                    return false;
                }
            }
            true
        }
        _ => {
            for el in boxes {
                if map[(el.coord.0 - 1) as usize][el.coord.1 as usize] == '#' {
                    return false;
                }
            }
            true
        }
    }
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

// TODO
// You need a specific kind of bfs, two actually
// one that check ONLY above head
// one that check only rows
fn bfs(map: &mut [Vec<char>], start: Coord) -> Vec<FishBox> {
    let mut queue = vec![];
    let mut fishBox = vec![];

    queue.push(start);
    fishBox.push(FishBox {
        coord: start,
        sym: map[start.0 as usize][start.1 as usize],
    });
    map[start.0 as usize][start.1 as usize] = '.';

    while !queue.is_empty() {
        let curr = queue.pop().unwrap();
        let neighbours = get_neighbour(map, curr);

        for neigh in neighbours.iter() {
            if !(map[neigh.0 as usize][neigh.1 as usize] == '.') {
                queue.push(*neigh);
                fishBox.push(FishBox {
                    coord: *neigh,
                    sym: map[neigh.0 as usize][neigh.1 as usize],
                });
                map[neigh.0 as usize][neigh.1 as usize] = '.';
            }
        }
    }

    fishBox
}

fn get_neighbour(map: &mut [Vec<char>], node: Coord) -> Vec<Coord> {
    let mut pot_neighbours = vec![];
    pot_neighbours.push(node - Coord(-1, 0));
    pot_neighbours.push(node - Coord(-1, -1));
    pot_neighbours.push(node - Coord(-1, 1));
    pot_neighbours.push(node - Coord(1, 0));
    pot_neighbours.push(node - Coord(1, -1));
    pot_neighbours.push(node - Coord(1, 1));
    pot_neighbours.push(node - Coord(0, -1));
    pot_neighbours.push(node - Coord(0, 1));
    let mut neighbours = vec![];

    for neigh in pot_neighbours {
        let tmp = map[neigh.0 as usize][neigh.1 as usize];
        if tmp == '[' || tmp == ']' {
            neighbours.push(neigh)
        }
    }

    neighbours
}

//boxes.push(FishBox {
//    coord: potential_move,
//    sym: map[potential_move.0 as usize][potential_move.1 as usize],
//});
//boxes.push(FishBox {
//    coord: potential_move + next_move,
//    sym: map[(potential_move.0 + next_move.0) as usize]
//        [(potential_move.1 + next_move.1) as usize],
//});
//loop {
//    potential_move = potential_move + next_move + next_move;
//    tile = map[potential_move.0 as usize][potential_move.1 as usize];
//
//    if tile == '[' {
//        boxes.push(FishBox {
//            coord: potential_move,
//            sym: map[potential_move.0 as usize][potential_move.1 as usize],
//        });
//        boxes.push(FishBox {
//            coord: potential_move + next_move,
//            sym: map[(potential_move.0 + next_move.0) as usize]
//                [(potential_move.1 + next_move.1) as usize],
//        });
//    }
//
//    if tile == ']' {
//        boxes.push(FishBox {
//            coord: potential_move,
//            sym: map[potential_move.0 as usize][potential_move.1 as usize],
//        });
//        boxes.push(FishBox {
//            coord: potential_move + next_move,
//            sym: map[(potential_move.0 + next_move.0) as usize]
//                [(potential_move.1 + next_move.1) as usize],
//        });
//    }
//
//    if tile == '#' {
//        boxes = vec![];
//        break;
//    }
//
//    if tile == '.' {
//        break;
//    }
//}
