use std::collections::HashMap;

use itertools::Itertools;
use seq_macro::seq;

const fn generate(secret: usize) -> usize {
    let mask = (1 << 24) - 1;
    let secret = ((secret << 6) ^ secret) & mask;
    let secret = ((secret >> 5) ^ secret) & mask;
    let secret = ((secret << 11) ^ secret) & mask;
    secret
}

fn get_input() -> Vec<usize> {
    include_str!("input.txt")
        .lines()
        .map(|line| usize::from_str_radix(line, 10).unwrap())
        .collect()
}

fn part_1() {
    let initials = get_input();
    let sum: usize = initials
        .into_iter()
        .map(|mut start| {
            // I suppose Rust will just optimize all the calls out lol
            seq!(_ in 0..2000 {
                start = generate(start);
            });
            start
        })
        .sum();

    println!("sum: {}", sum)
}

fn part_2() {
    let initials = get_input();
    let sequences = initials
        .into_iter()
        .map(|mut start| {
            let mut costs = vec![];
            for _ in 0..2000 {
                start = generate(start);
                costs.push(start % 10);
            }
            costs
                .into_iter()
                .tuple_windows()
                .map(|(v1, v2, v3, v4, v5)| {
                    let i1 = v1 as isize;
                    let i2 = v2 as isize;
                    let i3 = v3 as isize;
                    let i4 = v4 as isize;
                    let i5 = v5 as isize;
                    ([i2 - i1, i3 - i2, i4 - i3, i5 - i4], v5)
                })
                .unique_by(|entry| entry.0)
                .collect::<HashMap<[isize; 4], usize>>()
        })
        .collect::<Vec<_>>();

    let all_keys = sequences
        .iter()
        .flat_map(|seq| seq.keys().cloned())
        .unique()
        .collect::<Vec<_>>();

    let earned: usize = all_keys
        .iter()
        .map(|key| {
            sequences
                .iter()
                .map(|map| map.get(key).cloned().unwrap_or(0))
                .sum()
        })
        .max()
        .unwrap();

    println!("max earned: {earned}");
}

pub fn compute() {
    part_1();
    part_2();
}
