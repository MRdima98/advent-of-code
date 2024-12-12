use core::time;
use std::{
    fmt::{write, Display},
    thread,
};

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
    for (i, el) in disk.iter().enumerate() {
        let tmp: Disk;
        if i % 2 == 0 {
            tmp = Disk::Val(id);
            id += 1;
        } else {
            tmp = Disk::Dot;
        }
        for _ in 0..*el {
            unpacked_disk.push(tmp);
        }
    }
    //pretty_print(&unpacked_disk);

    loop {
        let mut i = 0;
        let mut j = unpacked_disk.len() - 1;
        loop {
            if i >= unpacked_disk.len() {
                break;
            }
            match unpacked_disk[i] {
                Disk::Val(_) => {
                    i += 1;
                }
                Disk::Dot => {
                    break;
                }
            };
        }

        loop {
            match unpacked_disk[j] {
                Disk::Val(_) => {
                    break;
                }
                Disk::Dot => {
                    if j == 0 {
                        break;
                    }
                    j -= 1;
                }
            };
        }

        let tmp = unpacked_disk[i];
        unpacked_disk[i] = unpacked_disk[j];
        unpacked_disk[j] = tmp;

        if contig_contiguous(&unpacked_disk) {
            break;
        }
    }

    let mut sum = 0;
    for (i, el) in unpacked_disk.iter().enumerate() {
        match el {
            Disk::Val(el) => {
                sum += i * el;
            }
            Disk::Dot => {
                break;
            }
        }
    }

    println!("The res is: {sum}");
}

fn contig_contiguous(arr: &[Disk]) -> bool {
    let mut count = 0;
    for (i, el) in arr.iter().enumerate() {
        let Some(tmp) = arr.get(i + 1) else {
            break;
        };

        match el {
            Disk::Val(_) => match tmp {
                Disk::Val(_) => {}
                Disk::Dot => count += 1,
            },
            Disk::Dot => {}
        }
    }

    if count > 1 {
        return false;
    }

    true
}

fn pretty_print(arr: &[Disk]) {
    for el in arr {
        print!("{el}");
    }
    println!();
}
