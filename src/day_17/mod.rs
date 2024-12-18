use std::ops::{BitXorAssign, Shr, ShrAssign};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Emulator {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub pc: usize,
    pub program: Vec<u8>,
    pub output: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instr {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

const INSTRUCTIONS: [Instr; 8] = [
    Instr::Adv,
    Instr::Bxl,
    Instr::Bst,
    Instr::Jnz,
    Instr::Bxc,
    Instr::Out,
    Instr::Bdv,
    Instr::Cdv,
];

impl Emulator {
    pub fn new_with_initial_state() -> Self {
        /*
           Register A: 23999685
           Register B: 0
           Register C: 0

           Program: 2,4,1,1,7,5,1,5,0,3,4,4,5,5,3,0
        */
        Self {
            a: 164516454365621,
            b: 0,
            c: 0,
            pc: 0,
            program: vec![2, 4, 1, 1, 7, 5, 1, 5, 0, 3, 4, 4, 5, 5, 3, 0],
            output: vec![],
        }
    }

    pub fn read_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        let opcode = self.program[self.pc];
        let operand = self.program[self.pc + 1];
        self.pc += 2;

        let instr = INSTRUCTIONS[opcode as usize];
        match instr {
            Instr::Adv => self.a.shr_assign(self.combo_value(operand)),
            Instr::Bxl => self.b.bitxor_assign(operand as usize),
            Instr::Bst => self.b = self.combo_value(operand) % 8,
            Instr::Jnz => {
                (self.a != 0).then(|| self.pc = operand as usize);
            }
            Instr::Bxc => self.b.bitxor_assign(self.c),
            Instr::Out => self.output.push((self.combo_value(operand) % 8) as u8),
            Instr::Bdv => self.b = self.a.shr(self.combo_value(operand)),
            Instr::Cdv => self.c = self.a.shr(self.combo_value(operand)),
        }

        true
    }

    pub fn combo_value(&self, combo: u8) -> usize {
        match combo {
            0..=3 => combo as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid combo value: {}", combo),
        }
    }
}

fn eval(a: usize) -> u8 {
    // 2,4 ; b  <-  a % 8  ; move bottom 3 bits of a into b
    // 1,1 ; b  ^=  1      ; XOR b with 1. b is now a % 8, except the ones digit is switched
    // 7,5 ; c  <-  a >> b ; c is A right shifted 0-7 bits (depending on A and A & 1)
    // 1,5 ; b  ^=  5      ; b ^= 5
    // 0,3 ; a >>=  3      ; move a 3 bits to the right
    // 4,4 ; b  ^=  c      ; b ^= c
    // 5,5 ; O  <<  b      ; add b to output, which is just c, which is just a >> b earlier
    // 3,0 ; a != 0 -> 0   ; loop if a > 0

    (((a >> ((a % 8) ^ 1)) ^ ((a % 8) ^ 4)) % 8) as u8
}

fn part_1() {
    let mut emu = Emulator::new_with_initial_state();
    while emu.read_instruction() {}
    let output = &emu.output;
    println!("{:?}", output.iter().map(|val| val.to_string()).join(","));
}

fn part_2() {
    // 2,4,1,1,7,5,1,5,0,3,4,4,5,5,3,0
    let emu_base = Emulator::new_with_initial_state();
    let mut program = emu_base.program.clone();
    // let mut program = vec![3, 0];
    let end_instr = program.pop().unwrap();
    let mut solutions = (0..8)
        .into_iter()
        .filter(|top_byte| eval(*top_byte) == end_instr)
        .collect::<Vec<_>>();
    while let Some(instr) = program.pop() {
        solutions = solutions
            .into_iter()
            .flat_map(|sol| (0..8).into_iter().map(move |addend| addend + (sol << 3)))
            .filter(|new_val| eval(*new_val) == instr)
            .collect();
    }

    println!("min a: {:?}", solutions.iter().min());
}

pub fn compute() {
    part_1();
    part_2();
}
