use itertools::Itertools;

pub fn get_map() -> Vec<Vec<u8>> {
    let input = include_str!("input.txt");
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| u8::from_str_radix(&chr.to_string(), 10).expect("expected digit"))
                .collect()
        })
        .collect()
}

struct Map {
    data: Vec<Vec<u8>>,
}

impl Map {
    pub fn new_from_file() -> Self {
        Self { data: get_map() }
    }

    pub fn get(&self, coord: (usize, usize)) -> u8 {
        self.data[coord.0][coord.1]
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn trailheads(&self) -> Vec<(usize, usize)> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(row_num, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, cell)| **cell == 0)
                    .map(move |(col, _)| (row_num, col))
            })
            .collect()
    }

    pub fn dfs(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        let val = self.get(coord);
        if val == 9 {
            // println!("{:#?}", coord);
            return vec![coord];
        }
        let map_height = self.height();
        let map_width = self.width();
        let valid_checks = [
            (coord.0 > 0).then(|| (coord.0 - 1, coord.1)),
            (coord.1 > 0).then(|| (coord.0, coord.1 - 1)),
            (coord.0 < map_height - 1).then(|| (coord.0 + 1, coord.1)),
            (coord.1 < map_width - 1).then(|| (coord.0, coord.1 + 1)),
        ];
        // a lot of collecting vecs going on
        valid_checks
            .into_iter()
            .flat_map(|check| {
                check
                    .filter(|next| self.get(*next) == val + 1)
                    .map(|coord| self.dfs(coord))
                    .unwrap_or_default()
            })
            .collect()
    }
}

pub fn part_1() {
    let map = Map::new_from_file();
    let sum: usize = map
        .trailheads()
        .clone()
        .iter()
        .map(|&trailhead| {
            // println!("trailhead: {:#?}", trailhead);
            map.dfs(trailhead).into_iter().unique().count()
        })
        .sum();

    println!("{sum}");
}

pub fn part_2() {
    let map = Map::new_from_file();
    let sum: usize = map
        .trailheads()
        .clone()
        .iter()
        .map(|&trailhead| {
            // println!("trailhead: {:#?}", trailhead);
            map.dfs(trailhead).len()
        })
        .sum();

    println!("{sum}");
}

pub fn compute() {
    part_1();
    part_2();
}
