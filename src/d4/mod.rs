use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn count_adjacent(x: usize, y: usize, input: &[Vec<bool>]) -> u8 {
    let x_i32 = x as i32;
    let y_i32 = y as i32;
    let max_y = input.len() as i32;
    let max_x = input[0].len() as i32;

    let adjacent = vec![
        (y_i32 - 1, x_i32 - 1),
        (y_i32 - 1, x_i32),
        (y_i32 - 1, x_i32 + 1),
        (y_i32 + 1, x_i32 - 1),
        (y_i32 + 1, x_i32),
        (y_i32 + 1, x_i32 + 1),
        (y_i32, x_i32 - 1),
        (y_i32, x_i32 + 1),
    ]
    .into_iter()
    .filter_map(|(y, x)| {
        if !(0..max_y).contains(&y) || !(0..max_x).contains(&x) {
            return None;
        }
        Some((y as usize, x as usize))
    })
    .collect::<Vec<_>>();

    adjacent.iter().map(|(y, x)| input[*y][*x] as u8).sum()
}

pub fn solve() {
    let f = File::open("./src/d4/input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut input: Vec<_> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| matches!(c, '@'))
                .collect::<Vec<bool>>()
        })
        .collect();

    dbg!(&input);

    // P1
    let mut count = 0;
    for y in 0..input.len() {
        let row = input.get(y).unwrap();
        for x in 0..row.len() {
            if input[y][x] {
                let adjacent = count_adjacent(x, y, &input);
                if adjacent < 4 {
                    count += 1;
                }
            }
        }
    }
    println!("P1: {count}");

    // P2
    let mut count = 0;
    let mut keep_going = true;

    while keep_going {
        keep_going = false;
        for y in 0..input.len() {
            let row = input.get(y).unwrap();
            for x in 0..row.len() {
                if input[y][x] {
                    let adjacent = count_adjacent(x, y, &input);
                    if adjacent < 4 {
                        input[y][x] = false;
                        keep_going = true;
                        count += 1;
                    }
                }
            }
        }
    }
    println!("P2: {count}");
}
