use core::{panic, prelude, time};
use std::{collections::HashMap, thread, usize, vec};

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

    let mut plots = get_plots(&mut farm.clone());

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

    for (_, plot) in plots.iter_mut() {
        for plant in plot.iter_mut() {
            plant.0 += 2;
            plant.1 += 2;
        }
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

        let mut sides = get_sides(&shadow_padded_farm, &plot);

        let mut inverted_shadow_farm = shadow_padded_farm.clone();
        for (i, row) in shadow_padded_farm.iter().enumerate() {
            for (j, el) in row.iter().enumerate() {
                inverted_shadow_farm[j][i] = el.clone();
            }
        }

        let mut inverted_plot = vec![];
        for el in plot.iter() {
            inverted_plot.push((el.1, el.0));
        }

        sides += get_sides(&inverted_shadow_farm, &inverted_plot);

        sum += sides * plot.len();
    }

    println!("\n Sum part 2: {sum}");
}

fn get_sides(farm: &[Vec<(String, bool)>], plot: &[(usize, usize)]) -> usize {
    let mut upper_fence = vec![];
    let mut lower_fence = vec![];
    let fence = "~".to_string();
    let mut sides = 0;
    for (i, row) in farm.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if !plot.contains(&(i, j)) {
                continue;
            }

            if farm[i - 1][j].0 == fence {
                upper_fence.push((i, j));
            }

            if farm[i + 1][j].0 == fence {
                lower_fence.push((i, j));
            }
        }
        upper_fence.sort();
        lower_fence.sort();
        sides += count_sides(&upper_fence);
        sides += count_sides(&lower_fence);
        upper_fence = vec![];
        lower_fence = vec![];
    }

    sides
}

fn count_sides(arr: &[(usize, usize)]) -> usize {
    if arr.is_empty() {
        return 0;
    }
    let mut count = 1;
    let first = arr.first().unwrap();
    let mut prev = first;
    for el in arr {
        if el == first {
            continue;
        }

        if !(el.0 == prev.0 && el.1 - prev.1 == 1) {
            count += 1;
        }

        prev = el;
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
        neighbours.push((coord.0 - 1, coord.1));
    }

    if !plot.contains(&(coord.0, coord.1 - 1)) {
        neighbours.push((coord.0, coord.1 - 1));
    }

    if !plot.contains(&(coord.0 + 1, coord.1)) {
        neighbours.push((coord.0 + 1, coord.1));
    }

    if !plot.contains(&(coord.0, coord.1 + 1)) {
        neighbours.push((coord.0, coord.1 + 1));
    }

    if !plot.contains(&(coord.0 + 1, coord.1 + 1)) {
        neighbours.push((coord.0 + 1, coord.1 + 1));
    }

    if !plot.contains(&(coord.0 - 1, coord.1 + 1)) {
        neighbours.push((coord.0 - 1, coord.1 + 1));
    }

    if !plot.contains(&(coord.0 - 1, coord.1 - 1)) {
        neighbours.push((coord.0 - 1, coord.1 - 1));
    }

    if !plot.contains(&(coord.0 + 1, coord.1 - 1)) {
        neighbours.push((coord.0 + 1, coord.1 - 1));
    }

    neighbours
}
