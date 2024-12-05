use std::vec;

//use std::collections::HashMap;
pub fn run() {
    let input = include_str!("../input");
    let mut rules = vec![];
    let mut updates = vec![];

    let mut divider = "|";
    for line in input.lines() {
        if line == "" {
            divider = ",";
            continue;
        }

        let row = line_to_num(line, divider);

        if row.len() == 2 {
            rules.push(row);
        } else {
            updates.push(row);
        }
    }

    let mut valid_updates = vec![];
    for (_, row) in updates.iter().enumerate() {
        println!("{:?}", row);
        let mut is_valid = true;
        for rule in rules.iter() {
            let Some(first) = row.iter().position(|&first| first == rule[0]) else {
                //println!("breaking rule 2 {:?}", rule);
                continue;
            };

            let Some(second) = row.iter().position(|&second| second == rule[1]) else {
                //println!("breaking rule 2 {:?}", rule);
                continue;
            };

            if first > second {
                println!("Breaking order {:?}", rule);
                is_valid = false;
                break;
            }
        }
        println!();
        if is_valid {
            valid_updates.push(row);
        }
    }

    let mut count = 0;
    println!("Resulting after filter: ");
    for up in valid_updates.iter() {
        count += up[up.len() / 2];
        println!("{:?}", up);
    }

    println!("Sum is: {count}");
}

fn line_to_num(line: &str, divider: &str) -> Vec<i32> {
    let tmp: Vec<&str> = line.split(divider).collect();
    let mut vec: Vec<i32> = vec![];
    for el in tmp {
        vec.push(el.trim().parse().expect("Can't parse right"));
    }
    vec
}
