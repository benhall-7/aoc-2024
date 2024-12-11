use std::collections::HashMap;

fn transform(stone: usize) -> (usize, Option<usize>) {
    let digit_count = stone.checked_ilog10().unwrap_or(0) + 1;
    if stone == 0 {
        (1, None)
    } else if digit_count % 2 == 0 {
        let right_side = stone % 10usize.pow(digit_count / 2);
        let left_side = stone / 10usize.pow(digit_count / 2);
        (left_side, Some(right_side))
    } else {
        (stone * 2024, None)
    }
}

#[test]
fn test_transform() {
    assert_eq!(transform(0), (1, None));
    assert_eq!(transform(1234), (12, Some(34)));
    assert_eq!(transform(123), (123 * 2024, None));
}

fn part_1() {
    let mut stones = vec![0, 7, 6618216, 26481, 885, 42, 202642, 8791];

    for _ in 0..25 {
        let mut new_stones = stones
            .iter_mut()
            .filter_map(|stone| {
                let new_stones = transform(*stone);
                *stone = new_stones.0;
                new_stones.1
            })
            .collect::<Vec<_>>();
        stones.append(&mut new_stones);
    }

    println!("{}", stones.len());
}

fn multiply_stones(stone: usize, steps: usize, seen: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(count) = seen.get(&(stone, steps)) {
        return *count;
    }
    if steps == 0 {
        return 1;
    }
    let next = transform(stone);
    let result = multiply_stones(next.0, steps - 1, seen)
        + next
            .1
            .map(|other| multiply_stones(other, steps - 1, seen))
            .unwrap_or(0);
    seen.insert((stone, steps), result);
    result
}

fn part_2() {
    let stones = vec![0, 7, 6618216, 26481, 885, 42, 202642, 8791];
    let mut seen = HashMap::new();
    let sum: usize = stones
        .iter()
        .map(|stone| multiply_stones(*stone, 75, &mut seen))
        .sum();

    println!("{sum}");
}

pub fn compute() {
    part_1();
    part_2();
}
