use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

use nalgebra::Vector2;

struct Map {
    walls: HashSet<Vector2<isize>>,
    distances: HashMap<Vector2<isize>, usize>,
    start: Vector2<isize>,
}

impl Map {
    fn get_from_file() -> Self {
        let input = include_str!("input.txt");
        let mut walls = HashSet::new();
        let mut distances = HashMap::new();

        let mut start = Vector2::default();
        input.lines().enumerate().for_each(|(y, line)| {
            line.char_indices().for_each(|(x, chr)| match chr {
                '#' => {
                    walls.insert(Vector2::new(x as isize, y as isize));
                }
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
        let mut map = Self {
            walls,
            distances,
            start,
        };
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
}

fn part_1() {
    let map = Map::get_from_file();
    let shortcut_amounts = map
        .distances
        .iter()
        .flat_map(|(pos, dist)| {
            [
                Vector2::new(0, -2),
                Vector2::new(2, 0),
                Vector2::new(0, 2),
                Vector2::new(-2, 0),
            ]
            .iter()
            .map(|offset| pos + offset)
            .filter_map(|nearby| map.distances.get(&nearby).cloned())
            .filter_map(|nearby_dist| {
                let saved = nearby_dist.saturating_sub(*dist).saturating_sub(2);
                if saved > 0 {
                    Some(saved)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!(
        "number of shortcuts saving >= 100: {}",
        shortcut_amounts
            .into_iter()
            .filter(|amount| *amount >= 100)
            .count()
    )
}

fn shortcut_count(
    start: Vector2<isize>,
    dist: usize,
    max_cheat: usize,
    min_save: usize,
    map: &Map,
) -> usize {
    let mut shortcuts = 0;
    let mut queue = VecDeque::from([(start, 0)]);
    let mut seen = HashSet::from([start]);
    while let Some((current, traversed)) = queue.pop_front() {
        for offset in [
            Vector2::new(0, -1),
            Vector2::new(1, 0),
            Vector2::new(0, 1),
            Vector2::new(-1, 0),
        ] {
            let next = current + offset;
            if seen.contains(&next) {
                continue;
            }
            if map.walls.contains(&next) && traversed + 1 < max_cheat {
                queue.push_back((next, traversed + 1));
                seen.insert(next);
            } else if let Some(next_dist) = map.distances.get(&next).cloned() {
                if next_dist >= dist + traversed + 1 + min_save {
                    let saves = next_dist - (dist + traversed + 1);
                    println!(
                        "shortcut! from: {:?}; to: {:?}; saves: {saves}",
                        start, next
                    );
                    shortcuts += 1;
                    seen.insert(next);
                }
            }
        }
    }

    shortcuts
}

fn part_2() {
    let map = Map::get_from_file();
    let max_cheat = 20;
    let min_save = 72;

    let shortcut_amounts: usize = map
        .distances
        .iter()
        .map(|(pos, dist)| {
            let count = shortcut_count(*pos, *dist, max_cheat, min_save, &map);
            // println!("good cheats for position {:?} = {count}", pos);
            count
        })
        .sum();
    println!(
        "number of new shortcuts saving >= {min_save}: {}",
        shortcut_amounts
    )
}

pub fn compute() {
    // part_1();
    part_2();
}
