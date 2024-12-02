enum Order {
    asc,
    desc,
    unset,
}

enum Ordeal {
    order(Order),
    safe(bool),
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
    let mut one_boom = 0;
    for level in levels {
        safe = true;
        one_boom = 0;
        println!("Level: {:?}", level);
        let mut order = Order::unset;
        for (i, el) in level.iter().enumerate() {
            match level.get(i + 1) {
                Some(tmp) => {
                    let stuf = part1_works(*tmp, *el, &mut order);
                    match stuf {
                        Ordeal::safe(safe_one) => {
                            if !safe_one && one_boom == 0 {
                                println!("One boom: {one_boom}");
                                one_boom += 1;
                            } else {
                                safe = false;
                                break;
                            }
                            //if !safe_one {
                            //    safe = false;
                            //    break;
                            //}
                        }
                        Ordeal::order(order_one) => {
                            order = order_one;
                        }
                    }
                }
                None => {}
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

fn part1_works(tmp: i32, el: i32, order: &mut Order) -> Ordeal {
    match order {
        Order::unset => {
            if el > tmp {
                return Ordeal::order(Order::asc);
            }

            if el < tmp {
                return Ordeal::order(Order::desc);
            }
        }
        Order::asc => {
            if el < tmp {
                return Ordeal::safe(false);
            }
        }
        Order::desc => {
            if el > tmp {
                return Ordeal::safe(false);
            }
        }
    }

    let tmp2 = (el - tmp).abs();

    if !(tmp2 <= 3 && tmp2 >= 1) {
        return Ordeal::safe(false);
    }

    Ordeal::safe(true)
}
