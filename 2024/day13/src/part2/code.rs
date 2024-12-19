use std::{
    backtrace, f64,
    fmt::Display,
    fs,
    ops::{Add, Div, Rem, Sub},
};

use regex::Regex;

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
struct Coord(f64, f64);

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Add for Coord {
    type Output = Coord;
    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Rem for Coord {
    type Output = Coord;
    fn rem(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub for Coord {
    type Output = Coord;
    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Div for Coord {
    type Output = Coord;

    fn div(self, rhs: Self) -> Self::Output {
        Coord(self.0 / rhs.0, self.1 / rhs.1)
    }
}

pub fn run(path: &str) {
    let input: String = fs::read_to_string(path).unwrap();
    let num_re = Regex::new(r"\d+").expect("This is a balls regex");
    let mut every_num = vec![];
    let mut button_a = vec![];
    let mut button_b = vec![];
    let mut prize = vec![];

    for line in input.lines() {
        if line == "".to_string() {
            continue;
        }
        let mut matches = num_re.captures_iter(line);
        let num1: f64 = matches
            .next()
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let num2: f64 = matches
            .next()
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        every_num.push(Coord(num1, num2));
    }

    for i in (0..every_num.len()).step_by(3) {
        button_a.push(every_num[i]);
    }

    for i in (1..every_num.len()).step_by(3) {
        button_b.push(every_num[i]);
    }

    let big = Coord(10000000000000.0, 10000000000000.0);
    for i in (2..every_num.len()).step_by(3) {
        prize.push(every_num[i] + big);
    }

    let mut sum = 0.0;
    for i in 0..prize.len() {
        sum += get_sums(button_a[i], button_b[i], prize[i]);
    }
    println!("Tot: {sum}");
}

fn get_sums(button_a: Coord, button_b: Coord, prize: Coord) -> f64 {
    let y = (prize.1 - (button_a.1 / button_a.0) * prize.0)
        / (-(button_a.1 * button_b.0) / button_a.0 + button_b.1);
    let x = (prize.0 - button_b.0 * y) / button_a.0;

    let x = x.round();
    let y = y.round();
    if (x * button_a.0 + y * button_b.0) as i64 == prize.0 as i64
        && (x * button_a.1 + y * button_b.1) as i64 == prize.1 as i64
    {
        println!("Sum: {}", 3.0 * x + y);
        return 3.0 * x + y;
    }

    0.0
}
