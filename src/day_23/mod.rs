use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn get_graph() -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::new();

    include_str!("input.txt").lines().for_each(|line| {
        let mut splits = line.split("-");
        let arr: [&str; 2] = std::array::from_fn(|_| splits.next().unwrap());

        let entry_0: &mut HashSet<String> = graph.entry(arr[0].to_string()).or_default();
        entry_0.insert(arr[1].to_string());
        let entry_1: &mut HashSet<String> = graph.entry(arr[1].to_string()).or_default();
        entry_1.insert(arr[0].to_string());
    });

    graph
}

fn get_data() -> (Vec<[String; 2]>, Vec<String>) {
    let mut vertex_set = HashSet::new();
    let edges = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut splits = line.split("-");
            let mut arr: [&str; 2] = std::array::from_fn(|_| splits.next().unwrap());

            vertex_set.insert(arr[0]);
            vertex_set.insert(arr[1]);
            arr.sort();
            arr.map(String::from)
        })
        .collect();
    let vertices = vertex_set.into_iter().map(String::from).collect();
    (edges, vertices)
}

fn part_1() {
    let (edges, vertices) = get_data();
    let edge_set = edges
        .iter()
        .map(|pair| [&pair[0], &pair[1]])
        .collect::<HashSet<_>>();
    let triplets = vertices
        .into_iter()
        .tuple_combinations()
        .map(|(a, b, c)| {
            let mut three = [a, b, c];
            three.sort();
            three
        })
        .filter(|three| three.iter().any(|name| name.starts_with("t")))
        .filter(|three| {
            edge_set.contains(&[&three[0], &three[1]])
                && edge_set.contains(&[&three[0], &three[2]])
                && edge_set.contains(&[&three[1], &three[2]])
        })
        .collect::<Vec<_>>();

    println!("triplet count {}", triplets.len());
}

fn bron_kerbosch(
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    graph: &HashMap<String, HashSet<String>>,
    results: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        results.push(r.clone());
    }
    for v in p.clone() {
        let new_r = &r | &HashSet::from([v.clone()]);
        let neighbors = graph
            .get(&v)
            .expect("expected the graph to include the key");
        bron_kerbosch(new_r, &p & neighbors, &x & neighbors, graph, results);

        p.remove(&v);
        x.insert(v.clone());
    }
}

fn part_2() {
    let graph = get_graph();
    let mut results = vec![];
    bron_kerbosch(
        HashSet::new(),
        graph.keys().map(|key| key.to_owned()).collect(),
        HashSet::new(),
        &graph,
        &mut results,
    );

    let maximum = results.into_iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap();
    let sorted = maximum.into_iter().sorted().collect::<Vec<_>>();
    println!("biggest clique:");
    println!("{}", sorted.join(","));
}

pub fn compute() {
    part_1();
    part_2();
}
