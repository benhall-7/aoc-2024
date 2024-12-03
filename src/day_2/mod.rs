use std::cmp::Ordering;

fn get_lists() -> Vec<Vec<u32>> {
    let raw = include_str!("input.txt");
    raw.lines()
        .map(|line| {
            line.split(' ')
                .map(|word| u32::from_str_radix(word, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn is_safe(list: &Vec<u32>) -> bool {
    let direction = list[1].cmp(&list[0]);
    if let Ordering::Equal = direction {
        return false;
    }

    for i in 1..list.len() {
        let prev = list[i - 1];
        let val = list[i];

        if val.abs_diff(prev) > 3 || val.cmp(&prev) != direction {
            return false;
        }
    }

    true
}

pub fn part_1() {
    let lists = get_lists();
    let safe_count: usize = lists.iter().filter(|&list| is_safe(list)).count();
    println!("{safe_count}");
}

pub fn part_2_brute() {
    let lists = get_lists();
    let safe_count: usize = lists
        .into_iter()
        .filter(|orig_list| {
            return (-1..orig_list.len() as isize)
                .map(|ind| {
                    if ind == -1 {
                        orig_list.clone()
                    } else {
                        let mut clone = orig_list.clone();
                        clone.remove(ind as usize);
                        clone
                    }
                })
                .any(|list| is_safe(&list));
        })
        .count();
    println!("{safe_count}");
}

pub fn compute() {
    part_1();
    part_2_brute();
}
