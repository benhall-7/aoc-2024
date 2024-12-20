use std::collections::HashSet;

use itertools::Itertools;

fn get_map() -> Vec<Vec<char>> {
    let input = include_str!("input.txt");
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug, Clone)]
struct Map {
    data: Vec<Vec<char>>,
}

#[derive(Debug, Clone)]
struct Region {
    coords: HashSet<(usize, usize)>,
    perimeter: usize,
}

impl Map {
    pub fn new_from_file() -> Self {
        Self { data: get_map() }
    }

    pub fn get(&self, coord: (usize, usize)) -> char {
        self.data[coord.0][coord.1]
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn get_neighbors(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        let height = self.height();
        let width = self.width();
        let value = self.get(coord);
        [
            (coord.0 > 0).then(|| (coord.0 - 1, coord.1)),
            (coord.1 > 0).then(|| (coord.0, coord.1 - 1)),
            (coord.0 < height - 1).then(|| (coord.0 + 1, coord.1)),
            (coord.1 < width - 1).then(|| (coord.0, coord.1 + 1)),
        ]
        .into_iter()
        .filter_map(|opt| opt)
        .filter(|neigh| self.get(*neigh) == value)
        .collect()
    }

    pub fn get_region_from_point(&self, coord: (usize, usize)) -> Region {
        let mut region = Region::empty();
        let mut stack = vec![coord];
        let mut seen = HashSet::new();

        while let Some(current) = stack.pop() {
            if seen.contains(&current) {
                continue;
            }
            seen.insert(current);
            region.push(current);

            let mut neighbors = self.get_neighbors(current);
            region.perimeter += 4 - neighbors.len();
            stack.append(&mut neighbors);
        }

        region
    }
}

impl Region {
    pub fn empty() -> Self {
        Self {
            coords: HashSet::new(),
            perimeter: 0,
        }
    }

    pub fn push(&mut self, coord: (usize, usize)) {
        self.coords.insert(coord);
    }

    pub fn contains(&self, coord: (usize, usize)) -> bool {
        self.coords.contains(&coord)
    }

    pub fn area(&self) -> usize {
        self.coords.len()
    }

    fn get_4_neighbors(&self, coord: (usize, usize)) -> [Neighbor; 4] {
        [
            (coord.0.wrapping_sub(1), coord.1),
            (coord.0, coord.1 + 1),
            (coord.0 + 1, coord.1),
            (coord.0, coord.1.wrapping_sub(1)),
        ]
        .map(|(y, x)| Neighbor {
            coord: (y, x),
            present: self.coords.contains(&(y, x)),
        })
    }

    fn get_8_neighbors(&self, coord: (usize, usize)) -> [Neighbor; 8] {
        [
            (coord.0.wrapping_sub(1), coord.1.wrapping_sub(1)),
            (coord.0.wrapping_sub(1), coord.1),
            (coord.0.wrapping_sub(1), coord.1 + 1),
            (coord.0, coord.1 + 1),
            (coord.0 + 1, coord.1 + 1),
            (coord.0 + 1, coord.1),
            (coord.0 + 1, coord.1.wrapping_sub(1)),
            (coord.0, coord.1.wrapping_sub(1)),
        ]
        .map(|(y, x)| Neighbor {
            coord: (y, x),
            present: self.coords.contains(&(y, x)),
        })
    }

    pub fn num_sides(&self) -> usize {
        // number of vertexes = number of edges
        // thanks Euler!
        self.coords
            .iter()
            .map(|coord| {
                let neighbors_8 = self.get_8_neighbors(*coord);
                let concave_vertices = (0..4)
                    .into_iter()
                    .filter(|&dir| {
                        // condition for a concave vertex to be made
                        let start = dir * 2 + 1;
                        let between = (start + 1) % 8;
                        let end = (between + 1) % 8;
                        neighbors_8[start].present
                            && !neighbors_8[between].present
                            && neighbors_8[end].present
                    })
                    .count();

                let neighbors_4 = self.get_4_neighbors(*coord);
                let valid_neighbors = neighbors_4
                    .into_iter()
                    .enumerate()
                    .filter(|(_, neighbor)| neighbor.present)
                    .map(|(ind, _)| ind)
                    .collect::<Vec<_>>();
                let convex_vertices = match valid_neighbors.len() {
                    0 => 4,
                    1 => 2,
                    2 => {
                        // if the two neighbors are not opposite each other
                        // there's a single vertex
                        // otherwise, no vertex
                        ((valid_neighbors[1] - valid_neighbors[0]) != 2) as usize
                    }
                    _ => 0,
                };
                concave_vertices + convex_vertices
            })
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Neighbor {
    coord: (usize, usize),
    present: bool,
}

fn part_1() {
    let map = Map::new_from_file();
    let cost: usize = (0..map.height())
        .cartesian_product(0..map.width())
        .fold(vec![], |mut regions: Vec<Region>, coord| {
            let contained = regions.iter().any(|region| region.contains(coord));
            if !contained {
                regions.push(map.get_region_from_point(coord));
            }
            regions
        })
        .into_iter()
        .map(|region| region.area() * region.perimeter)
        .sum();
    println!("1: {cost}");
}

fn part_2() {
    let map = Map::new_from_file();
    let cost: usize = (0..map.height())
        .cartesian_product(0..map.width())
        .fold(vec![], |mut regions: Vec<Region>, coord| {
            let contained = regions.iter().any(|region| region.contains(coord));
            if !contained {
                regions.push(map.get_region_from_point(coord));
            }
            regions
        })
        .into_iter()
        .map(|region| region.area() * region.num_sides())
        .sum();
    println!("2: {cost}");
}

pub fn compute() {
    part_1();
    part_2();
}
