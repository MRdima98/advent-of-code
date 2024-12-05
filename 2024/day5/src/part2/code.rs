use std::vec;

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
    for (_, row) in updates.iter_mut().enumerate() {
        let mut is_valid = true;
        loop {
            let mut has_changed = false;
            check_rule(&rules, &mut is_valid, &mut has_changed, row);
            if !has_changed {
                break;
            }
        }
        if !is_valid {
            valid_updates.push(row);
        }
    }

    let mut count = 0;
    for up in valid_updates.iter() {
        count += up[up.len() / 2];
    }

    println!("Sum part 2 is: {count}");
}

fn line_to_num(line: &str, divider: &str) -> Vec<i32> {
    let tmp: Vec<&str> = line.split(divider).collect();
    let mut vec: Vec<i32> = vec![];
    for el in tmp {
        vec.push(el.trim().parse().expect("Can't parse right"));
    }
    vec
}

fn check_rule(rules: &Vec<Vec<i32>>, is_valid: &mut bool, has_changed: &mut bool, row: &mut [i32]) {
    for rule in rules.iter() {
        let Some(first) = row.iter().position(|&first| first == rule[0]) else {
            continue;
        };

        let Some(second) = row.iter().position(|&second| second == rule[1]) else {
            continue;
        };

        if first > second {
            (row[first], row[second]) = (row[second], row[first]);
            *is_valid = false;
            *has_changed = true;
        }
    }
}
