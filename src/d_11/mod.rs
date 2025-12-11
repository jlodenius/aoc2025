use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

// Apparently there are no cycles so this is unnecessary..
fn _has_duplicates(list: &[&str]) -> bool {
    let mut seen = HashSet::new();
    for item in list {
        if !seen.insert(*item) {
            return true;
        }
    }
    false
}

#[derive(Debug)]
struct Link<'a> {
    name: &'a str,
    next: Vec<&'a str>,
}

#[derive(Debug, Default)]
struct LinkMap<'a> {
    index: HashMap<&'a str, Link<'a>>,
}

impl<'a> LinkMap<'a> {
    fn add_link(&mut self, link: Link<'a>) {
        self.insert(link.name, link);
    }
    fn paths_to_out(&self, nodes: &[&str]) -> usize {
        let last_node = nodes.last().unwrap();
        if let Some(last_node) = &self.get(last_node) {
            last_node
                .next
                .iter()
                .map(|node| self.paths_to_out(&[last_node.name, node]))
                .sum()
        } else if *last_node == "out" {
            1
        } else {
            panic!("sum ting wong");
        }
    }
    fn paths_to_out_through_dac_fft(&self, nodes: &[&'a str]) -> usize {
        let last = nodes.last().unwrap();

        if *last == "out" {
            if nodes.contains(&"dac") && nodes.contains(&"fft") {
                return 1;
            } else {
                return 0;
            }
        }

        let node = match self.get(last) {
            Some(n) => n,
            None => {
                panic!("sum ting wong")
            }
        };

        let sum: usize = node
            .next
            .iter()
            .map(|next| {
                let mut next_nodes = nodes.to_vec();
                next_nodes.push(next);
                self.paths_to_out_through_dac_fft(&next_nodes)
            })
            .sum();

        sum
    }
}

impl<'a> Deref for LinkMap<'a> {
    type Target = HashMap<&'a str, Link<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.index
    }
}

impl<'a> DerefMut for LinkMap<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.index
    }
}

pub fn solve() {
    let input = include_str!("./input.txt");

    let mut list = LinkMap::default();

    for line in input.split('\n') {
        let mut split = line.split(':').take(2);

        while let (Some(name), Some(links)) = (split.next(), split.next()) {
            let name = name.trim();
            let next = links.trim().split(' ').collect();
            list.add_link(Link { name, next });
        }
    }

    // P1
    let count = list.paths_to_out(&["you"]);
    println!("P1: {count}");

    // P2
    let count = list.paths_to_out_through_dac_fft(&["svr"]);
    println!("P2: {count}");
}
