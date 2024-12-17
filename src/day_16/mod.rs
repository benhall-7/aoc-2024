use std::{
    collections::{HashMap, HashSet},
    usize,
};

use nalgebra::Vector2;

#[derive(Debug, Clone)]
struct Map {
    walls: Vec<Vec<bool>>,
    start: Vector2<usize>,
    end: Vector2<usize>,
}

impl Map {
    pub fn new_from_file() -> Self {
        let input = include_str!("input.txt");
        let mut start = Vector2::default();
        let mut end = Vector2::default();
        let walls = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.char_indices()
                    .map(|(x, chr)| match chr {
                        '#' => true,
                        '.' => false,
                        'S' => {
                            start = Vector2::new(x, y);
                            false
                        }
                        'E' => {
                            end = Vector2::new(x, y);
                            false
                        }
                        _ => panic!("invalid map character"),
                    })
                    .collect()
            })
            .collect();
        Self { walls, start, end }
    }

    pub fn find_min_cost(
        &self,
        pos: Vector2<usize>,
        dir: usize,
        cost: usize,
        seen: &mut HashMap<Vector2<usize>, usize>,
    ) {
        seen.insert(pos, cost);

        if pos != self.end {
            [
                Vector2::new(0, -1),
                Vector2::new(1, 0),
                Vector2::new(0, 1),
                Vector2::new(-1, 0),
            ]
            .into_iter()
            .map(|step| (pos.map(|c| c as isize) + step).map(|c| c as usize))
            .enumerate()
            // can't travel through walls
            .filter(|(_, next)| !self.walls[next.y][next.x])
            .for_each(|(next_dir, next)| {
                let is_turn = next_dir != dir;
                let cost_before_move = cost + is_turn as usize * 1000;
                let next_cost = cost_before_move + 1;
                if next_cost < seen.get(&next).cloned().unwrap_or(usize::MAX) {
                    // if is_turn {
                    //     seen.insert(pos, cost_before_move);
                    // }
                    self.find_min_cost(next, next_dir, next_cost, seen)
                }
            })
        };
    }

    pub fn find_min_cost_2(
        &self,
        pos: Vector2<usize>,
        dir: usize,
        cost: usize,
        seen: &mut HashMap<(Vector2<usize>, bool), usize>,
    ) {
        seen.insert((pos, dir % 2 > 0), cost);

        if pos != self.end {
            [
                Vector2::new(0, -1),
                Vector2::new(1, 0),
                Vector2::new(0, 1),
                Vector2::new(-1, 0),
            ]
            .into_iter()
            .map(|step| (pos.map(|c| c as isize) + step).map(|c| c as usize))
            .enumerate()
            // can't travel through walls
            .filter(|(_, next)| !self.walls[next.y][next.x])
            .for_each(|(next_dir, next)| {
                let is_turn = next_dir != dir;
                let cost_before_move = cost + is_turn as usize * 1000;
                let next_cost = cost_before_move + 1;
                if next_cost
                    < seen
                        .get(&(next, next_dir % 2 > 0))
                        .cloned()
                        .unwrap_or(usize::MAX)
                {
                    self.find_min_cost_2(next, next_dir, next_cost, seen)
                }
            })
        };
    }

    pub fn get_path_after(
        &self,
        pos: Vector2<usize>,
        seen: &HashMap<Vector2<usize>, usize>,
        path: &mut HashSet<Vector2<usize>>,
    ) {
        path.insert(pos);
        let current_cost = seen[&pos];
        // println!("current: {:?}. cost: {}", pos, current_cost);

        [
            Vector2::new(0, -1),
            Vector2::new(1, 0),
            Vector2::new(0, 1),
            Vector2::new(-1, 0),
        ]
        .into_iter()
        .map(|step| (pos.map(|c| c as isize) + step).map(|c| c as usize))
        .filter(|next| seen.contains_key(next))
        .filter(|next| {
            // println!("neighbor cost: {}", seen[next]);
            seen[next] < current_cost
        })
        .for_each(|next| self.get_path_after(next, seen, path));
    }

    pub fn get_path_after_2(
        &self,
        pos: Vector2<usize>,
        dir: bool,
        seen: &HashMap<(Vector2<usize>, bool), usize>,
        path: &mut HashSet<Vector2<usize>>,
    ) {
        path.insert(pos);
        let current_cost = seen[&(pos, dir)];
        // println!("current: {:?}. cost: {}", pos, current_cost);

        [
            (Vector2::new(0, -1), false),
            (Vector2::new(1, 0), false),
            (Vector2::new(0, 1), false),
            (Vector2::new(-1, 0), false),
            (Vector2::new(0, -1), true),
            (Vector2::new(1, 0), true),
            (Vector2::new(0, 1), true),
            (Vector2::new(-1, 0), true),
        ]
        .into_iter()
        // convert offsets to addresses
        .map(|(step, dir)| ((pos.map(|c| c as isize) + step).map(|c| c as usize), dir))
        .filter(|next| seen.contains_key(next))
        .filter(|next| {
            // println!("neighbor cost: {}", seen[next]);
            seen[next] < current_cost
        })
        .for_each(|next| self.get_path_after_2(next.0, next.1, seen, path));
    }
}

pub fn compute() {
    let map = Map::new_from_file();
    let mut seen = HashMap::new();
    let mut path = HashSet::new();

    map.find_min_cost_2(map.start, 1, 0, &mut seen);

    let final_horiz = seen.get(&(map.end, true)).cloned().unwrap_or(usize::MAX);
    let final_vert = seen.get(&(map.end, false)).cloned().unwrap_or(usize::MAX);
    let final_cost = final_horiz.min(final_vert);
    let final_dir = final_horiz < final_vert;

    map.get_path_after_2(map.end, final_dir, &seen, &mut path);

    // print path after
    // map.walls.iter().enumerate().for_each(|(y, row)| {
    //     row.iter().enumerate().for_each(|(x, is_wall)| {
    //         if *is_wall {
    //             print!("#")
    //         } else if path.contains(&Vector2::new(x, y)) {
    //             print!("*");
    //         } else {
    //             print!(".");
    //         }
    //     });
    //     println!();
    // });

    println!("cost: {}", final_cost);
    println!("seats: {}", path.len());
}
