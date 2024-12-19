use std::env;

use template::part1::code::run as run1;
use template::part2::code::run as run2;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "1" {
        run1("./input");
    }

    if args[1] == "2" {
        run2("./input");
    }

    if args[1] != "1" || args[1] == "2" {
        println!("Pass either 1 or 2 as args");
    }
}
