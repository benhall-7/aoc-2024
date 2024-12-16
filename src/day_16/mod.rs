use std::collections::{HashMap, HashSet};

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
                    if is_turn {
                        seen.insert(pos, cost_before_move);
                    }
                    self.find_min_cost(next, next_dir, next_cost, seen)
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
        println!("current: {:?}. cost: {}", pos, current_cost);

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
            println!("neighbor cost: {}", seen[next]);
            seen[next] < current_cost
        })
        .for_each(|next| self.get_path_after(next, seen, path));
    }
}

pub fn compute() {
    let map = Map::new_from_file();
    let mut seen = HashMap::new();
    let mut path = HashSet::new();

    map.find_min_cost(map.start, 1, 0, &mut seen);
    map.get_path_after(map.end, &seen, &mut path);
    let final_cost = seen.get(&map.end).unwrap();

    println!("cost: {}", final_cost);
    println!("seats: {}", path.len());
}
