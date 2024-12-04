use std::usize;

struct Position {
    x: i32,
    y: i32,
}

pub fn run() {
    let input = include_str!("../input.txt");
    let mut xmas = vec![];

    for line in input.lines() {
        let mut row: Vec<&str> = line.split("").collect();
        row.pop();
        row.remove(0);
        xmas.push(row);
    }

    let mut sum = 0;
    for (x, row) in xmas.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            sum += count_xmas(
                xmas.clone(),
                "",
                0,
                Position {
                    x: x as i32,
                    y: y as i32,
                },
            );
        }
    }

    println!("This is my sum!: {sum}");
}

fn count_xmas(matr: Vec<Vec<&str>>, mut prev: &str, mut num: u32, pos: Position) -> u32 {
    if pos.x < 0 || pos.y < 0 {
        return 0;
    }

    let Some(row) = matr.get(pos.x as usize) else {
        return 0;
    };

    let Some(curr) = row.get(pos.y as usize) else {
        return 0;
    };

    match (*curr, num, prev) {
        // XMAS
        ("S", 3, "XMA") => {
            println!("XMAS end {}, {}\n", pos.x, pos.y);
            return 1;
        }
        ("A", 2, "XM") => {
            num += 1;
            prev = "XMA";
        }
        ("M", 1, "X") => {
            num += 1;
            prev = "XM";
        }
        ("X", 0, "") => {
            println!("XMAS start {}, {}", pos.x, pos.y);
            num += 1;
            prev = "X";
        }
        // SAMX
        ("X", 3, "SAM") => {
            println!("SAMX end {}, {}\n", pos.x, pos.y);
            return 1;
        }
        ("M", 2, "SA") => {
            num += 1;
            prev = "SAM";
        }
        ("A", 1, "S") => {
            num += 1;
            prev = "SA";
        }
        ("S", 0, "") => {
            println!("SAMX start {}, {}", pos.x, pos.y);
            num += 1;
            prev = "S";
        }
        (_, _, _) => {
            return 0;
        }
    }

    count_xmas(
        matr.clone(),
        prev,
        num,
        Position {
            x: pos.x - 1,
            y: pos.y - 1,
        },
    ) + count_xmas(
        matr.clone(),
        prev,
        num,
        Position {
            x: pos.x,
            y: pos.y - 1,
        },
    ) + count_xmas(
        matr.clone(),
        prev,
        num,
        Position {
            x: pos.x,
            y: pos.y + 1,
        },
    ) + count_xmas(
        matr.clone(),
        prev,
        num,
        Position {
            x: pos.x - 1,
            y: pos.y,
        },
    ) + count_xmas(
        matr.clone(),
        prev,
        num,
        Position {
            x: pos.x + 1,
            y: pos.y,
        },
    ) + count_xmas(
        matr.clone(),
        prev,
        num,
        Position {
            x: pos.x + 1,
            y: pos.y - 1,
        },
    ) + count_xmas(
        matr.clone(),
        prev,
        num,
        Position {
            x: pos.x - 1,
            y: pos.y + 1,
        },
    ) + count_xmas(
        matr.clone(),
        prev,
        num,
        Position {
            x: pos.x - 1,
            y: pos.y - 1,
        },
    )
}
