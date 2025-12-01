use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DIAL_START: i32 = 50;

#[derive(Debug)]
enum Instruction {
    L(i32),
    R(i32),
}

pub fn solve() {
    let f = File::open("./src/d1/input.txt").unwrap();
    let reader = BufReader::new(f);

    let instructions: Vec<_> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (direction, value) = line.split_at(1);
            match direction {
                "L" => Instruction::L(value.parse().unwrap()),
                "R" => Instruction::R(value.parse().unwrap()),
                _ => panic!("invalid input"),
            }
        })
        .collect();

    dbg!(&instructions);

    // P1
    let mut dial = DIAL_START;
    let mut count = 0;
    for instruction in instructions.iter() {
        match instruction {
            Instruction::L(val) => {
                dial = (dial + 100 - (val % 100)) % 100;
            }
            Instruction::R(val) => {
                dial = (dial + val) % 100;
            }
        }
        if dial == 0 {
            count += 1;
        }
    }
    println!("P1: {count}");

    // P2
    let mut dial = DIAL_START;
    let mut count = 0;
    for instruction in instructions {
        match instruction {
            Instruction::L(val) => {
                let full_rotations = val / 100;
                let remainder = val % 100;

                count += full_rotations;

                if remainder >= dial && dial != 0 {
                    count += 1;
                }

                dial = (dial + 100 - remainder) % 100;
            }
            Instruction::R(val) => {
                let full_rotations = val / 100;
                let remainder = val % 100;

                count += full_rotations;

                if dial + remainder >= 100 {
                    count += 1;
                }

                dial = (dial + val) % 100;
            }
        }
    }
    println!("P2: {count}");
}
