use core::time;
use std::collections::HashMap;
use std::{fmt::Display, ops, thread, usize};

#[derive(Debug)]
struct Data {
    tot: usize,
    sequence: Vec<usize>,
}

#[derive(Clone, Copy)]
enum Operation {
    Sum,
    Molt,
}
impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Sum => write!(f, "+"),
            Operation::Molt => write!(f, "*"),
        }
    }
}

pub fn run() {
    let input = include_str!("../input");
    let mut data: Vec<Data> = vec![];
    let mut longest_arr: usize = 0;

    for line in input.lines() {
        let mut tmp: Vec<&str> = line.split_whitespace().collect();
        let tmp_str = tmp.remove(0);
        let tmp_str = &tmp_str[0..tmp_str.len() - 1];
        if tmp.len() > longest_arr {
            longest_arr = tmp_str.len();
        }

        data.push(Data {
            tot: num_to_int(tmp_str),
            sequence: vec_to_usize(&tmp),
        });
    }

    println!("long! {longest_arr}");
    let mut possible_op: Vec<Vec<Operation>> = vec![];
    let mut longest_arr = longest_arr as u32;
    longest_arr = (2 as u32).pow(longest_arr) - 1;
    println!("{longest_arr}");
    for i in 0..=longest_arr {
        let tmp = random_binary(i as usize);
        possible_op.push(tmp);
    }
    println!("{}", possible_op.len());

    let mut sum = 0;
    for d in data {
        //println!("Sec {}", d.tot);
        let t = d.sequence.len();
        //let possibilities = t * (t + 1);
        for i in 0..=possible_op.len() {
            let mut acc = 0;
            let Some(ops) = possible_op.get(i) else {
                continue;
            };

            //for o in ops {
            //    print!("{o}");
            //}
            //println!();

            for (i, el) in d.sequence.iter().enumerate() {
                if i == 0 {
                    acc = *el;
                    continue;
                }

                let Some(op) = ops.get(i - 1) else {
                    break;
                };

                match op {
                    Operation::Sum => acc += *el,
                    Operation::Molt => acc *= *el,
                }
            }
            //println!();
            if d.tot == acc {
                println!("Hit: {acc}");
                sum += acc;
                break;
            }
        }
        println!();
    }

    print!("\n\nThe sum of part 1 is: {sum}\n\n");
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

fn random_binary(num: usize) -> Vec<Operation> {
    let mut op: Vec<Operation> = vec![];
    let tmp = format!("{num:13b}");

    for el in tmp.chars() {
        match el {
            '1' => op.push(Operation::Molt),
            _ => op.push(Operation::Sum),
        }
    }

    op.reverse();

    //print!("{tmp}: ");
    //for o in op.iter() {
    //    print!("{o}");
    //}
    //println!();

    op
}

fn pretty_print(data: &[Operation]) {
    for el in data.iter() {
        print!("{el }");
    }
}
