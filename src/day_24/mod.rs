use std::collections::{HashMap, VecDeque};

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    name: String,
    parents: Option<Box<(Operator, Node, Node)>>,
}

fn get_inputs() -> HashMap<String, bool> {
    let inputs = include_str!("inputs.txt");
    let regex = Regex::new(r"(.+): (\d)").unwrap();

    inputs
        .lines()
        .map(|line| {
            regex
                .captures(line)
                .map(|c| {
                    let (_, [a, b]) = c.extract();
                    (a.to_string(), b == "1")
                })
                .expect("expected the regex to match")
        })
        .collect()
}

fn get_gates() -> Vec<(String, String, Operator, String)> {
    let gates = include_str!("gates.txt");
    let regex = Regex::new(r"(.+) (AND|OR|XOR) (.+) -> (.+)").unwrap();

    gates
        .lines()
        .map(|line| {
            regex
                .captures(line)
                .map(|c| {
                    let (_, [a, b, c, d]) = c.extract();
                    let op = match b {
                        "AND" => Operator::And,
                        "OR" => Operator::Or,
                        "XOR" => Operator::Xor,
                        _ => panic!("invalid match: {}", b),
                    };
                    (a.to_string(), c.to_string(), op, d.to_string())
                })
                .unwrap()
        })
        .collect()
}

fn evaluate(mut solved: HashMap<String, bool>) -> usize {
    let gates = get_gates();
    let mut unsolved = gates
        .into_iter()
        .map(|(a, b, op, r)| (r, (op, a, b)))
        .collect::<HashMap<_, _>>();
    while !unsolved.is_empty() {
        for (output_name, inputs) in unsolved.clone() {
            let solution_a = solved.get(&inputs.1).cloned();
            let solution_b = solved.get(&inputs.2).cloned();

            if let Some((input_1, input_2)) = solution_a.zip(solution_b) {
                let result = match inputs.0 {
                    Operator::And => input_1 && input_2,
                    Operator::Or => input_1 || input_2,
                    Operator::Xor => input_1 ^ input_2,
                };

                unsolved.remove(&output_name);
                solved.insert(output_name, result);
            }
        }
    }
    let mut result: usize = 0;
    solved
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .for_each(|(z, value)| {
            let index = usize::from_str_radix(&z[1..], 10).expect("a decimal after the Z");
            if value {
                result |= 1 << index;
            }
        });

    result
}

fn translate_inputs(input_x: usize, input_y: usize) -> HashMap<String, bool> {
    let xs = (0..=44)
        .map(|var_num| {
            (
                format!("x{:0>2}", var_num),
                (input_x & 1usize << var_num) > 0,
            )
        })
        .collect::<Vec<_>>();
    let ys = (0..=44)
        .map(|var_num| {
            (
                format!("y{:0>2}", var_num),
                (input_y & 1usize << var_num) > 0,
            )
        })
        .collect::<Vec<_>>();
    [xs, ys].into_iter().flatten().collect()
}

fn part_1() {
    let solved = get_inputs();
    let result = evaluate(solved);

    println!("resulting calculation: {}", result);
}

fn get_node(base: String, gates: &HashMap<String, (Operator, String, String)>) -> Node {
    Node {
        name: base.clone(),
        parents: gates
            .get(&base)
            .cloned()
            .map(|(op, a, b)| Box::new((op, get_node(a, gates), get_node(b, gates)))),
    }
}

fn part_2() {
    let gates = get_gates();
    let unsolved = gates
        .into_iter()
        .map(|(a, b, op, r)| (r, (op, a, b)))
        .collect::<HashMap<_, _>>();
    ["z00", "z01", "z02", "z03", "z04", "z05", "z06", "z07"]
        .iter()
        .for_each(|output| {
            let node = get_node(output.to_string(), &unsolved);
            println!("TESTING {}", output);
            println!("{:#?}", node);
        });
    println!("worked out the answer by hand lol");
    // 7, 13, 24, 31
    // z07, z13, z24?, z31 are definitely wrong. What were they swapped with?
    // z07 = (x07 ^ y07) ^ ((x06 && y06) || ())
    // x07 ^ y07 -> kpv, kpv is XOR to produce swt.
    //
    // z07 <-> swt
    // z13 <-> pqc
    // y24 XOR x24 -> wsv <-> [ktp or rjm], rjm?
    // z31 <-> bgs
    // bgs,pqc,rjm,swt,wsv,z07,z13,z31
}

pub fn compute() {
    part_1();
    part_2();
}
