use nalgebra::{Matrix2, Vector2};
use regex::Regex;

fn get_machines() -> Vec<ClawMachine> {
    let input = include_str!("input.txt");
    let a_regex =
        Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").expect("expected a valid regex expression");
    let b_regex =
        Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").expect("expected a valid regex expression");
    let prize_regex =
        Regex::new(r"Prize: X=(\d+), Y=(\d+)").expect("expected a valid regex expression");
    input
        .lines()
        .array_chunks()
        .map(|[a_button, b_button, prize, _]| {
            let (a_x_step, a_y_step) = a_regex
                .captures(a_button)
                .map(|c| {
                    let (_, [x, y]) = c.extract();
                    (
                        usize::from_str_radix(x, 10).unwrap(),
                        usize::from_str_radix(y, 10).unwrap(),
                    )
                })
                .expect("expected the regex to match the string");
            let (b_x_step, b_y_step) = b_regex
                .captures(b_button)
                .map(|c| {
                    let (_, [x, y]) = c.extract();
                    (
                        usize::from_str_radix(x, 10).unwrap(),
                        usize::from_str_radix(y, 10).unwrap(),
                    )
                })
                .expect("expected the regex to match the string");
            let (prize_x, prize_y) = prize_regex
                .captures(prize)
                .map(|c| {
                    let (_, [x, y]) = c.extract();
                    (
                        usize::from_str_radix(x, 10).unwrap(),
                        usize::from_str_radix(y, 10).unwrap(),
                    )
                })
                .expect("expected the regex to match the string");
            ClawMachine {
                a_step: Vector2::new(a_x_step, a_y_step),
                b_step: Vector2::new(b_x_step, b_y_step),
                prize: Vector2::new(prize_x, prize_y),
            }
        })
        .collect()
}

#[derive(Debug, Default, Clone, Copy, Hash)]
struct ClawMachine {
    pub a_step: Vector2<usize>,
    pub b_step: Vector2<usize>,
    pub prize: Vector2<usize>,
}

impl ClawMachine {
    pub fn get_solution(&self) -> Option<Vector2<usize>> {
        let a_float: Vector2<f64> = Vector2::new(self.a_step.x as f64, self.a_step.y as f64);
        let b_float: Vector2<f64> = Vector2::new(self.b_step.x as f64, self.b_step.y as f64);
        let prize_float: Vector2<f64> = Vector2::new(self.prize.x as f64, self.prize.y as f64);
        let space = Matrix2::from_columns(&[a_float, b_float]);
        let solution: Vector2<f64> =
            space.try_inverse().expect("couldn't invert the matrix >:(") * prize_float;

        let int_solution = solution.map(|val| val.round() as usize);

        // check the integers solutions
        (Matrix2::from_columns(&[self.a_step, self.b_step]) * int_solution == self.prize)
            .then(|| int_solution)
    }
}

fn solution_cost(solution: Vector2<usize>) -> usize {
    solution.x * 3 + solution.y
}

fn part_1() {
    let solutions = get_machines()
        .iter()
        .enumerate()
        .filter_map(|(ind, machine)| machine.get_solution().map(|sol| (ind, sol)))
        .filter(|(_, solution)| (solution.x <= 100 && solution.y <= 100))
        .collect::<Vec<_>>();

    // println!("solvable machines: {:#?}", solutions);

    let cost: usize = solutions
        .iter()
        .map(|(_, solution)| solution_cost(*solution))
        .sum();

    println!("cost: {cost}");
}

fn part_2() {
    let mut machines = get_machines();
    machines.iter_mut().for_each(|machine| {
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;
    });
    let solutions = machines
        .iter()
        .enumerate()
        .filter_map(|(ind, machine)| machine.get_solution().map(|sol| (ind, sol)))
        .collect::<Vec<_>>();

    let cost: usize = solutions
        .iter()
        .map(|(_, solution)| solution_cost(*solution))
        .sum();

    println!("cost: {cost}");
}

pub fn compute() {
    part_1();
    part_2();
}
