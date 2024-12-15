use core::{panic, time};
use std::{collections::HashMap, thread, usize};

pub fn run() {
    let input = include_str!("../input");
    let mut farm = vec![];

    for line in input.lines() {
        let mut tmp = vec![];
        for c in line.chars() {
            tmp.push((c.to_string(), false));
        }
        farm.push(tmp);
    }

    let plots = get_plots(&mut farm.clone());

    let mut padded_farm = farm.clone();
    padded_farm.push(vec![("*".to_string(), false); padded_farm[0].len()]);
    padded_farm.push(vec![("*".to_string(), false); padded_farm[0].len()]);
    padded_farm.insert(0, vec![("*".to_string(), false); padded_farm[0].len()]);
    padded_farm.insert(0, vec![("*".to_string(), false); padded_farm[0].len()]);

    for row in padded_farm.iter_mut() {
        row.push(("*".to_string(), true));
        row.push(("*".to_string(), true));
        row.insert(0, ("*".to_string(), true));
        row.insert(0, ("*".to_string(), true));
    }

    let mut sum = 0;
    pretty_print(&padded_farm);
    for (_, plot) in plots {
        let mut shadow_padded_farm = padded_farm.clone();
        let mut fence = vec![];
        for coord in plot.iter() {
            let non_neighbour = get_not_neighbours_improved(&plot, (coord.0, coord.1));
            for f in non_neighbour.iter() {
                shadow_padded_farm[f.0][f.1] = ("~".to_string(), false);
            }
            fence.extend(non_neighbour);
        }

        let sides = calc_sides(
            &mut shadow_padded_farm,
            &fence,
            farm[plot[0].0][plot[0].1].0.to_string(),
        );

        //println!();
        pretty_print(&shadow_padded_farm);
        println!(
            "{}: {} * {} = {}\n",
            farm[plot[0].0][plot[0].1].0,
            plot.len(),
            sides,
            sides * plot.len(),
        );
        sum += sides * plot.len();
    }

    println!("\n Sum part 2: {sum}");
}

fn calc_sides(
    farm: &mut Vec<Vec<(String, bool)>>,
    coords: &[(usize, usize)],
    letter: String,
) -> usize {
    let mut count = 0;
    let mut already_done: Vec<(usize, usize)> = vec![];

    for coord in coords.iter() {
        let fence = "~".to_string();
        if already_done.contains(coord) {
            continue;
        } else {
            already_done.push(*coord);
        }

        let up = farm[coord.0 - 1][coord.1].0.clone();
        let left = farm[coord.0][coord.1 - 1].0.clone();
        let right = farm[coord.0][coord.1 + 1].0.clone();
        let down = farm[coord.0 + 1][coord.1].0.clone();

        count += edge_cases(farm, *coord, letter.clone());
        count += small_pot_edge_case(farm, *coord, fence.clone());

        count += base_corner(&fence, &up, &left);
        count += base_corner(&fence, &down, &left);
        count += base_corner(&fence, &up, &right);
        count += base_corner(&fence, &down, &right);
    }

    count
}

fn base_corner(fence: &String, adj1: &String, adj2: &String) -> usize {
    if fence == adj1 && fence == adj2 {
        println!("Base");
        return 1;
    }
    return 0;
}

fn get_plots(farm: &mut Vec<Vec<(String, bool)>>) -> HashMap<String, Vec<(usize, usize)>> {
    let mut farm = farm;
    let mut plots: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..farm.len() {
        let mut unique = "".to_string();
        for j in 0..farm[0].len() {
            if farm[i][j].1 {
                continue;
            }

            let mut queue = vec![];
            queue.push((i, j));
            unique = unique + &i.to_string() + &j.to_string();
            plots
                .entry(farm[i][j].0.clone() + &unique)
                .and_modify(|plot| plot.push((i, j)))
                .or_insert(vec![(i, j)]);
            farm[i][j].1 = true;

            while !queue.is_empty() {
                let node = queue.pop().unwrap();
                let neighbours = get_neighbours(&mut farm, node);

                for ghebur in neighbours {
                    if !farm[ghebur.0][ghebur.1].1 {
                        farm[ghebur.0][ghebur.1].1 = true;
                        plots
                            .entry(farm[ghebur.0][ghebur.1].0.clone() + &unique)
                            .and_modify(|plot| plot.push(ghebur))
                            .or_insert(vec![ghebur]);
                        queue.push(ghebur);
                    }
                }
            }
        }
    }

    plots
}

