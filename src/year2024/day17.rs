use crate::util::day::Day;
use crate::util::integer::ParseOps;
use itertools::Itertools;

pub struct Day17;
impl Day for Day17 {
    type Input = (Vec<usize>, Vec<usize>);
    fn parse(notes: &str) -> Self::Input {
        let (registers, inputs) = notes.split_once("\n\n").unwrap();
        let registers =
            registers.lines().map(|line| line.split_once(": ").unwrap().1.int::<usize>()).collect();
        (
            registers,
            inputs.trim_start_matches("Program: ").split(',').map(|c| c.parse().unwrap()).collect(),
        )
    }

    type Output1 = String;
    fn part_1((registers, inputs): &Self::Input) -> Self::Output1 {
        let mut registers = registers.clone();
        let mut pointer = 0;
        let mut out = Vec::new();
        while pointer < inputs.len() {
            run(&mut registers, inputs, &mut pointer, &mut out);
        }
        out.iter().join(",")
    }

    type Output2 = usize;
    fn part_2((registers, inputs): &Self::Input) -> Self::Output2 {
        let mut registers = registers.clone();
        let mut out = Vec::new();
        let mut initial_value = 45_000_000;
        while !inputs.iter().eq(out.iter()) {
            registers[0] = initial_value;
            out.clear();
            let mut pointer = 0;
            while pointer < inputs.len() {
                run(&mut registers, inputs, &mut pointer, &mut out);
            }
            initial_value += 1;
            if initial_value % 1_000_000 == 0 {
                println!("Initial value: {}", initial_value);
            }
        }
        initial_value
    }
}

fn get_combo_operand(registers: &Vec<usize>, value: usize) -> usize {
    match value {
        0..=3 => value,
        4..=6 => registers[value - 4],
        _ => unreachable!(),
    }
}

fn run(registers: &mut Vec<usize>, inputs: &Vec<usize>, pointer: &mut usize, out: &mut Vec<usize>) {
    let opcode = inputs[*pointer];
    let literal_operand = inputs[*pointer + 1];
    let combo_operand = get_combo_operand(registers, literal_operand);
    match opcode {
        0 => registers[0] = registers[0] / 2usize.pow(combo_operand as u32),
        1 => registers[1] = registers[1] ^ literal_operand,
        2 => registers[1] = combo_operand % 8,
        3 => {
            if registers[0] != 0 {
                *pointer = literal_operand;
            }
        }
        4 => registers[1] = registers[1] ^ registers[2],
        5 => out.push(combo_operand % 8),
        6 => registers[1] = registers[0] / 2usize.pow(combo_operand as u32),
        7 => registers[2] = registers[0] / 2usize.pow(combo_operand as u32),
        _ => unreachable!(),
    };
    if opcode != 3 || registers[0] == 0 {
        *pointer += 2;
    }
}
