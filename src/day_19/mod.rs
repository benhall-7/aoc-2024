use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn get_towels() -> HashSet<String> {
    let towels = include_str!("towels.txt");
    towels
        .split(", ")
        .map(|towel| towel.chars().collect())
        .collect()
}

fn get_designs() -> Vec<String> {
    let designs = include_str!("designs.txt");
    designs
        .lines()
        .map(|design| design.chars().collect())
        .collect()
}

fn path_counts(
    index: usize,
    design: &str,
    towels: &HashSet<String>,
    seen: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(seen_val) = seen.get(&index) {
        return *seen_val;
    }

    let combinations = match index.cmp(&design.len()) {
        Ordering::Greater => 0,
        Ordering::Equal => 1,
        Ordering::Less => ((index + 1)..=design.len())
            .rev()
            .map(|end| {
                let slice = &design[index..end];
                if towels.contains(slice) {
                    path_counts(end, design, towels, seen)
                } else {
                    0
                }
            })
            .sum(),
    };

    seen.insert(index, combinations);
    combinations
}

pub fn compute() {
    let towels = get_towels();
    let designs = get_designs();

    let count = designs
        .iter()
        .filter(|design| {
            let mut seen = HashMap::new();
            path_counts(0, design, &towels, &mut seen) > 0
        })
        .count();

    let sum: usize = designs
        .iter()
        .map(|design| {
            let mut seen = HashMap::new();
            path_counts(0, design, &towels, &mut seen)
        })
        .sum();

    println!("solvable count: {count}");
    println!("solutions: {sum}");
}
