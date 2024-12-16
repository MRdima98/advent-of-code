use core::{panic, time};
use std::{collections::HashMap, thread, usize};

// go throuw each eleement.
// Every elem up and down is a HORIZONTAL fence
// Every elem left and right is a VERTICAL fence
// Check the difference in the arrays. Any diff more than 1 is a side

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

        let sides = calc_sides(&shadow_padded_farm, &plot);

        println!(
            "{}: {} * {} = {}",
            farm[plot[0].0][plot[0].1].0,
            sides,
            plot.len(),
            sides * plot.len(),
        );
        sum += sides * plot.len();
    }

    println!("\n Sum part 2: {sum}");
}

fn calc_sides(farm: &[Vec<(String, bool)>], plot: &[(usize, usize)]) -> usize {
    let mut vertical_fence: HashMap<(usize, usize), usize> = HashMap::new();
    let mut horizonta_fence: HashMap<(usize, usize), usize> = HashMap::new();
    let fence = "~".to_string();

    let mut count = 0;
    for plant in plot {
        if farm[plant.0 - 1 + 2][plant.1 + 2].0 == fence {
            horizonta_fence
                .entry((plant.0 - 1 + 2, plant.1 + 2))
                .and_modify(|f| *f += 1)
                .or_insert(1);
        }

        if farm[plant.0 + 1 + 2][plant.1 + 2].0 == fence {
            horizonta_fence
                .entry((plant.0 + 1 + 2, plant.1 + 2))
                .and_modify(|f| *f += 1)
                .or_insert(1);
        }

        if farm[plant.0 + 2][plant.1 - 1 + 2].0 == fence {
            vertical_fence
                .entry((plant.1 - 1 + 2, plant.0 + 2))
                .and_modify(|f| *f += 1)
                .or_insert(1);
        }

        if farm[plant.0 + 2][plant.1 + 1 + 2].0 == fence {
            vertical_fence
                .entry((plant.1 + 1 + 2, plant.0 + 2))
                .and_modify(|f| *f += 1)
                .or_insert(1);
        }
    }

    count += determine_side(&mut vertical_fence.clone(), farm);
    count += determine_side(&mut horizonta_fence.clone(), farm);

    count
}

fn determine_side(
    fence: &mut HashMap<(usize, usize), usize>,
    farm: &[Vec<(String, bool)>],
) -> usize {
    let mut count = 1;
    let mut keys: Vec<(usize, usize)> = fence.keys().cloned().collect();
    keys.sort();
    let first = keys.first().cloned().unwrap();
    let mut prev = first;

    let mut max = 1;
    for key in keys {
        if key == first {
            continue;
        }

        if *fence.get(&key).unwrap() > max {
            max = *fence.get(&key).unwrap();
        }

        print!("{}", farm[key.0][key.1].0);

        if !(key.0 == prev.0 && key.1 - prev.1 == 1) {
            println!();
            count += max;
            max = 1;
        }

        prev = key;
    }

    count
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
