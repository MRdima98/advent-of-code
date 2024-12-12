use std::{usize, vec};

pub fn run() {
    let input = include_str!("../input.txt");
    let mut xmas = vec![];

    for line in input.lines() {
        let mut row: Vec<&str> = line.split("").collect();
        row.pop();
        row.remove(0);
        xmas.push(row);
    }

    let mut sum = 0;
    for (i, row) in xmas.iter().enumerate() {
        sum += count_line(row.to_vec());
    }

    let mut inverted = xmas.clone();
    for (i, row) in xmas.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            inverted[j][i] = el;
        }
    }

    for row in inverted.iter() {
        sum += count_line(row.to_vec());
    }

    let mut diag = vec![];
    for mut y in 0..xmas.len() {
        let mut x = 0;
        let mut row = vec![];
        for _ in 0..xmas.len() {
            let Some(tmp) = xmas.get(x) else {
                continue;
            };
            let Some(tmp) = tmp.get(y) else {
                continue;
            };

            row.push(*tmp);

            x += 1;
            y += 1;
        }
        if row.len() > 3 {
            diag.push(row);
        }
    }

    for ele in diag.iter_mut() {
        sum += count_line(ele.to_vec());
    }

    let mut diag2 = vec![];
    for mut x in 0..xmas.len() {
        let mut y = 0;
        let mut row = vec![];
        for _ in 0..xmas.len() {
            let Some(tmp) = xmas.get(x) else {
                continue;
            };
            let Some(tmp) = tmp.get(y) else {
                continue;
            };

            row.push(*tmp);

            x += 1;
            y += 1;
        }
        if row.len() > 3 {
            diag2.push(row);
        }
    }

    diag2.remove(0);

    for ele in diag2.iter_mut() {
        sum += count_line(ele.to_vec());
    }

    let mut diag3 = vec![];
    for mut x in 0..xmas.len() {
        let mut y = xmas.len() - 1;
        let mut row = vec![];
        for _ in 0..xmas.len() {
            let Some(tmp) = xmas.get(x) else {
                continue;
            };
            let Some(tmp) = tmp.get(y) else {
                continue;
            };

            row.push(*tmp);

            x += 1;
            if y == 0 {
                break;
            }
            y -= 1;
        }
        if row.len() > 3 {
            diag3.push(row);
        }
    }

    for ele in diag3.iter_mut() {
        sum += count_line(ele.to_vec());
    }

    let mut diag4 = vec![];
    for mut x in (0..xmas.len()).rev() {
        let mut y = 0;
        let mut row = vec![];
        for _ in 0..xmas.len() {
            let Some(tmp) = xmas.get(x) else {
                continue;
            };
            let Some(tmp) = tmp.get(y) else {
                continue;
            };

            row.push(*tmp);

            if x == 0 {
                break;
            }
            x -= 1;
            y += 1;
        }
        if row.len() > 3 {
            diag4.push(row);
        }
    }

    diag4.remove(0);

    for ele in diag4.iter_mut() {
        sum += count_line(ele.to_vec());
    }

    println!("\n\nThis is my sum!: {sum} \n\n");
}

fn count_line(row: Vec<&str>) -> usize {
    let mut count = 0;
    for (i, el) in row.iter().enumerate() {
        match *el {
            "X" => {
                count += is_xmas(&row, i);
            }
            "S" => {
                count += is_samx(&row, i);
            }
            _ => {}
        };
    }

    return count;
}

fn is_xmas(row: &Vec<&str>, index: usize) -> usize {
    let Some(el) = row.get(index + 1) else {
        return 0;
    };
    if *el != "M" {
        return 0;
    };

    let Some(el) = row.get(index + 2) else {
        return 0;
    };
    if *el != "A" {
        return 0;
    };

    let Some(el) = row.get(index + 3) else {
        return 0;
    };
    if *el != "S" {
        return 0;
    };

    return 1;
}

fn is_samx(row: &Vec<&str>, index: usize) -> usize {
    let Some(el) = row.get(index + 1) else {
        return 0;
    };
    if *el != "A" {
        return 0;
    };

    let Some(el) = row.get(index + 2) else {
        return 0;
    };
    if *el != "M" {
        return 0;
    };

    let Some(el) = row.get(index + 3) else {
        return 0;
    };
    if *el != "X" {
        return 0;
    };

    return 1;
}
