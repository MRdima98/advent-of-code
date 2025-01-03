use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    ops::{Add, AddAssign, Sub, SubAssign},
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

    pretty_print(&map);

    let path = A_star(&map, start, goal);
    println!("Path {:?}", path);

    print!("Hello from part1\n");
}

fn reconstruct_path(came_from: HashMap<Coord, Coord>, current: Coord) -> Vec<Coord> {
    let mut total_path = vec![current];
    for (key, _) in came_from {
        total_path.push(key);
    }

    total_path
}

fn heuritis(neighbour: Coord, goal: Coord) -> usize {
    ((neighbour.0 as i64 - goal.0 as i64) + (neighbour.1 as i64 - goal.1 as i64)).abs() as usize
}

fn A_star(map: &[Vec<char>], start: Coord, goal: Coord) -> Vec<Coord> {
    let mut open_set = vec![start];
    let mut came_from: HashMap<Coord, Coord> = HashMap::new();

    let mut g_score: HashMap<Coord, usize> = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score: HashMap<Coord, usize> = HashMap::new();
    f_score.insert(start, heuritis(start, goal));

    let mut dir = Direction::Left;
    while !open_set.is_empty() {
        let current = open_set.remove(0);
        if current == goal {
            return reconstruct_path(came_from, current);
        }

        let neighbours = get_neighbours(map, current);

        for neigh in neighbours {
            let tentative_score = g_score.get(&current).unwrap() + dist(current, neigh, &mut dir);
            g_score.entry(neigh).or_insert(usize::max_value());

            if tentative_score < *g_score.get(&neigh).unwrap() {
                came_from.entry(neigh).and_modify(|f| *f = current);
                g_score.entry(neigh).or_insert(tentative_score);
                f_score
                    .entry(neigh)
                    .or_insert(tentative_score + heuritis(neigh, goal));
                if !open_set.contains(&neigh) {
                    open_set.push(neigh);
                }
            }
        }
    }

    vec![]
}

fn dist(current: Coord, neigh: Coord, dir: &mut Direction) -> usize {
    let mut dist = 0;
    let mut actual_dir: Option<Direction> = None;
    let next_move = (
        current.0 as i64 - neigh.0 as i64,
        current.1 as i64 - neigh.1 as i64,
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
                actual_dir = Some(Direction::Up);
            }
        }
        (0, 1) => {
            if matches!(dir, Direction::Right) {
                dist += 1;
            } else {
                actual_dir = Some(Direction::Up);
            }
        }
        (0, -1) => {
            if matches!(dir, Direction::Left) {
                dist += 1;
            } else {
                actual_dir = Some(Direction::Up);
            }
        }
        (_, _) => {}
    }

    if dist == 1 {
        return 1;
    }

    loop {
        let Some(tmp) = actual_dir else {
            break;
        };

        if *dir == tmp {
            *dir = tmp;
            break;
        }

        match tmp {
            Direction::Up => {
                actual_dir = Some(Direction::Right);
                dist += 1000;
            }
            Direction::Down => {
                actual_dir = Some(Direction::Left);
                dist += 1000;
            }
            Direction::Left => {
                actual_dir = Some(Direction::Up);
                dist += 1000;
            }
            Direction::Right => {
                actual_dir = Some(Direction::Down);
                dist += 1000;
            }
        }
    }

    dist
}

fn get_neighbours(map: &[Vec<char>], current: Coord) -> Vec<Coord> {
    let mut possible_neighbours = vec![];
    possible_neighbours.push(current - Coord(1, 0));
    possible_neighbours.push(current + Coord(1, 0));
    possible_neighbours.push(current - Coord(0, 1));
    possible_neighbours.push(current + Coord(0, 1));

    let mut neighbours = vec![];
    for neigh in possible_neighbours {
        if map[neigh.0][neigh.1] == '.' {
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

#[derive(PartialEq, Eq)]
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
