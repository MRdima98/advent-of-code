use std::{env::consts, fs, usize};

use regex::Regex;

pub fn run(path: &str) {
    let input: String = fs::read_to_string(path).unwrap();
    let num_re = Regex::new(r"\d+").expect("This is a balls regex");
    let mut every_num = vec![];
    let mut button_A = vec![];
    let mut button_B = vec![];
    let mut prize = vec![];

    for line in input.lines() {
        if line == "".to_string() {
            continue;
        }
        let mut matches = num_re.captures_iter(line);
        let num1: usize = matches
            .next()
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let num2: usize = matches
            .next()
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        every_num.push((num1, num2));
    }
    println!("{:?}", every_num);

    for i in (0..every_num.len()).step_by(3) {
        button_A.push(every_num[i]);
    }

    for i in (1..every_num.len()).step_by(3) {
        button_B.push(every_num[i]);
    }

    for i in (2..every_num.len()).step_by(3) {
        prize.push(every_num[i]);
    }

    println!("A: {:?}\nB: {:?}\nP: {:?}\n", button_A, button_B, prize);
    let mut sum = 0;
    for i in 0..prize.len() {
        println!("Not stuck {i}");
        let mut combos = vec![];
        get_sums(
            button_A[i],
            button_B[i],
            prize[i],
            (0, 0),
            &mut vec![],
            &mut combos,
            (0, 0),
        );

        if combos.is_empty() {
            continue;
        }
        let mut costs = vec![];

        for el in combos {
            costs.push(el.0 * 3 + el.1);
        }

        sum += costs.iter().min().unwrap();
    }
    println!("Sum is: {:?}", sum);
}

fn get_sums(
    button_A: (usize, usize),
    button_B: (usize, usize),
    prize: (usize, usize),
    acc: (usize, usize),
    visited: &mut Vec<(usize, usize)>,
    costs: &mut Vec<(usize, usize)>,
    pushed: (usize, usize),
) -> usize {
    if acc.0 == prize.0 && acc.1 == prize.1 {
        costs.push(pushed);
        return 1;
    }

    if !visited.contains(&acc) {
        visited.push(acc);
    } else {
        return 0;
    }

    if acc.0 > prize.0 || acc.1 > prize.1 {
        return 0;
    }

    return get_sums(
        button_A,
        button_B,
        prize,
        (acc.0 + button_A.0, acc.1 + button_A.1),
        visited,
        costs,
        (pushed.0 + 1, pushed.1),
    ) + get_sums(
        button_A,
        button_B,
        prize,
        (acc.0 + button_B.0, acc.1 + button_B.1),
        visited,
        costs,
        (pushed.0, pushed.1 + 1),
    );
}
