fn get_grid() -> Vec<Vec<char>> {
    let input = include_str!("input.txt");
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

// fn get_test_grid() -> Vec<Vec<char>> {
//     let input = include_str!("test_input.txt");
//     input
//         .lines()
//         .map(|line| line.chars().collect::<Vec<_>>())
//         .collect::<Vec<_>>()
// }

fn check(
    x_dir: isize,
    y_dir: isize,
    x: isize,
    y: isize,
    grid: &Vec<Vec<char>>,
    word: &str,
) -> bool {
    if x_dir.abs() > 1 || y_dir.abs() > 1 {
        return false;
    }
    if x_dir == 0 && y_dir == 0 {
        return false;
    }

    let grid_width = grid[0].len() as isize;
    let grid_height = grid.len() as isize;

    // prevent out of bounds for X and Y directions
    let end_x: isize = x + x_dir * (word.len() as isize - 1);
    let end_y: isize = y + y_dir * (word.len() as isize - 1);
    if end_x < 0 || end_x >= grid_width {
        return false;
    }
    if end_y < 0 || end_y >= grid_height {
        return false;
    }

    word.chars().enumerate().all(|(ind, chr)| {
        grid[(y + (y_dir * ind as isize)) as usize][(x + (x_dir * ind as isize)) as usize] == chr
    })
}

fn part_1() {
    // assume grid is rectangular
    let grid = get_grid();

    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            for y_dir in -1..=1_isize {
                for x_dir in -1..=1_isize {
                    if check(x_dir, y_dir, x as isize, y as isize, &grid, "XMAS") {
                        count += 1
                    }
                }
            }
        }
    }
    println!("{count}");
}

fn get_arrangements() -> Vec<Vec<(char, usize, usize)>> {
    let base = vec![
        ('M', 0, 0),
        ('M', 0, 2),
        ('A', 1, 1),
        ('S', 2, 0),
        ('S', 2, 2),
    ];
    let flip_vertical = base
        .iter()
        .map(|(chr, y, x)| (*chr, 2 - y, *x))
        .collect::<Vec<_>>();

    let flip_lateral = base
        .iter()
        .map(|(chr, y, x)| (*chr, *x, *y))
        .collect::<Vec<_>>();

    let both_flip = flip_vertical
        .iter()
        .map(|(chr, y, x)| (*chr, *x, *y))
        .collect::<Vec<_>>();

    vec![base, flip_vertical, flip_lateral, both_flip]
}

fn check_part_2(
    x: usize,
    y: usize,
    grid: &Vec<Vec<char>>,
    arrangements: &Vec<Vec<(char, usize, usize)>>,
) -> bool {
    arrangements.iter().any(|arrangement| {
        arrangement
            .iter()
            .all(|(chr, offset_y, offset_x)| grid[y + offset_y][x + offset_x] == *chr)
    })
}

fn part_2() {
    let grid = get_grid();

    let y_bound = grid.len() - 2;
    let x_bound = grid[0].len() - 2;

    let arrangements = get_arrangements();

    let mut count = 0;

    for y in 0..y_bound {
        for x in 0..x_bound {
            if check_part_2(x, y, &grid, &arrangements) {
                count += 1;
            }
        }
    }

    println!("{count}");
}

pub fn compute() {
    part_1();
    part_2();
}
