use std::{fs::File, io::read_to_string, time::Instant};

fn main() {
    //let str = read_to_string(File::open("example.txt").unwrap()).unwrap();
    //let str = read_to_string(File::open("p2_example.txt").unwrap()).unwrap();
    let mut t = Instant::now();
    let str = read_to_string(File::open("input.txt").unwrap()).unwrap();
    let (registers, instr) = str.split_once("\n\n").unwrap();

    let mut regs = [0; 3];
    for (i, line) in registers.lines().enumerate() {
        let (_, val) = line.split_once(":").unwrap();
        regs[i] = val.trim().parse::<i64>().unwrap();
    }
    let instructions = instr
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let computer = Computer {
        registers: regs,
        instruction_pointer: 0,
    };
    println!(
        "{}",
        part1(computer, &instructions)
            .into_iter()
            .map(|x| x.to_string() + ",")
            .collect::<String>()
    );
    let mut values = instructions.clone();
    let mut results = vec![];
    part2(&mut values, &instructions, 0, &mut results, 1);
    dbg!(results);
    dbg!(t.elapsed());
}

fn part2(
    values: &mut Vec<i64>,
    instructions: &[i64],
    a: i64,
    results: &mut Vec<i64>,
    level: usize,
) {
    if values.is_empty() {
        return;
    }
    let val = values.pop().unwrap();
    let mut candidates = Vec::new();
    for i in 0..8 {
        let tmp = Computer {
            registers: [a + i, 0, 0],
            instruction_pointer: 0,
        };
        let output = part1(tmp, instructions);
        if output[0] == val {
            candidates.push(i);
            if level == instructions.len() {
                results.push(a + i);
            }
        }
    }
    for candidate in candidates {
        let mut v = values.clone();
        part2(
            &mut v,
            instructions,
            (a + candidate) << 3,
            results,
            level + 1,
        );
    }
}

fn part1(mut computer: Computer, instructions: &[i64]) -> Vec<i64> {
    let mut output = vec![];
    while (computer.instruction_pointer as usize) < instructions.len() {
        if let Some(result) = computer.run_instruction(
            instructions[computer.instruction_pointer as usize],
            instructions[computer.instruction_pointer as usize + 1],
        ) {
            output.push(result);
        }
    }
    output
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Computer {
    registers: [i64; 3],
    instruction_pointer: isize,
}

impl Computer {
    fn run_instruction(&mut self, opcode: i64, literal_operand: i64) -> Option<i64> {
        let combo_operand = match literal_operand {
            0..=3 => literal_operand,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            7 => i64::MAX,
            _ => unreachable!(),
        };
        self.instruction_pointer += 2;
        match opcode {
            0 => {
                self.registers[0] /= 2i64.pow(combo_operand as u32);
            }
            1 => {
                self.registers[1] ^= literal_operand;
            }
            2 => {
                self.registers[1] = combo_operand % 8;
            }
            3 => {
                if self.registers[0] != 0 {
                    self.instruction_pointer = literal_operand as isize;
                }
            }
            4 => {
                self.registers[1] ^= self.registers[2];
            }
            5 => {
                return Some(combo_operand % 8);
            }
            6 => {
                self.registers[1] = self.registers[0] / 2i64.pow(combo_operand as u32);
            }
            7 => {
                self.registers[2] = self.registers[0] / 2i64.pow(combo_operand as u32);
            }
            _ => unreachable!(),
        };
        None
    }
}
