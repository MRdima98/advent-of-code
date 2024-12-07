use std::{fmt::Display, ops, usize};

#[derive(Debug)]
struct Data {
    tot: usize,
    ops: Vec<usize>,
}

#[derive(Clone, Copy)]
enum Op {
    Sum,
    Molt,
}

#[derive(Clone, Copy)]
struct OpsList {
    nums: (usize, usize),
    op: Op,
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.ops.clone() {
            write!(f, "{},", i);
        }
        write!(f, ": {}", self.tot)
    }
}

pub fn run() {
    let input = include_str!("../input");
    let mut data: Vec<Data> = vec![];

    for line in input.lines() {
        let mut tmp: Vec<&str> = line.split_whitespace().collect();
        let tmp_str = tmp.remove(0);
        let tmp_str = &tmp_str[0..tmp_str.len() - 1];

        data.push(Data {
            tot: num_to_int(tmp_str),
            ops: vec_to_usize(&tmp),
        });
    }

    let mut sum = 0;
    for d in data {
        sum += valid_code(&mut get_possible_op(&d.ops), d.tot);
    }

    print!("\n\nThe sum of part 1 is: {sum}\n\n");
}

fn valid_code(lists: &mut [Vec<OpsList>], res: usize) -> usize {
    for row in lists.iter_mut() {
        //pretty_print(row);
        //println!();
        let mut acc: usize = 0;
        for el in row.iter_mut() {
            match el.op {
                Op::Sum => {
                    if acc == 0 {
                        acc = el.nums.1 + el.nums.0;
                    } else {
                        acc = acc + el.nums.1
                    }
                }
                Op::Molt => {
                    if acc == 0 {
                        acc = el.nums.1 * el.nums.0;
                    } else {
                        acc = acc * el.nums.1
                    }
                }
            }
            //println!("Sum: {}", acc);
        }
        //println!();

        if acc == res {
            return res;
        }
    }

    return 0;
}

fn num_to_int(num: &str) -> usize {
    num.parse().expect("Can't parse left")
}

fn vec_to_usize(num: &[&str]) -> Vec<usize> {
    let mut tmp = vec![];
    for n in num.iter() {
        tmp.push(num_to_int(n));
    }
    tmp
}

fn get_possible_op(nums: &[usize]) -> Vec<Vec<OpsList>> {
    let mut lists: Vec<Vec<OpsList>> = vec![];
    let mut all_plus: Vec<OpsList> = vec![];
    let mut all_molt: Vec<OpsList> = vec![];
    for (i, left) in nums.iter().enumerate() {
        let Some(right) = nums.get(i + 1) else {
            break;
        };

        if left == right {
            break;
        };

        all_molt.push(OpsList {
            nums: (*left, *right),
            op: Op::Molt,
        });

        all_plus.push(OpsList {
            nums: (*left, *right),
            op: Op::Sum,
        });
    }

    //println!("List: ");
    //for i in list.iter() {
    //    match i.op {
    //        Op::Sum => {
    //            print!("{}, {}", i.nums.0, i.nums.1);
    //        }
    //        Op::Molt => (),
    //    }
    //}
    //println!();

    for (i, _) in all_plus.iter().enumerate() {
        let mut tmp = all_plus.clone();
        let mut tmp2 = all_molt.clone();
        tmp[i].op = Op::Molt;
        tmp2[i].op = Op::Sum;

        lists.push(tmp);
        lists.push(tmp2);
    }

    lists.push(all_plus);
    lists.push(all_molt);

    //for l in lists.iter() {
    //    pretty_print(&l);
    //    println!();
    //}

    lists
}

fn pretty_print(fake_map: &Vec<OpsList>) {
    for row in fake_map {
        match row.op {
            Op::Sum => print!("{},{} +; ", row.nums.0, row.nums.1),
            Op::Molt => print!("{},{} *; ", row.nums.0, row.nums.1),
        }
    }
}
