pub fn solve() {
    let input = include_str!("./input.txt").trim_end();

    // P1
    let mut count: u64 = 0;
    let ranges = input.split(",");

    for range in ranges {
        let (first, last) = range.split_once('-').unwrap();
        let first: u64 = first.parse().unwrap();
        let last: u64 = last.parse().unwrap();

        for id in first..=last {
            let id_str = id.to_string();
            let len = id_str.len();
            if len % 2 != 0 {
                // if number is not evenly divisible, it can't be composed of a sequence repeated
                // only TWICE
                continue;
            }
            let longest_possible_repeatable = len / 2;
            let a = &id_str[0..longest_possible_repeatable];
            let b = &id_str[longest_possible_repeatable..(longest_possible_repeatable * 2)];

            if a == b {
                count += id
            }
        }
    }
    println!("P1: {count}");

    // P2
    let mut count: u64 = 0;
    let ranges = input.split(",");

    for range in ranges {
        let (first, last) = range.split_once('-').unwrap();
        let first: u64 = first.parse().unwrap();
        let last: u64 = last.parse().unwrap();

        for id in first..=last {
            let id_str = id.to_string();
            let len = id_str.len();
            let longest_possible_repeatable = len / 2;

            let is_repeating = (1..=longest_possible_repeatable).any(|i| {
                let wanted = &id_str[0..i];
                id_str
                    .as_bytes()
                    .chunks(i)
                    .all(|chunk| unsafe { std::str::from_utf8_unchecked(chunk) } == wanted)
            });

            if is_repeating {
                count += id
            }
        }
    }
    println!("P2: {count}");
}
