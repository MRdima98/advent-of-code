use core::time;
use std::{fmt::Display, ops, thread, usize};

#[derive(Debug)]
struct Data {
    tot: usize,
    ops: Vec<usize>,
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

#[derive(Clone, Copy)]
struct OpsList {
    nums: (usize, usize),
    op: Operation,
}

struct Tree {
    val: Operation,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.ops.clone() {
            write!(f, "{},", i);
        }
        write!(f, ": {}", self.tot)
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
            ops: vec_to_usize(&tmp),
        });
    }

    let mut sum = 0;
    //for d in data {
    //    //sum += valid_code(&mut get_possible_op(&d.ops), d.tot);
    //}
    let root = gen_tee(3);
    //print_tree(&Some(Box::new(root)));
    let mut arr: Vec<Operation> = vec![];
    let arr = get_random(&Some(Box::new(root)), &mut arr);
    for el in arr {
        print!("{}", el);
    }

    print!("\n\nThe sum of part 1 is: {sum}\n\n");
}

fn valid_code(lists: &mut [Vec<OpsList>], res: usize) -> usize {
    for row in lists.iter_mut() {
        //pretty_print(row);
        //println!();
        let mut acc: usize = 0;
        for el in row.iter_mut() {
            match el.op {
                Operation::Sum => {
                    if acc == 0 {
                        acc = el.nums.1 + el.nums.0;
                    } else {
                        acc = acc + el.nums.1
                    }
                }
                Operation::Molt => {
                    if acc == 0 {
                        acc = el.nums.1 * el.nums.0;
                    } else {
                        acc = acc * el.nums.1
                    }
                }
                Operation::Root => {}
            }
            //println!("Sum: {}", acc);
        }
        //println!();

        if acc == res {
            return res;
        }
    }

    return 0;
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

fn gen_tee(depth: usize) -> Tree {
    let mut root = Tree {
        val: Operation::Root,
        left: None,
        right: None,
    };

    for _ in 0..depth {
        add_leaf(&mut root);
    }

    root
}

fn add_leaf(root: &mut Tree) {
    if let Some(leaf) = &mut root.left {
        add_leaf(leaf);
    } else {
        root.left = Some(Box::new(Tree {
            val: Operation::Sum,
            left: None,
            right: None,
        }));
    }

    if let Some(leaf) = &mut root.right {
        add_leaf(leaf);
    } else {
        root.right = Some(Box::new(Tree {
            val: Operation::Molt,
            left: None,
            right: None,
        }));
    }
    return;
}

fn print_tree(root: &Option<Box<Tree>>) {
    let Some(root) = root else {
        print!(" ");
        return;
    };

    //print!("{}", root.val);
    print_tree(&root.left);
    print_tree(&root.right);
}

fn get_random(root: &Option<Box<Tree>>, arr: &mut Vec<Operation>) -> Vec<Operation> {
    let Some(root) = root else {
        return arr.to_vec();
    };

    let Some(left) = &root.left else {
        println!("push +");
        arr.push(Operation::Sum);
        return arr.to_vec();
    };

    let Some(right) = &root.right else {
        arr.push(Operation::Molt);
        return arr.to_vec();
    };

    get_random(&root.left, arr);
    get_random(&root.right, arr);

    return arr.to_vec();
}
