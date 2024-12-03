use std::{collections::HashMap, ops::AddAssign};

use list::LIST_1;

pub mod list;

// calculate distance between closest ordered ints
pub fn compute() {
    println!("PART 1");
    part_1();
    println!("PART 2");
    part_2();
}

fn part_1() {
    let mut vec_1 = Vec::from(list::LIST_1);
    let mut vec_2 = Vec::from(list::LIST_2);

    vec_1.sort();
    vec_2.sort();

    let distance: u32 = vec_1
        .into_iter()
        .zip(vec_2)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    println!("{distance}");
}

fn part_2() {
    let right_counts: HashMap<u32, u32> =
        list::LIST_2.iter().fold(HashMap::new(), |mut acc, val| {
            acc.entry(*val).or_insert(0).add_assign(1);
            acc
        });

    let similarity: u32 = LIST_1
        .iter()
        .map(|&left| {
            left * right_counts
                .get(&left)
                .map(Clone::clone)
                .unwrap_or_default()
        })
        .sum();

    println!("{similarity}");
}
