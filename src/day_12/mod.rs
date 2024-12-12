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
        // start anywhere on one of the left-most points
        let min_x = self.coords.iter().map(|coord| coord.1).min().unwrap();
        let start = self
            .coords
            .iter()
            .filter(|coord| coord.1 == min_x)
            .nth(0)
            .unwrap();

        // begin by facing upward
        let mut seen: HashSet<Machine> = HashSet::new();
        let mut steps = vec![];
        let mut machine = Machine {
            coord: (*start),
            direction: 0,
        };

        while !seen.contains(&machine) {
            seen.insert(machine);
            steps.push(machine);

            let neighbors = self.get_8_neighbors(machine.coord);
            // in front
            if neighbors[1 + machine.direction * 2].present {
                // front-left adjacent
                if neighbors[machine.direction * 2].present {
                    // move to the front-left adjacent position
                    machine.coord = neighbors[machine.direction * 2].coord;
                    // turn left 1/4 rotation
                    machine.direction = (machine.direction + 3) % 4;
                } else {
                    // move forward
                    machine.coord = neighbors[1 + machine.direction * 2].coord;
                }
            } else {
                // turn right 1/4 rotation
                machine.direction = (machine.direction + 1) % 4;
            }
        }

        // stitch incoming
        let stitch = (steps[0].direction != steps[steps.len() - 1].direction) as usize;
        let count = steps
            .into_iter()
            .tuple_windows()
            .filter(|(prev, curr)| prev.direction != curr.direction)
            .count()
            + stitch;

        println!("shape sides = {count}");
        count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Neighbor {
    coord: (usize, usize),
    present: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Machine {
    pub coord: (usize, usize),
    pub direction: usize,
}

pub fn part_1() {
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

pub fn part_2() {
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
