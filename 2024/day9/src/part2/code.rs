use core::time;
use std::{fmt::Display, thread};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Disk {
    Val(usize),
    Dot,
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Disk::Val(val) => write!(f, "{val}"),
            Disk::Dot => write!(f, "."),
        }
    }
}

pub fn run() {
    let input = include_str!("../input");
    let mut disk = vec![];
    let mut unpacked_disk = vec![];

    for line in input.lines() {
        for el in line.chars() {
            disk.push(el.to_digit(10).unwrap());
        }
    }

    let mut id = 0;
    let mut checked_id: Vec<(usize, bool)> = vec![];
    for (i, el) in disk.iter().enumerate() {
        let tmp: Disk;
        if i % 2 == 0 {
            tmp = Disk::Val(id);
            checked_id.push((id, false));
            id += 1;
        } else {
            tmp = Disk::Dot;
        }
        for _ in 0..*el {
            unpacked_disk.push(tmp);
        }
    }

    //pretty_print(&unpacked_disk);

    for el in checked_id.iter_mut().rev() {
        println!("El: {}", el.0);
        let id_coord = get_id_coord(&unpacked_disk, el.0);
        el.1 = true;

        let Some(space_coord) =
            get_space_coord(&unpacked_disk, (id_coord.1 - id_coord.0) + 1, id_coord.0)
        else {
            continue;
        };

        //print!("Num : ");
        //for i in id_coord.0..=id_coord.1 {
        //    print!("{}", unpacked_disk[i]);
        //}
        //println!();

        //print!("Space: ");
        //for i in space_coord.0..=space_coord.1 {
        //    print!("{}", unpacked_disk[i]);
        //}
        //println!();

        //pretty_print(&unpacked_disk);
        //println!();
        //thread::sleep(time::Duration::from_millis(200));

        let mut j = space_coord.0;
        for i in id_coord.0..=id_coord.1 {
            let tmp = unpacked_disk[i];
            unpacked_disk[i] = unpacked_disk[j];
            unpacked_disk[j] = tmp;
            j += 1;
        }
    }

    pretty_print(&unpacked_disk);

    let mut sum = 0;
    for (i, el) in unpacked_disk.iter().enumerate() {
        println!("Iter: {i}");
        match el {
            Disk::Val(el) => {
                sum += i * el;
            }
            Disk::Dot => {
                continue;
            }
        }
    }

    println!("The res is: {sum}");
}

fn get_space_coord(unpacked_disk: &[Disk], size: usize, limit: usize) -> Option<(usize, usize)> {
    let mut space = vec![];
    for (i, el) in unpacked_disk.iter().enumerate() {
        if i == limit {
            return None;
        }

        match el {
            Disk::Dot => {
                space.push(i);
            }
            _ => {
                if space.len() >= size {
                    return Some((space[0], space[space.len() - 1]));
                }

                space = vec![];
            }
        }
    }
    None
}

fn get_id_coord(unpacked_disk: &[Disk], id: usize) -> (usize, usize) {
    let mut coord = (0, unpacked_disk.len() - 1);
    let mut hit = false;
    for (i, el) in unpacked_disk.iter().enumerate() {
        let tmp;
        match el {
            Disk::Val(tmp2) => tmp = tmp2,
            Disk::Dot => {
                if hit {
                    hit = false;
                    coord.1 = i - 1;
                }
                continue;
            }
        }

        if *tmp == id && !hit {
            coord.0 = i;
            hit = true;
        }

        if *tmp != id && hit {
            hit = false;
            coord.1 = i - 1;
        }
    }
    coord
}

fn pretty_print(arr: &[Disk]) {
    for el in arr {
        print!("{el}");
    }
    println!();
}
