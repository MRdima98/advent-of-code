use core::time;
use std::{fmt::Display, ops, thread, usize};

#[derive(Debug)]
struct Data {
    tot: usize,
    sequence: Vec<usize>,
}

#[derive(Clone, Copy)]
enum Operation {
    Root,
    Sum,
    Molt,
}
impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Sum => write!(f, "+"),
            Operation::Molt => write!(f, "*"),
            Operation::Root => write!(f, "R"),
        }
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
            sequence: vec_to_usize(&tmp),
        });
    }

    let mut sum = 0;
    let mut acc = 0;

    for el in data.iter_mut() {
        sum += is_valid(el.tot, &mut el.sequence, &mut acc);
    }

    print!("\n\nThe sum of part 1 is: {sum}\n\n");
}

fn is_valid(tot: usize, sequence: &mut Vec<usize>, acc: &mut usize) -> usize {
    //print!("My accumulator: {:?}", acc);
    //thread::sleep(time::Duration::from_millis(200));
    let Some(num) = sequence.pop() else {
        if tot == *acc {
            println!("found match: ");
            return tot;
        } else {
            return 0;
        }
    };

    if *acc == 0 {
        return is_valid(tot, sequence, &mut (*acc + num));
    } else {
        return is_valid(tot, sequence, &mut (*acc * num));
    }

    //if *acc == 0 {
    //    *acc = 1;
    //}
    //
    //return is_valid(tot, sequence, &mut (*acc * num));
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
