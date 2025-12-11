use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

// Note: u16 in diagram and wirings are bitmask
//
// diagram: [.##.]
// mask:     0110
//
// wiring: (3)
// mask:     1000
//
// wiring: (1,3)
// mask:     1010
//
// bitwise ops:
// or:  |=
// xor: ^=
#[derive(Debug, Default)]
struct Machine {
    diagram: u16,
    wirings: Vec<u16>,
    joltages: Vec<u16>,
}

impl Machine {
    fn min_clicks_to_power(&self) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(0u16);
        queue.push_back((0u16, 0));

        while let Some((cur, count)) = queue.pop_front() {
            for w in self.wirings.iter() {
                let new_mask = cur ^ w;
                if visited.contains(&new_mask) {
                    continue;
                }
                if new_mask == self.diagram {
                    return count + 1;
                }
                visited.insert(new_mask);
                queue.push_back((new_mask, count + 1));
            }
        }

        // Note:
        // this will never happen with current input
        usize::MAX
    }
}

impl From<String> for Machine {
    fn from(value: String) -> Self {
        let delim_1 = value.find(']').unwrap();
        let delim_2 = value.find('{').unwrap();

        // bitmask from the diagram input
        let mut diagram = 0u16;
        for c in value[1..delim_1].chars().rev() {
            diagram <<= 1;
            diagram |= match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("invalid input: {}", c),
            };
        }

        // bitmasks from the wirings input
        let wirings: Vec<u16> = value[delim_1 + 2..delim_2 - 1]
            .split(' ')
            .map(|s| {
                let trimmed = &s[1..s.len() - 1];
                let mut mask = 0u16;

                for part in trimmed.split(',') {
                    let num = part.trim().parse::<u8>().unwrap();
                    mask |= 1 << num
                }

                mask
            })
            .collect();

        let joltages: Vec<_> = value[delim_2 + 1..value.len() - 1]
            .split(',')
            .map(|part| part.trim().parse().unwrap())
            .collect();

        Self {
            diagram,
            wirings,
            joltages,
        }
    }
}

pub fn solve() {
    let f = File::open("./src/d_10/input.txt").unwrap();
    let reader = BufReader::new(f);

    let machines: Vec<Machine> = reader.lines().map(|line| line.unwrap().into()).collect();

    // P1
    let min_clicks: usize = machines.iter().map(|m| m.min_clicks_to_power()).sum();
    println!("P1: {min_clicks}");

    // P2, this probably works given unlimited memory but it sucks
    let p2: usize = machines
        .iter()
        .enumerate()
        .map(|(idx, m)| {
            println!("COUNT: {idx}");
            p2(m)
        })
        .sum();
    println!("P2: {p2}");
}

// P2

fn p2(m: &Machine) -> usize {
    let state: Vec<u16> = vec![0; m.joltages.len()];
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((state, 0));

    while let Some((state, count)) = queue.pop_front() {
        for wiring in m.wirings.iter() {
            let mut new_state = state.clone();
            click(*wiring, &m.joltages, &mut new_state);

            if visited.contains(&new_state) {
                continue;
            }

            match check(&m.joltages, &new_state) {
                Check::Correct => return count + 1,
                Check::CorrectWithMult(multi) => {
                    let new_state: Vec<u16> =
                        new_state.clone().iter_mut().map(|c| *c * multi).collect();
                    visited.insert(new_state.clone());
                    queue.push_back((new_state, (count + 1) * multi as usize));
                }
                Check::False => {
                    visited.insert(new_state.clone());
                    queue.push_back((new_state, count + 1));
                }
                Check::Impossible => {
                    continue;
                }
            }
        }
    }

    usize::MAX
}

enum Check {
    Correct,
    CorrectWithMult(u16),
    False,
    Impossible,
}

fn check(joltages: &[u16], state: &[u16]) -> Check {
    // Check if we gone above
    for (idx, count) in state.iter().enumerate() {
        if *count != 0 {
            let j = joltages.get(idx).unwrap();
            if count > j {
                return Check::Impossible;
            }
        }
    }

    // Check if we can multiply everything and fast-forward
    let mut i = *joltages.iter().min().unwrap();
    while i > 1 {
        let can_mult = joltages.iter().enumerate().all(|(idx, j)| {
            let s = state[idx];
            if *j != 0 && s == 0 {
                return false;
            }
            s * i <= *j
        });
        if can_mult {
            return Check::CorrectWithMult(i);
        }
        i -= 1;
    }

    // Check if any not true
    for (idx, jolt) in joltages.iter().enumerate() {
        if let Some(count) = state.get(idx)
            && count != jolt
        {
            return Check::False;
        }
    }

    Check::Correct
}

fn click(wiring: u16, joltages: &[u16], state: &mut [u16]) {
    let indexes = indexes_to_incr(wiring, joltages);
    for i in indexes {
        if let Some(count) = state.get_mut(i) {
            *count += 1;
        }
    }
}

fn indexes_to_incr(wiring: u16, joltages: &[u16]) -> Vec<usize> {
    joltages
        .iter()
        .enumerate()
        .filter_map(|(j, _)| {
            let mask: u16 = 1 << j;
            let is_set = mask & wiring != 0;
            if is_set { Some(j) } else { None }
        })
        .collect()
}
