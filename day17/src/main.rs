use std::{fs::File, io::read_to_string};

fn main() {
    //let str = read_to_string(File::open("example.txt").unwrap()).unwrap();
    //let str = read_to_string(File::open("p2_example.txt").unwrap()).unwrap();
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
    println!("{}", part2(computer, &instructions));
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

fn part2(computer: Computer, instructions: &[i64]) -> i64 {
    let mut found = 0;
    for l in (0..instructions.len()).rev() {
        found <<= 3;
        let f = found;
        for possibility in f..f + 8 {
            let mut tmp = computer;
            tmp.registers[0] = possibility;
            let output = part1(tmp, instructions);
            if output == instructions[l..] {
                dbg!(output, possibility);
                found = possibility;
                break;
            }
        }
    }
    dbg!(found);

    let mut tmp = computer;
    tmp.registers[0] = found;
    let output = part1(tmp, instructions);
    assert_eq!(output, instructions);
    found
    //for i in (0b101110000 << 3)..(0b101110000 << 3) + 8 {
    //    let mut tmp = computer;
    //    tmp.registers[0] = i;
    //    let output = part1(tmp, instructions);
    //    println!("{output:?}, {i}, {i:0b}");
    //    //println!("{output:?}");
    //}
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
                self.registers[0] =
                    (self.registers[0] as f32 / 2f32.powi(combo_operand as i32)).trunc() as i64
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
                self.registers[1] =
                    (self.registers[0] as f32 / 2f32.powi(combo_operand as i32)).trunc() as i64
            }
            7 => {
                self.registers[2] =
                    (self.registers[0] as f32 / 2f32.powi(combo_operand as i32)).trunc() as i64
            }
            _ => unreachable!(),
        };
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_c_9_program() {
        let computer = Computer {
            registers: [0, 0, 9],
            instruction_pointer: 0,
        };
        let instructions = vec![2, 6];
        part1(computer, &instructions);
        assert_eq!(computer.registers[1], 1);
    }

    #[test]
    fn test_register_a_10_program() {
        let computer = Computer {
            registers: [10, 0, 0],
            instruction_pointer: 0,
        };
        let instructions = vec![5, 0, 5, 1, 5, 4];
        let output = part1(computer, &instructions);
        assert_eq!(output, vec![0, 1, 2]);
    }

    #[test]
    fn test_register_a_2024_program() {
        let computer = Computer {
            registers: [2024, 0, 0],
            instruction_pointer: 0,
        };
        let instructions = vec![0, 1, 5, 4, 3, 0];
        let output = part1(computer, &instructions);
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.registers[0], 0);
    }

    #[test]
    fn test_register_b_29_program() {
        let computer = Computer {
            registers: [0, 29, 0],
            instruction_pointer: 0,
        };
        let instructions = vec![1, 7];
        part1(computer, &instructions);
        assert_eq!(computer.registers[1], 26);
    }

    #[test]
    fn test_register_b_2024_c_43690_program() {
        let computer = Computer {
            registers: [0, 2024, 43690],
            instruction_pointer: 0,
        };
        let instructions = vec![4, 0];
        part1(computer, &instructions);
        assert_eq!(computer.registers[1], 44354);
    }
}
