use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, PartialEq)]
enum Thing {
    Start,
    Air,
    Splitter,
    Beam,
}

#[derive(Clone, PartialEq)]
struct Manifold {
    shaft: Vec<Vec<Thing>>,
    spawn_point: (usize, usize),
    beams: Vec<(usize, usize)>,
    split_count: u32,
}

impl Manifold {
    fn new() -> Self {
        let f = File::open("./src/d7/input.txt").unwrap();
        let reader = BufReader::new(f);
        let mut spawn_point = (0, 0);

        let shaft = reader
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            spawn_point = (y + 1, x);
                            Thing::Start
                        }
                        '.' => Thing::Air,
                        '^' => Thing::Splitter,
                        _ => panic!("invalid input"),
                    })
                    .collect()
            })
            .collect();

        Self {
            shaft,
            spawn_point,
            beams: vec![],
            split_count: 0,
        }
    }

    fn spawn_beam(&mut self) {
        let (y, x) = self.spawn_point;
        self.beams.push((y, x));
        self.shaft[y][x] = Thing::Beam;
    }

    fn is_out_of_bounds(&self) -> bool {
        self.shaft.last().unwrap().contains(&Thing::Beam)
    }

    fn apply_gravity(&mut self) {
        let mut new_beams: Vec<(usize, usize)> = vec![];

        for beam in self.beams.iter() {
            let (y, x) = beam;
            self.shaft[*y][*x] = Thing::Air;

            let next_thing = self.shaft[y + 1][*x];
            match next_thing {
                Thing::Start => unreachable!("nope"),
                Thing::Beam => {}
                Thing::Air => {
                    self.shaft[y + 1][*x] = Thing::Beam;
                    new_beams.push((y + 1, *x));
                }
                Thing::Splitter => {
                    self.split_count += 1;
                    self.shaft[y + 1][x - 1] = Thing::Beam;
                    self.shaft[y + 1][x + 1] = Thing::Beam;
                    new_beams.push((y + 1, x - 1));
                    new_beams.push((y + 1, x + 1));
                }
            }
        }

        self.beams = new_beams;
    }

    pub fn count_worlds(&self) -> u128 {
        let max_y = self.shaft.len();
        let max_x = self.shaft[0].len();

        // Current state:
        // how many worlds are currently at a given position
        let mut ways = vec![vec![0u128; max_x]; max_y];

        let (sy, sx) = self.spawn_point;
        ways[sy][sx] = 1;

        let mut worlds_at_floor: u128 = 0;

        for y in sy..max_y {
            for x in 0..max_x {
                let count = ways[y][x];
                if count == 0 {
                    continue;
                }

                match self.shaft[y][x] {
                    Thing::Start | Thing::Air | Thing::Beam => {
                        if y + 1 < max_y {
                            ways[y + 1][x] += count;
                        } else {
                            // this branch is at bottom
                            worlds_at_floor += count;
                        }
                    }
                    Thing::Splitter => {
                        // Left branch
                        if y + 1 < max_y && x > 0 {
                            ways[y + 1][x - 1] += count;
                        } else {
                            // this branch is at bottom
                            worlds_at_floor += count;
                        }

                        // Right branch
                        if y + 1 < max_y && x + 1 < max_x {
                            ways[y + 1][x + 1] += count;
                        } else {
                            // this branch is at bottom
                            worlds_at_floor += count;
                        }
                    }
                }
            }
        }

        worlds_at_floor
    }
}

impl Display for Thing {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Thing::Start => 'S',
            Thing::Air => '.',
            Thing::Splitter => '^',
            Thing::Beam => '|',
        };

        write!(f, "{}", c)
    }
}

impl Display for Manifold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.shaft {
            for thing in row {
                write!(f, "{}", thing)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn solve() {
    // P1
    let mut manifold = Manifold::new();
    manifold.spawn_beam();

    while !manifold.is_out_of_bounds() {
        manifold.apply_gravity();
    }
    println!("Count: {}", manifold.split_count);

    // P2
    let manifold = Manifold::new();
    println!("Count: {}", manifold.count_worlds());
}
