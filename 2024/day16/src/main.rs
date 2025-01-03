use std::env;

use template::part1::code::run as run1;
use template::part2::code::run as run2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(choose) = args.get(1) else {
        println!("Pass either 1 or 2 as args");
        return;
    };

    if choose == "1" {
        run1("./input");
    } else if choose == "2" {
        run2("./input");
    } else {
        println!("Chief is either 1 or 2");
    }

}
