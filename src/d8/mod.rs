use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Deref, DerefMut},
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Junction {
    x: i64,
    y: i64,
    z: i64,
}

impl Junction {
    fn euclidean_distance(&self, other: &Junction) -> i64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        dx * dx + dy * dy + dz * dz // Note: This should be .sqrt() for actual euclidean_distance
    }
}

#[derive(Debug)]
struct CircuitVec(Vec<RefCell<Circuit>>);

impl CircuitVec {
    fn connect_junctions(&mut self, a: &Junction, b: &Junction) {
        let circuit_a = self
            .0
            .iter()
            .find(|circuit| circuit.borrow().junctions.contains(a));
        let circuit_b = self
            .0
            .iter()
            .find(|circuit| circuit.borrow().junctions.contains(b));

        match (circuit_a, circuit_b) {
            (None, None) => {
                let new_circuit = Circuit {
                    junctions: vec![*a, *b],
                };
                self.0.push(new_circuit.into());
            }
            (None, Some(circuit_b)) => {
                circuit_b.borrow_mut().push(*a);
            }
            (Some(circuit_a), None) => {
                circuit_a.borrow_mut().push(*b);
            }
            (Some(circuit_a), Some(circuit_b)) => {
                let circuit_b_idx = self
                    .0
                    .iter()
                    .position(|circuit| circuit.borrow().contains(b))
                    .unwrap();
                circuit_a
                    .borrow_mut()
                    .extend_from_slice(&circuit_b.borrow());
                self.0.swap_remove(circuit_b_idx);
            }
        }
    }
}

#[derive(Debug)]
struct Circuit {
    junctions: Vec<Junction>,
}

impl Deref for Circuit {
    type Target = Vec<Junction>;

    fn deref(&self) -> &Self::Target {
        &self.junctions
    }
}

impl DerefMut for Circuit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.junctions
    }
}

pub fn solve() {
    let f = File::open("./src/d8/input.txt").unwrap();
    let reader = BufReader::new(f);

    let junctions: Vec<_> = reader
        .lines()
        .map(|line| {
            let binding = line.unwrap();
            let mut parts = binding.splitn(3, ',');

            if let (Some(x), Some(y), Some(z)) = (parts.next(), parts.next(), parts.next()) {
                Junction {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                    z: z.parse().unwrap(),
                }
            } else {
                panic!("sum ting wong")
            }
        })
        .collect();

    // P1
    let mut circuits = CircuitVec(vec![]);

    for (a, b, _) in find_and_sort_pairs(&junctions).iter().take(1000) {
        let ac = circuits.0.iter().find(|c| c.borrow().contains(a));
        if let Some(ac) = ac
            && ac.borrow().contains(b)
        {
            continue;
        }
        circuits.connect_junctions(a, b);
    }

    circuits.0.sort_by_key(|c| c.borrow().len());

    let result: usize = circuits
        .0
        .iter()
        .rev()
        .take(3)
        .map(|v| v.borrow().len())
        .product();

    println!("P1: {}", result);

    // P2
    circuits = CircuitVec(vec![]);

    let binding = find_and_sort_pairs(&junctions);
    let mut pairs_iter = binding.iter();

    let result = loop {
        if let Some((a, b, _)) = pairs_iter.next() {
            let ac = circuits.0.iter().find(|c| c.borrow().contains(a));
            if let Some(ac) = ac
                && ac.borrow().contains(b)
            {
                continue;
            }
            circuits.connect_junctions(a, b);
            // break when the first and only circuit contains all junctions
            if circuits.0.first().unwrap().borrow().len() == junctions.len() {
                break a.x * b.x;
            }
        }
    };
    println!("P2: {result}");
}

fn find_and_sort_pairs(junctions: &[Junction]) -> Vec<(&Junction, &Junction, i64)> {
    let mut pairs = Vec::new();

    for i in 0..junctions.len() {
        let a = &junctions[i];

        (i + 1..junctions.len()).for_each(|j| {
            let b = &junctions[j];
            let d = a.euclidean_distance(b);

            pairs.push((a, b, d));
        });
    }

    pairs.sort_by_key(|(_, _, d)| *d);
    pairs
}
