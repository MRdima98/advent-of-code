use std::usize;

use regex::Regex;

pub fn run() {
    let input = include_str!("../input.txt");

    let mul_re = Regex::new(r"mul\(\d*,\d*\)").expect("This is a balls regex");
    let num_re = Regex::new(r"\d+").expect("This is a balls regex");

    let mut res_mul = vec![];
    let mut couple = vec![];

    for cap in mul_re.captures_iter(input) {
        for c in cap.iter() {
            if let Some(mul) = c {
                res_mul.push(mul.as_str());
            };
        }
    }

    for mul in res_mul {
        let mut cap = num_re.captures_iter(mul);
        let num1: usize = cap
            .next()
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let num2: usize = cap
            .next()
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        couple.push((num1, num2));
    }

    let mut sum = 0;
    for c in couple {
        sum += c.0 * c.1;
    }

    println!("Part 1 res: {:?}", sum);
}
