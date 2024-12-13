use std::usize;

pub fn run() {
    let input = include_str!("../input");
    let mut farm = vec![];
    let mut plots = vec![];

    for line in input.lines() {
        let mut tmp = vec![];
        for c in line.chars() {
            tmp.push((c, false));
        }
        farm.push(tmp);
    }

    plots = get_plots(&mut farm.clone());

    let mut sum = 0;
    for plot in plots.iter() {
        //println!(
        //    "Size: {}{}, plot: {:?}",
        //    plot.len(),
        //    farm[plot[0].0][plot[0].1].0,
        //    plot
        //);
        let mut perimeter = 0;
        for plant in plot {
            perimeter += get_not_neighbours(&mut farm, *plant).len();
        }
        println!(
            "{}: {} * {perimeter}",
            farm[plot[0].0][plot[0].1].0,
            plot.len()
        );
        sum += perimeter * plot.len();
    }

    println!("\n My sum is: {sum}");
}

fn get_plots(farm: &mut Vec<Vec<(char, bool)>>) -> Vec<Vec<(usize, usize)>> {
    let mut farm = farm;
    let mut plots = vec![];

    for i in 0..farm.len() {
        let mut plot: Vec<(usize, usize)> = vec![];
        for j in 0..farm[0].len() {
            if farm[i][j].1 {
                println!("{}", farm[i][j].0);
                continue;
            }

            let mut queue = vec![];
            if !plot.is_empty() {
                let prev = plot.first().unwrap();
                if farm[prev.0][prev.1].0 != farm[i][j].0 {
                    continue;
                }
            }
            queue.push((i, j));
            plot.push((i, j));
            farm[i][j].1 = true;

            while !queue.is_empty() {
                let node = queue.pop().unwrap();
                let neighbours = get_neighbours(&mut farm, node);

                for ghebur in neighbours {
                    if !farm[ghebur.0][ghebur.1].1 {
                        farm[ghebur.0][ghebur.1].1 = true;
                        plot.push(ghebur);
                        queue.push(ghebur);
                    }
                }
            }
        }

        if !plot.is_empty() {
            plots.push(plot);
        }
    }

    plots
}

fn get_neighbours(farm: &mut [Vec<(char, bool)>], coord: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];

    if coord.0 > 0 {
        if farm[coord.0][coord.1].0 == farm[coord.0 - 1][coord.1].0 {
            neighbours.push((coord.0 - 1, coord.1));
        }
    } else {
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
    farm: &mut [Vec<(char, bool)>],
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
