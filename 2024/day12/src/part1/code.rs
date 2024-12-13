use std::{collections::HashMap, usize};

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

    let mut sum = 0;
    for plot in plots {
        println!(
            "Size: {} {}",
            farm[plot.1[0].0][plot.1[0].1].0,
            plot.1.len(),
        );
        //println!(
        //    "Size: {}{}, plot: {:?}",
        //    plot.1.len(),
        //    farm[plot.1[0].0][plot.1[0].1].0,
        //    plot
        //);
        let mut perimeter = 0;
        for plant in plot.1.iter() {
            perimeter += get_not_neighbours(&mut farm, *plant).len();
        }
        sum += perimeter * plot.1.len();
    }

    println!("\n My sum is: {sum}");
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

fn get_not_neighbours(
    farm: &mut [Vec<(String, bool)>],
    coord: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];

    if coord.0 > 0 {
        if farm[coord.0][coord.1].0 != farm[coord.0 - 1][coord.1].0 {
            neighbours.push((coord.0 - 1, coord.1));
        }
    } else {
        neighbours.push((coord.0 - 1, coord.1));
    }

    if coord.1 > 0 {
        if farm[coord.0][coord.1].0 != farm[coord.0][coord.1 - 1].0 {
            neighbours.push((coord.0, coord.1 - 1));
        }
    } else {
        neighbours.push((coord.0 - 1, coord.1));
    }

    if coord.0 < farm.len() - 1 {
        if farm[coord.0][coord.1].0 != farm[coord.0 + 1][coord.1].0 {
            neighbours.push((coord.0 + 1, coord.1));
        }
    } else {
        neighbours.push((coord.0 - 1, coord.1));
    }

    if coord.1 < farm[0].len() - 1 {
        if farm[coord.0][coord.1].0 != farm[coord.0][coord.1 + 1].0 {
            neighbours.push((coord.0, coord.1 + 1));
        }
    } else {
        neighbours.push((coord.0 - 1, coord.1));
    }

    neighbours
}