fn get_neighbours(farm: &mut [Vec<(String, bool)>], coord: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];

    if coord.0 > 0 {
        if farm[coord.0][coord.1].0 == farm[coord.0 - 1][coord.1].0 {
            neighbours.push((coord.0 - 1, coord.1));
        }
    }

    if coord.1 > 0 {
        if farm[coord.0][coord.1].0 == farm[coord.0][coord.1 - 1].0 {
            neighbours.push((coord.0, coord.1 - 1));
        }
    }

    if coord.0 < farm.len() - 1 {
        if farm[coord.0][coord.1].0 == farm[coord.0 + 1][coord.1].0 {
            neighbours.push((coord.0 + 1, coord.1));
        }
    }

    if coord.1 < farm[0].len() - 1 {
        if farm[coord.0][coord.1].0 == farm[coord.0][coord.1 + 1].0 {
            neighbours.push((coord.0, coord.1 + 1));
        }
    }

    neighbours
}

fn get_not_neighbours_improved(
    plot: &[(usize, usize)],
    coord: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];

    if !plot.contains(&(coord.0 - 1, coord.1)) {
        neighbours.push((coord.0 + 2 - 1, coord.1 + 2));
    }

    if !plot.contains(&(coord.0, coord.1 - 1)) {
        neighbours.push((coord.0 + 2, coord.1 - 1 + 2));
    }

    if !plot.contains(&(coord.0 + 1, coord.1)) {
        neighbours.push((coord.0 + 2 + 1, coord.1 + 2));
    }

    if !plot.contains(&(coord.0, coord.1 + 1)) {
        neighbours.push((coord.0 + 2, coord.1 + 1 + 2));
    }

    if !plot.contains(&(coord.0 + 1, coord.1 + 1)) {
        neighbours.push((coord.0 + 2 + 1, coord.1 + 1 + 2));
    }

    if !plot.contains(&(coord.0 - 1, coord.1 + 1)) {
        neighbours.push((coord.0 + 2 - 1, coord.1 + 1 + 2));
    }

    if !plot.contains(&(coord.0 - 1, coord.1 - 1)) {
        neighbours.push((coord.0 + 2 - 1, coord.1 - 1 + 2));
    }

    if !plot.contains(&(coord.0 + 1, coord.1 - 1)) {
        neighbours.push((coord.0 + 2 + 1, coord.1 - 1 + 2));
    }

    neighbours
}

fn pretty_print(farm: &[Vec<(String, bool)>]) {
    for row in farm {
        for el in row {
            print!("{}", el.0);
        }
        println!();
    }
}

fn edge_cases(farm: &mut Vec<Vec<(String, bool)>>, coord: (usize, usize), letter: String) -> usize {
    let mut corner_case = vec![];
    corner_case.push(farm[coord.0 - 1][coord.1].0.clone());
    corner_case.push(farm[coord.0][coord.1 - 1].0.clone());
    corner_case.push(farm[coord.0][coord.1 + 1].0.clone());
    corner_case.push(farm[coord.0 + 1][coord.1].0.clone());

    let mut count = 0;
    for case in corner_case {
        if letter == case {
            count += 1;
        }
    }

    if count > 2 {
        println!("Egde 1: {:?}", coord);
        let mut corner_case = vec![];
        corner_case.push(farm[coord.0 + 1][coord.1 + 1].0.clone());
        corner_case.push(farm[coord.0 - 1][coord.1 - 1].0.clone());
        corner_case.push(farm[coord.0 - 1][coord.1 + 1].0.clone());
        corner_case.push(farm[coord.0 + 1][coord.1 - 1].0.clone());

        let mut count = 0;
        for case in corner_case {
            if letter == case {
                count += 1;
            }
        }

        if count >= 2 {
            println!("Egde 2: {:?}", coord);
            return 2;
        }
        1
    } else {
        0
    }
}

fn small_pot_edge_case(
    farm: &mut Vec<Vec<(String, bool)>>,
    coord: (usize, usize),
    letter: String,
) -> usize {
    let mut corner_case = vec![];
    corner_case.push(farm[coord.0 - 1][coord.1].0.clone());
    corner_case.push(farm[coord.0][coord.1 - 1].0.clone());
    corner_case.push(farm[coord.0][coord.1 + 1].0.clone());
    corner_case.push(farm[coord.0 + 1][coord.1].0.clone());

    for case in corner_case {
        if letter == case {
            return 0;
        }
    }
    println!("Small pot: {:?}", coord);
    2
}
