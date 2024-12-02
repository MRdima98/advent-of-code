enum Order {
    Asc,
    Desc,
    Unset,
}

pub fn run() {
    let input = include_str!("../example_input.txt");
    let mut levels = vec![];
    let mut level: Vec<i32>;

    for line in input.lines() {
        level = line_to_num(line);
        levels.push(level);
    }

    let mut safe_levels = 0;
    let mut safe: bool;
    for level in levels {
        safe = false;
        for (i, _) in level.iter().enumerate() {
            let mut copy = level.clone();
            copy.remove(i);
            if remove_boom(copy) {
                safe = true;
                break;
            }
        }
        println!("Safe: {safe}\n");
        if safe {
            safe_levels += 1;
        }
    }

    print!("Safe levels: {safe_levels} \n");
}

fn line_to_num(line: &str) -> Vec<i32> {
    let tmp = str::split_ascii_whitespace(line);
    let mut vec: Vec<i32> = vec![];
    for el in tmp {
        vec.push(el.trim().parse().expect("Can't parse right"));
    }
    vec
}

fn remove_boom(level: Vec<i32>) -> bool {
    let mut safe = true;

    let mut order = Order::Unset;
    for (i, el) in level.iter().enumerate() {
        match level.get(i + 1) {
            Some(tmp) => {
                let tmp2 = (el - tmp).abs();

                match order {
                    Order::Unset => {
                        if el < tmp {
                            order = Order::Asc;
                        }

                        if el > tmp {
                            order = Order::Desc;
                        }
                    }
                    Order::Asc => {
                        if el > tmp {
                            safe = false;
                            break;
                        }
                    }
                    Order::Desc => {
                        if el < tmp {
                            safe = false;
                            break;
                        }
                    }
                }

                if !(tmp2 <= 3 && tmp2 >= 1) {
                    safe = false;
                    break;
                }
            }
            None => {}
        }
    }

    safe
}
