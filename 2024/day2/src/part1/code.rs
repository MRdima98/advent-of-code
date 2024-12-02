enum Order {
    asc,
    desc,
    unset,
}

pub fn run() {
    let input = include_str!("../example_input.txt");
    let mut levels = vec![];
    let mut level = vec![];

    for line in input.lines() {
        level = line_to_num(line);
        levels.push(level);
    }

    let mut safe_levels = 0;
    let mut safe: bool;
    for level in levels {
        safe = true;
        let mut order = Order::unset;
        for (i, el) in level.iter().enumerate() {
            match level.get(i + 1) {
                Some(tmp) => {
                    let tmp2 = (el - tmp).abs();

                    match order {
                        Order::unset => {
                            if el > tmp {
                                order = Order::asc;
                            }

                            if el < tmp {
                                order = Order::desc;
                            }
                        }
                        Order::asc => {
                            if el < tmp {
                                safe = false;
                                break;
                            }
                        }
                        Order::desc => {
                            if el > tmp {
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

        if safe {
            safe_levels += 1;
        }
    }
}

fn line_to_num(line: &str) -> Vec<i32> {
    let tmp = str::split_ascii_whitespace(line);
    let mut vec: Vec<i32> = vec![];
    for el in tmp {
        vec.push(el.trim().parse().expect("Can't parse right"));
    }
    vec
}
