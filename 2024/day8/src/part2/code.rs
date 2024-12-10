use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../input");
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let heigth: i32 = input.lines().count() as i32;

    for (i, line) in input.lines().enumerate() {
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

    //pretty_print(antennas.clone());

    let mut unique_antennas: Vec<(i32, i32)> = vec![];

    for antenna in antennas.iter_mut() {
        for ant1 in antenna.1.iter() {
            for ant2 in antenna.1.iter() {
                if ant2 == ant1 {
                    continue;
                }

                let diff1 = (ant1.0 - ant2.0, ant1.1 - ant2.1);
                let diff2 = (ant2.0 - ant1.0, ant2.1 - ant1.1);
                let mut antis = get_line_anties(&mut ant1.clone(), diff1, heigth);
                let antis2 = get_line_anties(&mut ant2.clone(), diff2, heigth);
                antis.extend(antis2.iter());
                if !unique_antennas.contains(&ant1) {
                    unique_antennas.push(*ant1);
                };
                if !unique_antennas.contains(&ant2) {
                    unique_antennas.push(*ant2);
                };

                for a in antis.iter() {
                    if !unique_antennas.contains(&a) {
                        unique_antennas.push(*a);
                    };
                }
            }
        }
    }

    for i in 0..=12 {
        for el in unique_antennas.iter() {
            if el.0 == i {
                println!("Match: ({}, {})", el.0, el.1);
            }
        }
        println!();
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

fn get_line_anties(ant1: &mut (i32, i32), diff: (i32, i32), heigth: i32) -> Vec<(i32, i32)> {
    let mut anties = vec![];
    let mut anti = (ant1.0 + diff.0, ant1.1 + diff.1);

    loop {
        if anti.0 >= 0 && anti.0 < heigth && anti.1 >= 0 && anti.1 < heigth {
            anties.push(anti);
            anti = (anti.0 + diff.0, anti.1 + diff.1);
        } else {
            return anties;
        }
    }
}
