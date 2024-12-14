use std::{cmp::Ordering, collections::HashSet};

use nalgebra::Vector2;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    pos: Vector2<isize>,
    vel: Vector2<isize>,
}

fn get_robots() -> Vec<Robot> {
    let input = include_str!("input.txt");
    let reg =
        Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").expect("expected a valid regex expression");
    input
        .lines()
        .map(|line| {
            let (_, [px, py, vx, vy]) = reg
                .captures(line)
                .expect("expected line to match regex")
                .extract();
            Robot {
                pos: Vector2::new(
                    isize::from_str_radix(px, 10).expect("expected a number"),
                    isize::from_str_radix(py, 10).expect("expected a number"),
                ),
                vel: Vector2::new(
                    isize::from_str_radix(vx, 10).expect("expected a number"),
                    isize::from_str_radix(vy, 10).expect("expected a number"),
                ),
            }
        })
        .collect()
}

fn part_1() {
    let width = 101;
    let height = 103;
    let turns = 100;
    let counts = get_robots()
        .iter()
        .map(|robot| {
            let end_position_unwrapped = robot.pos + robot.vel * turns;
            Robot {
                pos: Vector2::new(
                    end_position_unwrapped.x.rem_euclid(width),
                    end_position_unwrapped.y.rem_euclid(height),
                ),
                vel: robot.vel,
            }
        })
        .fold([0, 0, 0, 0], |mut counts, robot| {
            let quadrant = match (
                robot.pos.x.cmp(&(width / 2)),
                robot.pos.y.cmp(&(&height / 2)),
            ) {
                (Ordering::Less, Ordering::Less) => 0,
                (Ordering::Greater, Ordering::Less) => 1,
                (Ordering::Less, Ordering::Greater) => 2,
                (Ordering::Greater, Ordering::Greater) => 3,
                _ => return counts,
            };
            counts[quadrant] += 1;
            counts
        });

    println!(
        "safety score = {}",
        counts[0] * counts[1] * counts[2] * counts[3]
    );
}

fn part_2() {
    let width = 101;
    let height = 103;
    let robots = get_robots();
    let mut max_contiguous = 0;
    for i in 0..10_000 {
        let positions = robots
            .iter()
            .map(|robot| {
                let end_position_unwrapped = robot.pos + robot.vel * i;
                Vector2::new(
                    end_position_unwrapped.x.rem_euclid(width),
                    end_position_unwrapped.y.rem_euclid(height),
                )
            })
            .collect::<Vec<_>>();
        let mut position_set = positions.clone().into_iter().collect::<HashSet<_>>();
        // count max contiguous positions
        while !position_set.is_empty() {
            // take out an element, and then find and remove its neighbors iteratively
            // then, count how many were removed
            let num_before = position_set.len();
            let pos = position_set.iter().nth(0).unwrap();

            let mut remove_stack = vec![*pos];
            // let mut removed = HashSet::new();
            while let Some(to_remove) = remove_stack.pop() {
                if position_set.remove(&to_remove) {
                    // removed.insert(to_remove);
                    remove_stack.extend(
                        [
                            Vector2::new(to_remove.x - 1, to_remove.y),
                            Vector2::new(to_remove.x, to_remove.y - 1),
                            Vector2::new(to_remove.x + 1, to_remove.y),
                            Vector2::new(to_remove.x, to_remove.y + 1),
                        ]
                        .iter()
                        .filter(|neigh| position_set.contains(neigh)),
                    );
                }
            }
            let num_after = position_set.len();
            let num_removed_at_once = num_before - num_after;

            if num_removed_at_once > max_contiguous {
                println!("turn #{}", i);
                println!("contiguous robots: {}", num_removed_at_once);
                println!();
                max_contiguous = num_removed_at_once;
            }
        }
    }
}

pub fn compute() {
    // part_1();
    part_2();
}
