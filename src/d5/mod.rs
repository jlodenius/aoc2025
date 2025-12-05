use std::ops::RangeInclusive;

pub fn solve() {
    let input = include_str!("./input.txt");

    let index = input.find("\n\n").unwrap();
    let (ranges, ids) = input.split_at(index);

    let mut ranges: Vec<RangeInclusive<u64>> = ranges
        .trim()
        .split('\n')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    let ids: Vec<u64> = ids
        .trim()
        .split('\n')
        .map(|id| id.parse().unwrap())
        .collect();

    // P1
    let mut count = 0;
    for id in ids {
        if ranges.iter().any(|range| range.contains(&id)) {
            count += 1;
        }
    }

    println!("P1: {count}");

    // P2
    ranges.sort_by_key(|range| *range.start());
    let count: u64 = ranges
        .iter()
        .fold(vec![], |mut acc: Vec<RangeInclusive<u64>>, cur| {
            if let Some(last) = acc.last_mut() {
                // overlap
                if cur.start() <= last.end() {
                    *last = *last.start()..=*(last.end().max(cur.end()));
                    return acc;
                }
                // no overlap
                acc.push(cur.clone());
            } else {
                // first value
                acc.push(cur.clone());
            };
            acc
        })
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum();

    println!("P2: {count}");
}
