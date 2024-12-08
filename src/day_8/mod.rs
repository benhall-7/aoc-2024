use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Map {
    size: (usize, usize),
    nodes: HashMap<char, Vec<(usize, usize)>>,
}

fn consec(a: usize, b: usize) -> isize {
    (2 * b as isize) - (a as isize)
}

impl Map {
    pub fn new_from_file() -> Self {
        let input = include_str!("input.txt");
        let height = input.lines().count();
        let width = input.lines().nth(0).unwrap().chars().count();
        let nodes = input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut map, (y, line)| {
                line.chars().enumerate().for_each(|(x, chr)| {
                    if chr == '.' {
                        return;
                    }
                    let entry: &mut Vec<_> = map.entry(chr).or_default();
                    entry.push((y, x));
                });
                map
            });

        Self {
            size: (height, width),
            nodes,
        }
    }

    fn in_bounds(&self, coord: (isize, isize)) -> bool {
        coord.0 >= 0
            && coord.0 < self.size.0 as isize
            && coord.1 >= 0
            && coord.1 < self.size.1 as isize
    }

    fn in_line(a: (usize, usize), b: (usize, usize), c: (usize, usize)) -> bool {
        let vec_1 = ((b.0 as isize - a.0 as isize), (b.1 as isize - a.1 as isize));
        let vec_2 = ((c.0 as isize - a.0 as isize), (c.1 as isize - a.1 as isize));
        // parallel when magnitude of cross-product is 0
        vec_1.0 * vec_2.1 - vec_1.1 * vec_2.0 == 0
    }

    fn aligned(&self, coords: (usize, usize)) -> bool {
        self.nodes.iter().any(|(_, (locs))| {
            for i in 0..locs.len() {
                for j in 0..i {
                    let a = locs[i];
                    let b = locs[j];
                    if Self::in_line(a, b, coords) {
                        return true;
                    }
                }
            }
            false
        })
    }

    pub fn antinodes_1(&self) -> Vec<(usize, usize)> {
        self.nodes
            .iter()
            .fold(Vec::new(), |mut antinodes, (_, locs)| {
                for i in 0..locs.len() {
                    for j in 0..i {
                        let a = locs[i];
                        let b = locs[j];
                        let anti_1 = (consec(a.0, b.0), consec(a.1, b.1));
                        let anti_2 = (consec(b.0, a.0), consec(b.1, a.1));
                        if self.in_bounds(anti_1) {
                            antinodes.push((anti_1.0 as usize, anti_1.1 as usize));
                        }
                        if self.in_bounds(anti_2) {
                            antinodes.push((anti_2.0 as usize, anti_2.1 as usize));
                        }
                    }
                }
                antinodes
            })
    }

    pub fn antinodes_2(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                if self.aligned((y, x)) {
                    positions.push((y, x));
                }
            }
        }

        positions
    }
}

fn part_1() {
    let antis = Map::new_from_file()
        .antinodes_1()
        .iter()
        .map(|coord| {
            println!("coord: {:#?}", coord);
            *coord
        })
        .collect::<HashSet<_>>();

    println!("{}", antis.len());
}

fn part_2() {
    let count = Map::new_from_file().antinodes_2().len();

    println!("{count}");
}

pub fn compute() {
    part_1();
    part_2();
}
