use std::{fs, usize, vec};

pub fn run(path: &str) {
    let input: String = fs::read_to_string(path).unwrap();
    let (mut a_register, mut b_register, mut c_register, opcodes) = parse_input(&input);
    let mut output: Vec<usize> = vec![];

    println!(
        "{}, {}, {}, {:?},",
        a_register, c_register, c_register, opcodes
    );

    let mut i = 0;
    loop {
        println!("{}, {}, {} ", a_register, b_register, c_register);
        let Some(op) = opcodes.get(i) else {
            break;
        };
        let op = *op;

        let Some(literal) = opcodes.get(i + 1) else {
            break;
        };
        let literal = *literal;
        let mut combo = literal;
        i += 2;

        if literal == Combos::A as usize {
            combo = a_register;
        } else if literal == Combos::B as usize {
            combo = b_register;
        } else if literal == Combos::C as usize {
            combo = c_register;
        }

        if op == Opcodes::Adv as usize {
            println!("op: {op}, literal: {literal}");
            print!("Adv A = A / {} -> ", 2usize.pow(combo as u32));
            a_register = a_register / 2usize.pow(combo as u32);
        }

        if op == Opcodes::Bxl as usize {
            print!("Bxl B = B ^ {} -> ", literal);
            b_register = b_register ^ literal;
        }

        if op == Opcodes::Bst as usize {
            print!("Bst B = {literal} % 8 -> ");
            b_register = combo % 8;
        }

        if op == Opcodes::Jnz as usize && a_register != 0 {
            print!("Jnz jump -> ");
            println!("Looped once\n");
            i = literal;
        }

        if op == Opcodes::Bxc as usize {
            print!("Bxc B = B ^ C -> ");
            b_register = b_register ^ c_register;
        }

        if op == Opcodes::Out as usize {
            print!("Out print -> ");
            //println!("{}, {}, {a_register}", literal, literal % 8);
            output.push(combo % 8);
        }

        if op == Opcodes::Bdv as usize {
            print!("Bdv B = A / {} -> ", 2usize.pow(literal as u32));
            b_register = a_register / 2usize.pow(combo as u32);
        }

        if op == Opcodes::Cdv as usize {
            print!("Cdv C = A / {} -> ", 2usize.pow(literal as u32));
            c_register = a_register / 2usize.pow(combo as u32);
        }
    }

    let mut res: String = Default::default();
    for el in output {
        res += &(el.to_string() + &",");
    }

    //println!("{:?}", &res[..res.len() - 1]);
    println!("{res}");
}

enum Combos {
    A = 4,
    B,
    C,
}

enum Opcodes {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

fn parse_input(input: &str) -> (usize, usize, usize, Vec<usize>) {
    let mut line = input.lines();
    let a_register = line.next().unwrap().split_whitespace();
    let a_register = a_register.last().unwrap();
    let a_register = a_register.parse::<usize>().unwrap();

    let b_register = line.next().unwrap().split_whitespace();
    let b_register = b_register.last().unwrap();
    let b_register = b_register.parse::<usize>().unwrap();

    let c_register = line.next().unwrap().split_whitespace();
    let c_register = c_register.last().unwrap();
    let c_register = c_register.parse::<usize>().unwrap();

    line.next();
    let chars = line.next().unwrap().chars();
    let mut nums = vec![];
    for el in chars {
        if let Some(digit) = el.to_digit(10) {
            nums.push(digit as usize);
        }
    }

    (a_register, b_register, c_register, nums)
}
