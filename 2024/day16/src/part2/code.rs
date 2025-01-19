use core::{panic, time};
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    fs,
    ops::{Add, AddAssign, Sub, SubAssign},
    thread::{self, current},
    usize,
};

pub fn run(path: &str) {
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

    let mut path_with_cost = a_star(&map, start, goal, Direction::Right);
    let mut seats = vec![];
    let optimal = path_with_cost.first().unwrap().cost;

    println!("Optimal: {optimal}");
    for el in path_with_cost.iter() {
        seats.push(el.coord);
    }

    while let Some(node) = path_with_cost.pop() {
        let neighbours = get_neighbours(&map, node.coord);

        for neigh in neighbours.iter() {
            if seats.contains(&neigh) {
                continue;
            }

            let mut dir = node.dir;
            let one_move_cost = dist(node.coord, *neigh, &mut dir);
            let partial = a_star(&map, *neigh, goal, dir);

            if partial.is_empty() {
                continue;
            }

            if partial.first().unwrap().cost + one_move_cost + node.cost == optimal {
                path_with_cost.push(CoordWithCost {
                    coord: *neigh,
                    cost: node.cost + one_move_cost,
                    dir,
                });
                seats.push(*neigh);

                for el in partial.iter() {
                    if !seats.contains(&el.coord) {
                        seats.push(el.coord);
                        path_with_cost.push(CoordWithCost {
                            coord: el.coord,
                            cost: node.cost + one_move_cost + el.cost,
                            dir: el.dir,
                        });
                    }
                }
            }
        }
    }

    for el in seats.iter() {
        map[el.0][el.1] = 'O';
    }

    println!("Seats: {}", seats.len());
}

fn reconstruct_path2(
    came_from: HashMap<(Coord, Direction), Option<(Coord, Direction)>>,
    current: (Coord, Direction),
    g_score: HashMap<(Coord, Direction), usize>,
) -> Vec<CoordWithCost> {
    let mut total_path = vec![CoordWithCost {
        coord: current.0,
        dir: current.1,
        cost: *g_score.get(&current).unwrap(),
    }];

    let mut current = current;
    while came_from.contains_key(&current) {
        match *came_from.get(&current).unwrap() {
            Some(curr) => {
                current = curr;
                total_path.push(CoordWithCost {
                    coord: current.0,
                    dir: current.1,
                    cost: *g_score.get(&current).unwrap(),
                });
            }
            None => {
                break;
            }
        }
    }

    total_path
}

fn heuritis(neighbour: Coord, goal: Coord) -> usize {
    (((neighbour.0 as i64 - goal.0 as i64).pow(2) + (neighbour.1 as i64 - goal.1 as i64).pow(2))
        .abs() as f64)
        .sqrt() as usize
}

fn a_star(map: &[Vec<char>], start: Coord, goal: Coord, dir: Direction) -> Vec<CoordWithCost> {
    let mut open_set = BinaryHeap::new();
    open_set.push(CoordDir { coord: start, dir });
    let mut came_from: HashMap<Coord, Coord> = HashMap::new();

    let mut g_score: HashMap<(Coord, Direction), usize> = HashMap::new();
    g_score.insert((start, dir), 0);

    let mut prev: HashMap<(Coord, Direction), Option<(Coord, Direction)>> = HashMap::new();
    prev.insert((start, dir), None);

    let mut f_score: HashMap<(Coord, Direction), usize> = HashMap::new();
    f_score.insert((start, dir), heuritis(start, goal));

    while let Some(tmp) = open_set.pop() {
        let current = tmp.coord;
        let dir = tmp.dir;
        let neighbours = get_neighbours(map, current);

        for neigh in neighbours {
            let current_score = g_score.get(&(current, dir)).unwrap();
            let prev_dir = dir;
            let mut current_direction = dir;
            let tentative_score = current_score + dist(current, neigh, &mut current_direction);

            g_score
                .entry((neigh, current_direction))
                .or_insert(usize::max_value());

            f_score
                .entry((neigh, current_direction))
                .or_insert(usize::max_value());

            if tentative_score < *g_score.get(&(neigh, current_direction)).unwrap() {
                came_from.entry(neigh).or_insert(current);

                g_score
                    .entry((neigh, current_direction))
                    .and_modify(|el| *el = tentative_score);

                prev.entry((neigh, current_direction))
                    .and_modify(|el| *el = Some((current, prev_dir)))
                    .or_insert(Some((current, prev_dir)));

                f_score
                    .entry((neigh, current_direction))
                    .and_modify(|el| *el = tentative_score + heuritis(neigh, goal));

                open_set.push(CoordDir {
                    coord: neigh,
                    dir: current_direction,
                });
            }
        }
    }

    let mut best = (start, dir);
    let mut min = usize::max_value();
    for el in g_score.keys() {
        if el.0 == goal && *g_score.get(&el).unwrap() <= min {
            best = *el;
            min = *g_score.get(&el).unwrap();
        }
    }

    //println!("Res: {min}");

    reconstruct_path2(prev, best, g_score)
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

#[derive(Clone, Ord, Debug, Copy, Eq, Hash, PartialEq, PartialOrd)]
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

#[derive(PartialEq, Ord, PartialOrd, Hash, Eq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialOrd, Ord, Eq, PartialEq)]
struct CoordDir {
    coord: Coord,
    dir: Direction,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct CoordWithCost {
    coord: Coord,
    cost: usize,
    dir: Direction,
}

impl Display for CoordWithCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {:?}", self.coord, self.cost, self.dir)
    }
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
