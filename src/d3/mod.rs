use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() {
    // P1
    let f = File::open("./src/d3/input.txt").unwrap();
    let reader = BufReader::new(f);

    let sum: u64 = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let max = max_num(&line, 2);
            max.parse::<u64>().unwrap()
        })
        .sum();

    println!("P1: {sum}");

    // P2
    let f = File::open("./src/d3/input.txt").unwrap();
    let reader = BufReader::new(f);

    let sum: u64 = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let max = max_num(&line, 12);
            max.parse::<u64>().unwrap()
        })
        .sum();

    println!("P2: {sum}");
}

fn max_num(input: &str, max_len: usize) -> String {
    let mut stack: VecDeque<char> = VecDeque::new();
    let mut to_remove = input.len() - max_len;

    for c in input.chars() {
        while to_remove > 0 && !stack.is_empty() && stack.back().unwrap() < &c {
            stack.pop_back();
            to_remove -= 1;
        }
        stack.push_back(c);
    }

    stack.truncate(max_len);
    stack.into_iter().collect()
}
