use core::{panic, time};
use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    ops::{Add, AddAssign, Sub, SubAssign},
    thread, usize,
};

pub fn run(path: &str) -> usize {
    let input: String = fs::read_to_string(path).unwrap();
    let mut map = vec![];
    let mut start = Coord(0, 0);
    let mut goal = Coord(0, 0);

    for (i, line) in input.lines().enumerate() {
        let mut tmp = vec![];
        for (j, el) in line.chars().enumerate() {
            tmp.push(el);
            if el == 'S' {
                start = Coord(i, j);
            }

            if el == 'E' {
                goal = Coord(i, j);
            }
        }
        map.push(tmp);
    }

    let path_info = a_star(&map, start, goal, Direction::Right);
    let optimal = path_info[0].1;

    //println!("Optimal: {optimal}");

    let mut path = vec![];

    for node in path_info.iter() {
        path.push(node.0);
    }

    for ele in path_info.iter() {
        let cost_until_now = ele.1;
        let mut one_move_cost = 0;
        let mut partial = vec![];
        let neighbours = get_neighbours(&map, ele.0);
        for neigh in neighbours {
            if path.contains(&neigh) {
                continue;
            }
            let mut dir = ele.2;
            one_move_cost = dist(ele.0, neigh, &mut dir);
            partial = a_star(&map, neigh, goal, dir);

            if neigh.0 == 113 && neigh.1 == 12 {
                println!("Should hit");
                //println!("{:?}", partial);
                println!("{}", cost_until_now + one_move_cost + partial[0].1);
                println!("{}", one_move_cost);
                //wrong_path = partial.clone();
            }
        }

        if partial.is_empty() {
            continue;
        }

        if optimal >= cost_until_now + one_move_cost + partial[0].1 {
            for node in partial.iter() {
                if !path.contains(&node.0) {
                    path.push(node.0);
                }
            }
        }
    }

    for (node, _, _) in path_info.iter() {
        map[node.0][node.1] = 'O';
    }

    //pretty_print(&map);

    println!("Optimal: {optimal}");
    println!("Count of nodes: {}", path.len());
    optimal
}

fn reconstruct_path(came_from: HashMap<Coord, Coord>, current: Coord) -> Vec<Coord> {
    let mut total_path = vec![current];

    let mut current = current;
    while came_from.contains_key(&current) {
        current = *came_from.get(&current).unwrap();
        total_path.push(current);
    }

    total_path
}

fn heuritis(neighbour: Coord, goal: Coord) -> usize {
    if neighbour.0 != goal.0 {
        return 5000;
    }

    ((neighbour.0 as i64 - goal.0 as i64).abs() + (neighbour.1 as i64 - goal.1 as i64).abs())
        as usize
}

fn a_star(
    map: &[Vec<char>],
    start: Coord,
    goal: Coord,
    dir: Direction,
) -> Vec<(Coord, usize, Direction)> {
    let mut open_set = vec![(start, dir)];
    let mut came_from: HashMap<Coord, Coord> = HashMap::new();

    let mut g_score: HashMap<Coord, (usize, Direction)> = HashMap::new();
    g_score.insert(start, (0, dir));

    let mut f_score: HashMap<Coord, usize> = HashMap::new();
    f_score.insert(start, heuritis(start, goal));

    while !open_set.is_empty() {
        let mut min = usize::MAX;
        let mut coord = Coord(0, 0);
        for node in open_set.iter() {
            if let Some(val) = f_score.get(&node.0) {
                if *val < min {
                    coord = node.0;
                    min = *val;
                }
            }
        }

        let mut idx = 0;
        for (i, el) in open_set.iter().enumerate() {
            if coord == el.0 {
                idx = i;
            }
        }

        let current = open_set.remove(idx);

        //println!("current: {}  min heuristi {}", current.0, min);
        //let mut tmp = vec![];
        //for el in map.iter() {
        //    tmp.push(el.clone());
        //}
        //tmp[current.0 .0][current.0 .1] = 'X';
        //pretty_print(&tmp);
        //thread::sleep(time::Duration::from_millis(200));

        if current.0 == goal {
            let tmp = reconstruct_path(came_from.clone(), current.0);
            let mut res = vec![];
            for el in tmp {
                if let Some(cost) = g_score.get(&el) {
                    res.push((el, cost.0, cost.1));
                }
            }
            return res;
        }

        let neighbours = get_neighbours(map, current.0);
        //println!("{:?}", g_score);

        for neigh in neighbours {
            let current_score = g_score.get(&current.0).unwrap();
            let mut current_direction = current.1;
            let tentative_score = current_score.0 + dist(current.0, neigh, &mut current_direction);
            //println!(
            //    "tentative: {tentative_score} node: {}, dir: {:?}",
            //    current.0, current_direction
            //);
            //
            //if let Some(score) = g_score.get(&neigh) {
            //    println!("extra score: {}", score.0);
            //}

            g_score
                .entry(neigh)
                .or_insert((usize::max_value(), current_direction));

            f_score.entry(neigh).or_insert(usize::max_value());

            if tentative_score < g_score.get(&neigh).unwrap().0 {
                //println!("curr score: {}", g_score.get(&neigh).unwrap().0);

                came_from.entry(neigh).or_insert(current.0);

                g_score
                    .entry(neigh)
                    .and_modify(|el| *el = (tentative_score, current_direction));

                f_score
                    .entry(neigh)
                    .and_modify(|el| *el = tentative_score + heuritis(neigh, goal));
                if !open_set.contains(&(neigh, current_direction)) {
                    open_set.push((neigh, current_direction));
                }
            }
            //println!();
        }
    }
    //println!("{}", g_score.get(&goal).unwrap().0);

    vec![]
}

