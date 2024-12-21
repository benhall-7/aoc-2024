use std::{
    collections::{HashMap, VecDeque},
    usize,
};

use itertools::Itertools;
use nalgebra::Vector2;

struct Map {
    distances: HashMap<Vector2<isize>, usize>,
    start: Vector2<isize>,
}

impl Map {
    fn get_from_file() -> Self {
        let input = include_str!("input.txt");
        let mut distances = HashMap::new();

        let mut start = Vector2::default();
        input.lines().enumerate().for_each(|(y, line)| {
            line.char_indices().for_each(|(x, chr)| match chr {
                '#' => {}
                '.' => {
                    distances.insert(Vector2::new(x as isize, y as isize), usize::MAX);
                }
                'S' => {
                    start = Vector2::new(x as isize, y as isize);
                    distances.insert(Vector2::new(x as isize, y as isize), usize::MAX);
                }
                'E' => {
                    // end = Vector2::new(x as isize, y as isize);
                    distances.insert(Vector2::new(x as isize, y as isize), usize::MAX);
                }
                _ => panic!("invalid map cell"),
            })
        });
        let mut map = Self { distances, start };
        map.set_distances();
        map
    }

    fn set_distances(&mut self) {
        let start = self.start;

        let mut queue = VecDeque::from([(start, 0)]);
        while let Some((space, distance)) = queue.pop_front() {
            match self.distances.get(&space) {
                Some(next_distance) => {
                    if *next_distance < distance {
                        continue;
                    }
                }
                None => continue,
            }
            self.distances.insert(space, distance);
            queue.extend(
                [
                    Vector2::new(0, -1),
                    Vector2::new(1, 0),
                    Vector2::new(0, 1),
                    Vector2::new(-1, 0),
                ]
                .map(|disp| space + disp)
                .map(|neigh| (neigh, distance + 1)),
            );
        }
    }

    fn shortcut_lengths(
        &self,
        pos: Vector2<isize>,
        dist: usize,
        max_cheat: usize,
        min_save: usize,
    ) -> Vec<usize> {
        let max = max_cheat as isize;
        let cheat_range = (-max..=max)
            .cartesian_product(-max..=max)
            .filter(|(x, y)| x.abs() + y.abs() <= max);

        cheat_range
            .map(|(y, x)| Vector2::new(x, y))
            .map(|offset| pos + offset)
            .filter_map(|other| {
                self.distances
                    .get(&other)
                    .cloned()
                    .map(|dist| (dist, other))
            })
            .filter_map(|(other_dist, other)| {
                let abs_distance = other_dist as isize - dist as isize;
                let euclid = (other - pos).abs().sum();
                let saved = abs_distance - euclid;
                (saved >= min_save as isize).then(|| saved as usize)
            })
            .collect()
    }
}

fn part_1() {
    let map = Map::get_from_file();
    let max_cheat = 2;
    let min_save = 100;

    let shortcuts = map
        .distances
        .iter()
        .flat_map(|(pos, dist)| map.shortcut_lengths(*pos, *dist, max_cheat, min_save))
        .count();

    println!("number of shortcuts saving >= {min_save}: {}", shortcuts)
}

fn part_2() {
    let map = Map::get_from_file();
    let max_cheat = 20;
    let min_save = 100;

    let shortcuts = map
        .distances
        .iter()
        .flat_map(|(pos, dist)| map.shortcut_lengths(*pos, *dist, max_cheat, min_save))
        .count();
    // .counts()
    // .into_iter()
    // .sorted()
    // .collect();

    println!(
        "number of new shortcuts saving >= {min_save}: {}",
        shortcuts
    )
}

pub fn compute() {
    part_1();
    part_2();
}
