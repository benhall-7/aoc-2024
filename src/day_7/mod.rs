use std::u64;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    pub fn operate(self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a.saturating_add(b),
            Operator::Mul => a.saturating_mul(b),
            Operator::Concat => {
                let digit_count = b.checked_ilog10().unwrap_or(0) + 1;
                a.saturating_mul(10_u64.pow(digit_count)).saturating_add(b)
            }
        }
    }
}

#[test]
fn test_operation() {
    assert_eq!(Operator::Concat.operate(12, 345), 12345);
}

fn get_input() -> Vec<(u64, Vec<u64>)> {
    let input = include_str!("input.txt");
    // let input = "7290: 6 8 6 15";
    input
        .lines()
        .map(|line| {
            let mut splits = line.split(":");
            let first = splits.next().expect("expected string before :");
            let after = splits.next().expect("expected string after :");
            (
                u64::from_str_radix(first, 10).expect("number to convert"),
                after
                    .split_whitespace()
                    .map(|word| u64::from_str_radix(word, 10).expect("number to convert"))
                    .collect(),
            )
        })
        .collect()
}

fn dfs(target: u64, acc: u64, remaining: &[u64]) -> Option<Vec<Operator>> {
    if acc > target {
        return None;
    }
    if remaining.is_empty() {
        if acc == target {
            return Some(vec![]);
        }
        return None;
    }
    for op in [Operator::Add, Operator::Mul] {
        let result = op.operate(acc, remaining[0]);
        if let Some(mut op_list) = dfs(target, result, &remaining[1..]) {
            op_list.push(op);
            return Some(op_list);
        }
    }

    None
}

fn dfs_2(target: u64, acc: u64, remaining: &[u64]) -> Option<Vec<Operator>> {
    if acc > target {
        return None;
    }
    if remaining.is_empty() {
        if acc == target {
            return Some(vec![]);
        }
        return None;
    }
    for op in [Operator::Add, Operator::Mul, Operator::Concat] {
        let result = op.operate(acc, remaining[0]);
        if let Some(mut op_list) = dfs_2(target, result, &remaining[1..]) {
            op_list.push(op);
            return Some(op_list);
        }
    }

    None
}

fn part_1() {
    let input = get_input();
    let sum: u64 = input
        .iter()
        .filter_map(|(target, operands)| dfs(*target, operands[0], &operands[1..]).map(|_| *target))
        .sum();
    println!("{sum}");
}

fn part_2() {
    let input = get_input();
    let sum: u64 = input
        .iter()
        .filter_map(|(target, operands)| {
            dfs_2(*target, operands[0], &operands[1..]).map(|ops| {
                // println!(
                //     "{}",
                //     ops.iter()
                //         .rev()
                //         .map(|op| format!("{:#?}", op))
                //         .collect::<Vec<String>>()
                //         .join(",")
                // );
                *target
            })
        })
        .sum();
    println!("{sum}");
}

pub fn compute() {
    part_1();
    part_2();
}
