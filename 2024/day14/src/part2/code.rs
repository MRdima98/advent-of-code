use std::fs;

pub fn run(path: &str) {
    let input: String = fs::read_to_string(path).unwrap();

    for line in input.lines() {}

    print!("Hello from part2\n");
}
