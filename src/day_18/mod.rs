use std::collections::{HashMap, HashSet, VecDeque};

use image::{ImageBuffer, Rgb};
use itertools::Itertools;
use nalgebra::Vector2;

struct Map {
    corrupted_set: HashSet<Vector2<isize>>,
    corruptions: Vec<Vector2<isize>>,
}

impl Map {
    pub fn from_file() -> Self {
        Self {
            corrupted_set: HashSet::new(),
            corruptions: include_str!("input.txt")
                .lines()
                .map(|line| {
                    let mut splits = line.split(",");
                    let (x, y) = splits.next_tuple().unwrap();
                    Vector2::new(
                        isize::from_str_radix(x, 10).unwrap(),
                        isize::from_str_radix(y, 10).unwrap(),
                    )
                })
                .collect(),
        }
    }

    pub fn set_n_corruptions(&mut self, n: usize) {
        self.corrupted_set.clear();
        self.corruptions.iter().take(n).for_each(|cor| {
            self.corrupted_set.insert(*cor);
        });
    }

    pub fn find_path(&self) -> Option<isize> {
        let start = Vector2::new(0, 0);
        let mut distances = HashMap::new();
        let mut stack = VecDeque::from([(start, 0)]);
        while let Some((space, distance)) = stack.pop_front() {
            if distances.contains_key(&space) {
                continue;
            }
            distances.insert(space, distance);
            // insert neighbors where the neighbors:
            // - distances can be set or reduced
            // - are not in the corrupted set
            stack.extend(
                [
                    Vector2::new(0, -1),
                    Vector2::new(1, 0),
                    Vector2::new(0, 1),
                    Vector2::new(-1, 0),
                ]
                .map(|disp| space + disp)
                .into_iter()
                .filter(|neigh| {
                    neigh.x >= 0
                        && neigh.x < 71
                        && neigh.y >= 0
                        && neigh.y < 71
                        && !self.corrupted_set.contains(neigh)
                        && (distance + 1) < *distances.get(neigh).unwrap_or(&isize::MAX)
                })
                .map(|neigh| (neigh, distance + 1)),
            );
        }
        distances.get(&Vector2::new(70, 70)).cloned()
    }

    #[allow(dead_code)]
    pub fn debug(&self, image_name: &str) {
        let mut image = ImageBuffer::new(71, 71);
        (0..=70).for_each(|y| {
            (0..=70).for_each(|x| {
                match self.corrupted_set.contains(&Vector2::new(x, y)) {
                    true => image.put_pixel(x as u32, y as u32, Rgb([255u8, 255u8, 255u8])),
                    false => {}
                };
            });
        });
        image.save(image_name).unwrap();
    }
}

fn part_1() {
    let mut map = Map::from_file();
    map.set_n_corruptions(1024);
    let distance = map.find_path();
    println!("min distance: {:?}", distance);
}

fn part_2() {
    let mut map = Map::from_file();
    // don't need to start at 0, the last part already had 1024 corruptions
    let unsolvable_count = (1025..=map.corruptions.len()).find(|&num_corruptions| {
        map.set_n_corruptions(num_corruptions);
        let distance = map.find_path();
        distance.is_none()
    });
    println!(
        "first impossible state index: {:?}",
        unsolvable_count.map(|n| n - 1)
    );
    println!(
        "first impossible corruption: {:?}",
        unsolvable_count.map(|index| map.corruptions[index - 1])
    );

    // map.set_n_corruptions(none_index.unwrap() - 1);
    // map.debug("SOLVABLE.png");
    // map.set_n_corruptions(none_index.unwrap());
    // map.debug("UNSOLVABLE.png");
}

pub fn compute() {
    part_1();
    part_2();
}
