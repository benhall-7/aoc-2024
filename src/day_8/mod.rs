use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Map {
    size: (usize, usize),
    nodes: HashMap<char, Vec<(usize, usize)>>,
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

    fn consec(a: usize, b: usize) -> isize {
        (2 * b as isize) - (a as isize)
    }

    fn collinear(a: (usize, usize), b: (usize, usize), c: (usize, usize)) -> bool {
        let vec_1 = ((b.0 as isize - a.0 as isize), (b.1 as isize - a.1 as isize));
        let vec_2 = ((c.0 as isize - a.0 as isize), (c.1 as isize - a.1 as isize));
        // check when magnitude of cross-product is 0
        vec_1.0 * vec_2.1 - vec_1.1 * vec_2.0 == 0
    }

    fn aligned(&self, coords: (usize, usize)) -> bool {
        self.nodes.iter().any(|(_, locs)| {
            locs.iter()
                .tuple_combinations()
                .any(|(loc_a, loc_b)| Self::collinear(*loc_a, *loc_b, coords))
        })
    }

    pub fn antinodes_1(&self) -> Vec<(usize, usize)> {
        self.nodes
            .iter()
            .flat_map(|(_, locs)| {
                locs.iter().tuple_combinations().flat_map(|(a, b)| {
                    [
                        (Self::consec(a.0, b.0), Self::consec(a.1, b.1)),
                        (Self::consec(b.0, a.0), Self::consec(b.1, a.1)),
                    ]
                    .into_iter()
                    .filter(|coords| self.in_bounds(*coords))
                    .map(|(y, x)| (y as usize, x as usize))
                })
            })
            .unique()
            .collect()
    }

    pub fn antinodes_2(&self) -> Vec<(usize, usize)> {
        (0..self.size.0)
            .cartesian_product(0..self.size.1)
            .filter(|coords| self.aligned(*coords))
            .collect()
    }
}

fn part_1() {
    let count = Map::new_from_file().antinodes_1().len();
    println!("{count}");
}

fn part_2() {
    let count = Map::new_from_file().antinodes_2().len();
    println!("{count}");
}

pub fn compute() {
    part_1();
    part_2();
}
