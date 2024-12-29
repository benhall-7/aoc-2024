use itertools::Itertools;
use nalgebra::{Matrix5, Vector5};

fn get_locks_and_keys() -> (Vec<[usize; 5]>, Vec<[usize; 5]>) {
    let input = include_str!("input.txt");
    let mut locks: Vec<[usize; 5]> = vec![];
    let mut keys: Vec<[usize; 5]> = vec![];
    input
        .lines()
        .array_chunks()
        .for_each(|[l1, l2, l3, l4, l5, l6, _l7, _l8]| {
            if l1.chars().all(|chr| chr == '#') {
                // locks
                let mat = Matrix5::from_row_iterator(
                    [l2, l3, l4, l5, l6]
                        .map(|line| line.chars().collect::<Vec<_>>())
                        .concat(),
                );
                locks.push(
                    mat.column_iter()
                        .map(|col| col.iter().filter(|cell| **cell == '#').count())
                        .collect::<Vec<_>>()
                        .try_into()
                        .expect("expected the array to be length 5"),
                );
            } else {
                // keys
                let mat = Matrix5::from_row_iterator(
                    [l2, l3, l4, l5, l6]
                        .map(|line| line.chars().collect::<Vec<_>>())
                        .concat(),
                );
                keys.push(
                    mat.column_iter()
                        .map(|col| col.iter().filter(|cell| **cell == '#').count())
                        .collect::<Vec<_>>()
                        .try_into()
                        .expect("expected the array to be length 5"),
                );
            }
        });

    (locks, keys)
}

fn part_1() {
    let (locks, keys) = get_locks_and_keys();
    let matching = locks
        .into_iter()
        .cartesian_product(keys)
        .map(|(lock, key)| {
            let vec1 = Vector5::from(lock);
            let vec2 = Vector5::from(key);
            vec1 + vec2
        })
        .filter(|sum| sum.max() <= 5)
        .count();

    println!("number of matching locks and keys {matching}");
}

pub fn compute() {
    part_1();
}