fn dist(current: Coord, neigh: Coord, dir: &mut Direction) -> usize {
    let mut dist = 0;
    let mut actual_dir: Option<Direction> = None;
    let next_move = (
        neigh.0 as i64 - current.0 as i64,
        neigh.1 as i64 - current.1 as i64,
    );

    match next_move {
        (-1, 0) => {
            if matches!(dir, Direction::Up) {
                dist += 1;
            } else {
                actual_dir = Some(Direction::Up);
            }
        }
        (1, 0) => {
            if matches!(dir, Direction::Down) {
                dist += 1;
            } else {
                actual_dir = Some(Direction::Down);
            }
        }
        (0, 1) => {
            if matches!(dir, Direction::Right) {
                dist += 1;
            } else {
                actual_dir = Some(Direction::Right);
            }
        }
        (0, -1) => {
            if matches!(dir, Direction::Left) {
                dist += 1;
            } else {
                actual_dir = Some(Direction::Left);
            }
        }
        (_, _) => {}
    }

    if dist == 1 {
        return 1;
    }

    if let Some(actual_dir) = actual_dir {
        let clockwise = rotate(&mut dir.clone(), actual_dir.clone(), "CLOCKWISE");
        let anti_clockwise = rotate(&mut dir.clone(), actual_dir, "ANTI-CLOCKWISE");
        *dir = actual_dir;
        return clockwise.min(anti_clockwise);
    }

    dist
}

fn rotate(dir: &mut Direction, actual_dir: Direction, which_way: &str) -> usize {
    let mut counter = 0;
    let clockwise = "CLOCKWISE";
    loop {
        if *dir == actual_dir {
            break;
        }

        match dir {
            Direction::Up => {
                counter += 1000;
                if which_way == clockwise {
                    *dir = Direction::Right;
                } else {
                    *dir = Direction::Left;
                }
            }
            Direction::Down => {
                counter += 1000;
                if which_way == clockwise {
                    *dir = Direction::Left;
                } else {
                    *dir = Direction::Right;
                }
            }
            Direction::Left => {
                counter += 1000;
                if which_way == clockwise {
                    *dir = Direction::Up;
                } else {
                    *dir = Direction::Down;
                }
            }
            Direction::Right => {
                counter += 1000;
                if which_way == clockwise {
                    *dir = Direction::Down;
                } else {
                    *dir = Direction::Up;
                }
            }
        }
    }

    counter + 1
}

fn get_neighbours(map: &[Vec<char>], current: Coord) -> Vec<Coord> {
    let mut possible_neighbours = vec![];
    possible_neighbours.push(current - Coord(1, 0));
    possible_neighbours.push(current + Coord(1, 0));
    possible_neighbours.push(current - Coord(0, 1));
    possible_neighbours.push(current + Coord(0, 1));

    let mut neighbours = vec![];
    for neigh in possible_neighbours {
        if map[neigh.0][neigh.1] == '.' || map[neigh.0][neigh.1] == 'E' {
            neighbours.push(neigh);
        }
    }

    neighbours
}

#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq, PartialOrd)]
struct Coord(usize, usize);

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

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn pretty_print(map: &[Vec<char>]) {
    for line in map {
        for el in line {
            print!("{el}");
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reddit2() {
        let res = run("./inputs/reddit2");
        let expected = 4013;
        assert_eq!(res, expected, "\nres: \n{res}\nexpected:\n{expected}");
    }

    #[test]
    fn base1() {
        let res = run("./inputs/base1");
        let expected = 7036;
        assert_eq!(res, expected, "\nres: \n{res}\nexpected:\n{expected}");
    }
}
