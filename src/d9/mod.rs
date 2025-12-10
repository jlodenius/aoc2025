use std::borrow::Cow;
use std::collections::HashMap;
use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, Copy, Hash)]
enum Color {
    Red,
    Green,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct Tile<'a> {
    color: Color,
    point: Cow<'a, Point>,
}

#[derive(Debug, Clone, Default)]
struct Tiles<'a> {
    tiles: Vec<Tile<'a>>,
}

impl Display for Tiles<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "<empty Tiles>");
        }

        let pad = 0;
        let max_x = self.iter().map(|t| t.point.x).max().unwrap() + pad;
        let max_y = self.iter().map(|t| t.point.y).max().unwrap() + pad;

        let mut map = HashMap::new();
        for tile in self.iter() {
            map.insert((tile.point.x, tile.point.y), tile.color);
        }

        for y in 0..=max_y {
            for x in 0..=max_x {
                match map.get(&(x, y)) {
                    Some(Color::Red) => write!(f, "#")?,
                    Some(Color::Green) => write!(f, "X")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<'a> Deref for Tiles<'a> {
    type Target = Vec<Tile<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl<'a> DerefMut for Tiles<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tiles
    }
}

impl Point {
    fn square_area(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x).abs() + 1;
        let dy = (self.y - other.y).abs() + 1;
        dx * dy
    }
}

impl<'a> Tiles<'a> {
    fn point_in_polygon(&self, point: &Point) -> bool {
        fn point_on_segment(p: &Point, a: &Point, b: &Point) -> bool {
            let cross = (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);

            if cross != 0 {
                return false;
            }

            let min_x = a.x.min(b.x);
            let max_x = a.x.max(b.x);
            let min_y = a.y.min(b.y);
            let max_y = a.y.max(b.y);

            p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
        }

        let n = self.len();
        if n < 3 {
            return false;
        }

        let mut inside = false;
        let mut j = n - 1;

        for i in 0..n {
            let pi = &self[i].point;
            let pj = &self[j].point;

            // treat edge as inside
            if point_on_segment(point, pi, pj) {
                return true;
            }

            // ray-casting
            let intersects = (pi.y > point.y) != (pj.y > point.y);
            if intersects {
                let x_intersect = pi.x as f64
                    + (point.y as f64 - pi.y as f64) * (pj.x as f64 - pi.x as f64)
                        / (pj.y as f64 - pi.y as f64);

                if (point.x as f64) < x_intersect {
                    inside = !inside;
                }
            }

            j = i;
        }

        inside
    }

    fn fill_between(&mut self, a: &'a Point, b: &'a Point) {
        if a.x == b.x {
            for j in (a.y.min(b.y) + 1)..a.y.max(b.y) {
                self.push(Tile {
                    color: Color::Green,
                    point: Cow::Owned((a.x, j).into()),
                });
            }
        } else {
            for j in (a.x.min(b.x) + 1)..a.x.max(b.x) {
                self.push(Tile {
                    color: Color::Green,
                    point: Cow::Owned((j, a.y).into()),
                });
            }
        };
    }
}

pub fn solve() {
    let f = File::open("./src/d9/input.txt").unwrap();
    let reader = BufReader::new(f);

    let points: Vec<Point> = reader
        .lines()
        .map(|line| {
            let binding = line.unwrap();
            let mut line = binding.split(',');

            let x: i64 = line.next().unwrap().parse().unwrap();
            let y: i64 = line.next().unwrap().parse().unwrap();

            (x, y).into()
        })
        .collect();

    // P1
    let mut biggest: i64 = 0;
    for i in 0..points.len() {
        let a = points.get(i).unwrap();
        for j in i + 1..points.len() {
            let b = points.get(j).unwrap();
            let area = a.square_area(b);
            if area > biggest {
                biggest = area
            }
        }
    }
    println!("P1: {biggest}");

    // P2
    let mut tiles = Tiles::default();
    for i in 0..points.len() {
        let point = points.get(i).unwrap();
        tiles.push(Tile {
            color: Color::Red,
            point: Cow::Borrowed(point),
        });
        // if let Some(next) = points.get(i + 1) {
        //     tiles.fill_between(point, next);
        // } else {
        //     // Connect the first and the last
        //     tiles.fill_between(points.first().unwrap(), points.last().unwrap());
        // }
    }
    // println!("{}", tiles);

    // 1. sort all pairs of coords in order of biggest area
    let mut pairs = vec![];

    for i in 0..points.len() {
        let a = points.get(i).unwrap();
        for j in i + 1..points.len() {
            let b = points.get(j).unwrap();
            let area = a.square_area(b);
            pairs.push((a, b, area));
        }
    }
    pairs.sort_by_key(|(_, _, area)| *area);

    // 2. find the first one that has all points inside the polygon
    let biggest = pairs
        .iter()
        .rev()
        .find(|(a, b, _)| {
            let min_x = a.x.min(b.x);
            let max_x = a.x.max(b.x);

            let min_y = a.y.min(b.y);
            let max_y = a.y.max(b.y);

            // check each corner
            let corners: &[Point] = &[
                (min_x, min_y).into(),
                (min_x, max_y).into(),
                (max_x, min_y).into(),
                (max_x, max_y).into(),
            ];
            for corner in corners.iter() {
                if !tiles.point_in_polygon(corner) {
                    return false;
                }
            }
            // check bottom/top edges
            for x in min_x..=max_x {
                if !tiles.point_in_polygon(&(x, min_y).into()) {
                    return false;
                }
                if !tiles.point_in_polygon(&(x, max_y).into()) {
                    return false;
                }
            }
            // check left/right edges
            for y in min_y..=max_y {
                if !tiles.point_in_polygon(&(min_x, y).into()) {
                    return false;
                }
                if !tiles.point_in_polygon(&(max_x, y).into()) {
                    return false;
                }
            }

            true
        })
        .unwrap();

    println!("P2: {:?}", biggest.2);
}
