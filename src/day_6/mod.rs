use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}

impl Direction {
    pub fn rotate_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Map {
    // returns None when the guard has left the map
    guard: Option<(usize, usize)>,
    guard_direction: Direction,
    visited: HashSet<(usize, usize)>,
    visited_direction: HashSet<(Direction, usize, usize)>,
    size: (usize, usize),
    obstacles: HashSet<(usize, usize)>,
}

#[derive(Debug, Clone, Copy)]
enum MapFinishCondition {
    Exit,
    Loop,
}

impl Map {
    pub fn new_from_file() -> Self {
        let input = include_str!("input.txt");
        let mut map = Map::default();
        let height = input.lines().count();
        let width = input.lines().nth(0).unwrap().chars().count();

        map.size = (height, width);
        input
            .lines()
            .enumerate()
            .fold(map, |mut map, (line_index, line)| {
                line.chars()
                    .enumerate()
                    .for_each(|(char_index, char)| match char {
                        '#' => {
                            map.obstacles.insert((line_index, char_index));
                        }
                        '^' => {
                            map.guard = Some((line_index, char_index));
                            map.guard_direction = Direction::Up;
                        }
                        _ => {}
                    });
                map
            })
    }

    /// Moves the guard one step, or rotates them.
    /// Returns true if in a loop, false otherwise.
    pub fn move_guard(&mut self) -> bool {
        // grid traversal, or object collision detection?
        // I'll do grid traversal, because that's more intuitive
        if let Some((y, x)) = self.guard {
            if self
                .visited_direction
                .contains(&(self.guard_direction, y, x))
            {
                return true;
            }
            let (next_y, next_x) = match self.guard_direction {
                Direction::Up => (y as isize - 1, x as isize),
                Direction::Right => (y as isize, x as isize + 1),
                Direction::Down => (y as isize + 1, x as isize),
                Direction::Left => (y as isize, x as isize - 1),
            };

            self.visited.insert((y, x));
            self.visited_direction.insert((self.guard_direction, y, x));

            // check out of bounds move
            if next_y < 0
                || next_y >= self.size.0 as isize
                || next_x < 0
                || next_x >= self.size.1 as isize
            {
                self.guard = None;
                return false;
            }

            if self.obstacles.contains(&(next_y as usize, next_x as usize)) {
                self.guard_direction = self.guard_direction.rotate_right();
            } else {
                self.guard = Some((next_y as usize, next_x as usize));
            }
        }
        false
    }

    pub fn run(&mut self) -> MapFinishCondition {
        loop {
            if self.guard == None {
                return MapFinishCondition::Exit;
            }
            if self.move_guard() {
                return MapFinishCondition::Loop;
            }
        }
    }
}

fn part_1() {
    let mut map = Map::new_from_file();
    while map.guard.is_some() {
        map.move_guard();
    }
    println!("{}", map.visited.len());
}

fn part_2() {
    // poorly optimized solution
    let clonable = Map::new_from_file();
    let mut map = clonable.clone();
    let guard_start_position = map.guard.unwrap();
    map.run();

    let mut looping_obstacles = 0;

    for visited in map.visited {
        if visited == guard_start_position {
            continue;
        }
        let mut edited_map = clonable.clone();
        edited_map.obstacles.insert(visited);
        if let MapFinishCondition::Loop = edited_map.run() {
            looping_obstacles += 1;
        }
    }

    println!("{}", looping_obstacles);
}

pub fn compute() {
    part_1();
    part_2();
}
