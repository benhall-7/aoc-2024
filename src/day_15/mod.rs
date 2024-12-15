use itertools::Itertools;
use nalgebra::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Box,
    Wall,
}

#[derive(Debug, Clone)]
struct Map {
    pub grid: Vec<Vec<Tile>>,
    pub robot: Vector2<usize>,
}

impl Map {
    fn new_from_file() -> Self {
        let mut robot = None;
        let grid = include_str!("map.txt")
            .lines()
            .into_iter()
            .enumerate()
            .map(|(y, line)| {
                line.char_indices()
                    .map(|(x, chr)| match chr {
                        '#' => Tile::Wall,
                        'O' => Tile::Box,
                        '.' => Tile::Empty,
                        '@' => {
                            robot = Some(Vector2::new(x, y));
                            Tile::Empty
                        }
                        _ => panic!("unexpected char: {}", chr),
                    })
                    .collect()
            })
            .collect();
        Self {
            grid,
            robot: robot.unwrap(),
        }
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn get_tile(&self, pos: Vector2<usize>) -> Tile {
        self.grid[pos.y][pos.x]
    }

    pub fn get_tile_mut(&mut self, pos: Vector2<usize>) -> &mut Tile {
        &mut self.grid[pos.y][pos.x]
    }

    pub fn move_robot(&mut self, direction: usize) {
        let step: Vector2<isize> = [
            Vector2::new(0, -1),
            Vector2::new(1, 0),
            Vector2::new(0, 1),
            Vector2::new(-1, 0),
        ][direction];
        let next = (self.robot.map(|c| c as isize) + step).map(|c| c as usize);
        match self.get_tile(next) {
            Tile::Empty => self.robot = next,
            Tile::Box => {
                if let Some(empty_cell) = self.can_move(next, direction) {
                    *self.get_tile_mut(next) = Tile::Empty;
                    *self.get_tile_mut(empty_cell) = Tile::Box;
                    self.robot = next;
                }
            }
            Tile::Wall => {}
        }
    }

    pub fn can_move(&mut self, pos: Vector2<usize>, direction: usize) -> Option<Vector2<usize>> {
        let step: Vector2<isize> = [
            Vector2::new(0, -1),
            Vector2::new(1, 0),
            Vector2::new(0, 1),
            Vector2::new(-1, 0),
        ][direction];
        let next = (pos.map(|c| c as isize) + step).map(|c| c as usize);
        match self.get_tile(next) {
            Tile::Empty => Some(next),
            Tile::Wall => None,
            Tile::Box => self.can_move(next, direction),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile2 {
    Empty,
    BoxL,
    BoxR,
    Wall,
}

#[derive(Debug, Clone)]
struct Map2 {
    pub grid: Vec<Vec<Tile2>>,
    pub robot: Vector2<usize>,
}

impl Map2 {
    fn new_from_file() -> Self {
        let mut robot = None;
        let grid = include_str!("map.txt")
            .lines()
            .into_iter()
            .enumerate()
            .map(|(y, line)| {
                line.char_indices()
                    .flat_map(|(x, chr)| match chr {
                        '#' => [Tile2::Wall, Tile2::Wall],
                        'O' => [Tile2::BoxL, Tile2::BoxR],
                        '.' => [Tile2::Empty, Tile2::Empty],
                        '@' => {
                            robot = Some(Vector2::new(2 * x, y));
                            [Tile2::Empty, Tile2::Empty]
                        }
                        _ => panic!("unexpected char: {}", chr),
                    })
                    .collect()
            })
            .collect();
        Self {
            grid,
            robot: robot.unwrap(),
        }
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn get_tile(&self, pos: Vector2<usize>) -> Tile2 {
        self.grid[pos.y][pos.x]
    }

    pub fn get_tile_mut(&mut self, pos: Vector2<usize>) -> &mut Tile2 {
        &mut self.grid[pos.y][pos.x]
    }

    pub fn move_robot(&mut self, direction: usize) {
        let step: Vector2<isize> = [
            Vector2::new(0, -1),
            Vector2::new(1, 0),
            Vector2::new(0, 1),
            Vector2::new(-1, 0),
        ][direction];
        let next = (self.robot.map(|c| c as isize) + step).map(|c| c as usize);
        match self.get_tile(next) {
            Tile2::Empty => self.robot = next,
            Tile2::BoxL | Tile2::BoxR => {
                if self.can_box_move(next, direction) {
                    self.force_box_move(next, direction);
                    self.robot = next;
                }
            }
            Tile2::Wall => {}
        };
    }

    pub fn can_box_move(&mut self, pos: Vector2<usize>, direction: usize) -> bool {
        let step: Vector2<isize> = [
            Vector2::new(0, -1),
            Vector2::new(1, 0),
            Vector2::new(0, 1),
            Vector2::new(-1, 0),
        ][direction];
        if step.y != 0 {
            let (left, right) = match self.get_tile(pos) {
                Tile2::BoxL => (pos, pos + Vector2::new(1, 0)),
                Tile2::BoxR => (pos - Vector2::new(1, 0), pos),
                _ => panic!("box is missing a side at {:?}!", pos),
            };
            let next_l: Vector2<usize> = (left.map(|c| c as isize) + step).map(|c| c as usize);
            let next_r: Vector2<usize> = (right.map(|c| c as isize) + step).map(|c| c as usize);

            let can_move_l = match self.get_tile(next_l) {
                Tile2::Empty => true,
                Tile2::Wall => false,
                Tile2::BoxL | Tile2::BoxR => self.can_box_move(next_l, direction),
            };
            let can_move_r = match self.get_tile(next_r) {
                Tile2::Empty => true,
                Tile2::Wall => false,
                Tile2::BoxL => self.can_box_move(next_r, direction),
                // if a box is vertically aligned, we don't need to check it twice
                Tile2::BoxR => true,
            };

            can_move_l && can_move_r
        } else {
            // horizontal
            let next: Vector2<usize> = (pos.map(|c| c as isize) + step).map(|c| c as usize);
            match self.get_tile(next) {
                Tile2::Empty => true,
                Tile2::Wall => false,
                Tile2::BoxL | Tile2::BoxR => self.can_box_move(next, direction),
            }
        }
    }

    pub fn force_box_move(&mut self, pos: Vector2<usize>, direction: usize) {
        let step: Vector2<isize> = [
            Vector2::new(0, -1),
            Vector2::new(1, 0),
            Vector2::new(0, 1),
            Vector2::new(-1, 0),
        ][direction];
        if step.y != 0 {
            let (left, right) = match self.get_tile(pos) {
                Tile2::BoxL => (pos, pos + Vector2::new(1, 0)),
                Tile2::BoxR => (pos - Vector2::new(1, 0), pos),
                _ => panic!("box is missing a side at {:?}!", pos),
            };
            let next_l: Vector2<usize> = (left.map(|c| c as isize) + step).map(|c| c as usize);
            let next_r: Vector2<usize> = (right.map(|c| c as isize) + step).map(|c| c as usize);

            if let Tile2::BoxL | Tile2::BoxR = self.get_tile(next_l) {
                self.force_box_move(next_l, direction);
            }
            if let Tile2::BoxL = self.get_tile(next_r) {
                self.force_box_move(next_r, direction);
            }

            *self.get_tile_mut(next_l) = self.get_tile(left);
            *self.get_tile_mut(next_r) = self.get_tile(right);
            *self.get_tile_mut(right) = Tile2::Empty;
            *self.get_tile_mut(left) = Tile2::Empty;
        } else {
            // horizontal
            let next: Vector2<usize> = (pos.map(|c| c as isize) + step).map(|c| c as usize);
            match self.get_tile(next) {
                Tile2::BoxL | Tile2::BoxR => self.force_box_move(next, direction),
                _ => {}
            }
            *self.get_tile_mut(next) = self.get_tile(pos);
            *self.get_tile_mut(pos) = Tile2::Empty;
        }
    }
}

fn get_instructions() -> Vec<usize> {
    include_str!("instrs.txt")
        .lines()
        .flat_map(|line| line.chars())
        .map(|chr| match chr {
            '^' => 0,
            '>' => 1,
            'v' => 2,
            '<' => 3,
            _ => panic!("unexpected char"),
        })
        .collect()
}

fn part_1() {
    let mut map = Map::new_from_file();
    get_instructions()
        .into_iter()
        .for_each(|dir| map.move_robot(dir));
    let sum: usize = (0..map.width())
        .cartesian_product(0..map.height())
        .map(|(x, y)| {
            if let Tile::Box = map.get_tile(Vector2::new(x, y)) {
                y * 100 + x
            } else {
                0
            }
        })
        .sum();

    println!("{sum}");
}

fn part_2() {
    let mut map = Map2::new_from_file();
    get_instructions()
        .into_iter()
        .for_each(|dir| map.move_robot(dir));
    let sum: usize = (0..map.width())
        .cartesian_product(0..map.height())
        .map(|(x, y)| {
            if let Tile2::BoxL = map.get_tile(Vector2::new(x, y)) {
                y * 100 + x
            } else {
                0
            }
        })
        .sum();

    println!("{sum}");
}

pub fn compute() {
    part_1();
    part_2();
}
