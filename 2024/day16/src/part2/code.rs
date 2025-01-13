use core::{panic, time};
use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    hash::Hash,
    ops::{Add, AddAssign, Sub, SubAssign},
    thread, usize,
};

pub fn run(path: &str) -> usize {
    let input: String = fs::read_to_string(path).unwrap();
    let mut map = vec![];
    let mut source = Coord(0, 0);
    let mut goal = Coord(0, 0);

    for (i, line) in input.lines().enumerate() {
        let mut tmp = vec![];
        for (j, el) in line.chars().enumerate() {
            tmp.push(el);
            if el == 'S' {
                source = Coord(i, j);
            }

            if el == 'E' {
                goal = Coord(i, j);
            }
        }
        map.push(tmp);
    }

    let (dist, prev) = dijkstra(&map, source, goal, Direction::Right);
    let (optimal, _) = dist.get(&goal).unwrap().clone();
    let mut path_info = reconstruct_path(dist, prev, &mut goal.clone());

    let tmp = path_info.clone();
    for (i, el) in tmp.iter().rev().enumerate() {
        path_info[i] = *el;
    }
    let mut path = vec![];

    for node in path_info.iter() {
        path.push(node.0);
    }

    for el in path_info.iter() {
        println!("el: {}", el.0);
        let cost_until_now = el.1;
        let mut one_move_cost = 0;
        let mut partial = vec![];
        let neighbours = get_neighbours(&map, el.0);
        for neigh in neighbours {
            if path.contains(&neigh) {
                continue;
            }
            let mut dir = el.2;
            one_move_cost = get_dist(el.0, neigh, &mut dir);
            let (dist, prev) = dijkstra(&map, neigh, goal, dir);
            partial = reconstruct_path(dist, prev, &mut goal.clone());
        }

        if partial.is_empty() {
            continue;
        }

        if optimal == cost_until_now + one_move_cost + partial[0].1 {
            for node in partial.iter() {
                if !path.contains(&node.0) {
                    path.push(node.0);
                }
            }
        }
    }

    for node in path.iter() {
        map[node.0][node.1] = 'O';
    }

    pretty_print(&map);

    println!("Count of nodes: {}", path.len());
    println!("{}", optimal);
    optimal
}

fn reconstruct_path(
    dist: HashMap<Coord, (usize, Direction)>,
    prev: HashMap<Coord, Option<(Coord, Direction)>>,
    goal: &mut Coord,
) -> Vec<(Coord, usize, Direction)> {
    let mut path = vec![];
    loop {
        //println!("{}", *goal + Coord(1, 1));
        //thread::sleep(time::Duration::from_millis(200));
        let Some(node) = prev.get(&goal) else {
            break;
        };

        let cost = dist.get(&goal).unwrap();

        let Some(node) = node else {
            path.push((*goal, 0, cost.1));
            break;
        };

        path.push((*goal, cost.0, cost.1));

        *goal = node.0;
    }
    path
}

fn dijkstra(
    map: &[Vec<char>],
    source: Coord,
    goal: Coord,
    dir: Direction,
) -> (
    HashMap<Coord, (usize, Direction)>,
    HashMap<Coord, Option<(Coord, Direction)>>,
) {
    let mut dist: HashMap<Coord, (usize, Direction)> = HashMap::new();
    let mut prev: HashMap<Coord, Option<(Coord, Direction)>> = HashMap::new();
    let mut queue = vec![];

    for (i, line) in map.iter().enumerate() {
        for (j, el) in line.iter().enumerate() {
            if *el == '.' || *el == 'E' {
                let node = Coord(i, j);
                dist.entry(node).or_insert((usize::max_value(), dir));
                prev.entry(node).or_insert(None);
                queue.push(node);
            }
        }
    }
    dist.entry(source)
        .and_modify(|val| *val = (0, dir))
        .or_insert((0, dir));
    prev.entry(source)
        .and_modify(|prev| *prev = None)
        .or_insert(None);
    queue.push(source);

    while !queue.is_empty() {
        let mut min_coord: Option<Coord> = None;
        let mut min_val = usize::max_value();
        for el in queue.iter() {
            if let Some(node) = dist.get(&el) {
                if node.0 <= min_val {
                    min_val = node.0;
                    min_coord = Some(*el);
                }
            }
        }

        let mut current = source;
        if let Some(coord) = min_coord {
            let mut idx = 0;
            for (i, el) in queue.iter().enumerate() {
                if *el == coord {
                    idx = i;
                }
            }
            current = queue.remove(idx);
        }

        let neighbours = get_neighbours(map, current);
        //let mut pretty = map.to_vec();
        //pretty[current.0][current.1] = 'O';
        //pretty_print(&pretty);
        //thread::sleep(time::Duration::from_millis(50));

        for edge in neighbours {
            if !queue.contains(&edge) {
                continue;
            }
            //pretty[edge.0][edge.1] = 'O';
            //pretty_print(&pretty);
            //thread::sleep(time::Duration::from_millis(50));
            let prev_dir = dist.get(&current).unwrap().1;
            let (cost, mut dir) = dist.get(&current).unwrap().clone();
            //println!("Cost: {cost}");

            let alt = cost + get_dist(current, edge, &mut dir);
            if alt < dist.get(&edge).unwrap().0 {
                dist.entry(edge).and_modify(|f| *f = (alt, dir));
                prev.entry(edge)
                    .and_modify(|f| *f = Some((current, prev_dir)));
            }
        }
    }

    //println!("{:?}", dist.get(&goal).unwrap());
    //println!("{:?}", dist);

    (dist, prev)
}

fn get_dist(current: Coord, neigh: Coord, dir: &mut Direction) -> usize {
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

    #[test]
    fn base2() {
        let res = run("./inputs/base2");
        let expected = 11048;
        assert_eq!(res, expected, "\nres: \n{res}\nexpected:\n{expected}");
    }

    //#[test]
    //fn input() {
    //    let res = run("./inputs/input");
    //    let expected = 130536;
    //    assert_eq!(res, expected, "\nres: \n{res}\nexpected:\n{expected}");
    //}
}
