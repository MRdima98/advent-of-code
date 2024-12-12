use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../input");
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut width: i32 = 0;
    let heigth: i32 = input.lines().count() as i32;

    for (i, line) in input.lines().enumerate() {
        if width == 0 {
            width = line.len() as i32;
        }
        for (j, c) in line.chars().enumerate() {
            let i = i as i32;
            let j = j as i32;
            if c != '.' {
                antennas
                    .entry(c)
                    .and_modify(|coords| coords.push((i, j)))
                    .or_insert(vec![(i, j)]);
            }
        }
    }

    pretty_print(antennas.clone());

    let mut unique_antennas: Vec<(i32, i32)> = vec![];

    for antenna in antennas.iter_mut() {
        for ant1 in antenna.1.iter() {
            for ant2 in antenna.1.iter() {
                if ant2 == ant1 {
                    continue;
                }

                let anti1 = (2 * ant1.0 - ant2.0, 2 * ant1.1 - ant2.1);
                let anti2 = (2 * ant2.0 - ant1.0, 2 * ant2.1 - ant1.1);

                if anti1.0 >= 0 && anti1.0 < heigth && anti1.1 >= 0 && anti1.1 < heigth {
                    if !unique_antennas.contains(&anti1) {
                        unique_antennas.push(anti1);
                    };
                }

                if anti2.0 >= 0 && anti2.0 < heigth && anti2.1 >= 0 && anti2.1 < heigth {
                    if !unique_antennas.contains(&anti2) {
                        unique_antennas.push(anti2);
                    };
                }
            }
        }
    }

    println!("Sum is: {}\n", unique_antennas.iter().count());
}

fn pretty_print(antenna: HashMap<char, Vec<(i32, i32)>>) {
    for el in antenna {
        print!("{} :", el.0);
        for pos in el.1 {
            print!("({}, {})", pos.0, pos.1);
        }
        println!();
    }
}
