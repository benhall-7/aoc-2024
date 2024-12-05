use std::collections::{HashMap, HashSet};

fn get_precedence_rules() -> HashMap<u32, Vec<u32>> {
    let input = include_str!("rules.txt");
    let rule_numbers = input
        .lines()
        .map(|line| {
            (
                u32::from_str_radix(&line[0..2], 10).unwrap(),
                u32::from_str_radix(&line[3..5], 10).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    rule_numbers.iter().fold(HashMap::new(), |mut rules, rule| {
        rules.entry(rule.0).or_default().push(rule.1);
        rules
    })
}

fn get_updates() -> Vec<Vec<u32>> {
    let input = include_str!("updates.txt");
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|update| u32::from_str_radix(update, 10).unwrap())
                .collect()
        })
        .collect()
}

fn check_precedence(updates: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut previous_numbers = HashSet::new();
    let empty_vec = vec![];
    updates.iter().all(|num| {
        let proceeding_numbers = rules.get(num).unwrap_or(&empty_vec);
        let order_broken = proceeding_numbers
            .iter()
            .any(|proceeding| previous_numbers.contains(proceeding));
        previous_numbers.insert(*num);

        !order_broken
    })
}

fn sort(updates: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Option<Vec<u32>> {
    let mut updates = updates.clone();
    let mut previous_numbers = HashSet::new();
    let empty_vec = vec![];
    let mut resorted = false;
    for i in 0..updates.len() {
        let num = updates[i];
        let proceeding_numbers = rules.get(&num).unwrap_or(&empty_vec);
        let earlier_mistakes: HashSet<u32> = proceeding_numbers
            .iter()
            .filter(|proceeding| previous_numbers.contains(*proceeding))
            .map(|a| *a)
            .collect();

        let earliest_mistake = updates
            .iter()
            .enumerate()
            .find(|(_, update_val)| earlier_mistakes.contains(*update_val))
            .map(|pair| pair.0);

        if let Some(mistake_index) = earliest_mistake {
            // move the element at i to the place right before the mistake index
            // by slicing and using a right-shift of 1
            updates[mistake_index..=i].rotate_right(1);
            resorted = true
        }

        previous_numbers.insert(num);
    }

    resorted.then_some(updates)
}

fn part_1() {
    let rules = get_precedence_rules();
    let update_lists = get_updates();
    let sum = update_lists.iter().fold(0, |acc, updates| {
        if check_precedence(updates, &rules) {
            let middle = updates[(updates.len() - 1) / 2];
            acc + middle
        } else {
            acc
        }
    });

    println!("{sum}");
}

fn part_2() {
    // should have done a true topological sort...
    let rules = get_precedence_rules();
    let update_lists = get_updates();
    let sum: u32 = update_lists
        .iter()
        .map(|updates| {
            let resorted = sort(updates, &rules);
            if let Some(resorted) = resorted {
                resorted[(resorted.len() - 1) / 2]
            } else {
                0
            }
        })
        .sum();
    
    println!("{sum}")
}

pub fn compute() {
    part_1();
    part_2();
}
