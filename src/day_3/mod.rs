use regex::Regex;

fn part_1() {
    let input = include_str!("input.txt");
    let regex_matcher = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let sum: u32 = regex_matcher
        .captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            u32::from_str_radix(a, 10).unwrap() * u32::from_str_radix(b, 10).unwrap()
        })
        .sum();
    println!("{sum}");
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Mul(u32, u32),
    On,
    Off,
}

#[derive(Debug)]
struct Machine {
    is_on: bool,
    sum: u32,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            is_on: true,
            sum: 0,
        }
    }

    pub fn instruct(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Mul(a, b) => {
                if self.is_on {
                    self.sum += a * b;
                }
            }
            Instruction::Off => self.is_on = false,
            Instruction::On => self.is_on = true,
        }
    }
}

fn part_2() {
    let input = include_str!("input.txt");
    let regex_matcher: Regex =
        Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don\'t)\(\)").unwrap();
    let sum = regex_matcher
        .captures_iter(input)
        .map(|c| {
            let components = c.iter().skip(1).collect::<Vec<_>>();
            if components[0].is_some() {
                Instruction::Mul(
                    u32::from_str_radix(components[1].unwrap().as_str(), 10).unwrap(),
                    u32::from_str_radix(components[2].unwrap().as_str(), 10).unwrap(),
                )
            } else if components[3].is_some() {
                Instruction::On
            } else if components[4].is_some() {
                Instruction::Off
            } else {
                panic!("shouldn't be possible");
            }
        })
        .fold(Machine::new(), |mut machine, instr| {
            machine.instruct(instr);
            machine
        })
        .sum;

    println!("{sum}");
}

pub fn compute() {
    part_1();
    part_2();
}
