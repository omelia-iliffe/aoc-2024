use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{AddAssign};
use derive_more::Display;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;

advent_of_code::solution!(17);

#[derive(Default, Clone)]
struct Computer {
    instruction_pointer: usize,
    a: i64,
    b: i64,
    c: i64,
    instructions: Vec<U3>,
    output: Vec<U3>,
    op_count: HashMap<OpCode, usize>,
}

impl Debug for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Computer")
        .field("instruction_pointer", &self.instruction_pointer)
        .field("a", &self.a)
        .field("b", &self.b)
        .field("c", &self.c)
        .finish()
    }
}

impl Computer {
    fn new(i: usize, a: i64, b: i64, c: i64, instructions: Vec<U3>) -> Self {
        Self {
            instruction_pointer: i,
            a,
            b,
            c,
            instructions,
            ..Self::default()
        }
    }

    fn step(&mut self) -> bool {
        use OpCode::*;
        let opcode = OpCode::new(self.instructions[self.instruction_pointer]);
        let operand = self.instructions[self.instruction_pointer + 1];
        let mut increase = true;
        // println!("running {opcode:?} with operand: {operand:?}");
        self.op_count.entry(opcode).or_default().add_assign(1);
        match opcode {
            Adv => self.dv(Register::A, operand),
            Bdv => self.dv(Register::B, operand),
            Cdv => self.dv(Register::C, operand),
            Bxl => self.set_register(Register::B, self.b ^ operand.as_i64()),
            Bst => self.set_register(Register::B, self.evaluate_combo(operand) % 8),
            Jnz => {
                if self.a != 0 {
                    // println!("jumping to {operand:?}");
                    self.instruction_pointer = operand.as_usize();
                    increase = false;
                // } else {
                //     println!("not jumping");
                }
            }
            Bxc => {
                self.set_register(Register::B, self.b ^ self.c);
            }
            Out => {
                self.output.push(U3::new((self.evaluate_combo(operand) % 8) as u8));
            }
        }
        if increase {
            self.instruction_pointer += 2;
        }
        self.instruction_pointer < self.instructions.len()
    }

    fn dv(&mut self, register: Register, operand: U3) {
        let n = self.register(Register::A);
        let d = self.evaluate_combo(operand);
        let r = n / 2_i64.pow(d as u32);
        self.set_register(register, r);
    }

    fn evaluate_combo(&self, operand: U3) -> i64 {
        match operand.0 {
            0..=3 => operand.as_i64(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("operand 7 is reserved"),
            n => unreachable!("invalid operand {}", n),
        }
    }

    fn register(&mut self, register: Register) -> i64 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
        }
    }

    fn set_register(&mut self, register: Register, value: i64) {
        // println!("setting register {register:?} to {value}");
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
        }
    }

    fn run(&mut self) -> i64 {
        let mut steps = 0;
        while self.step() {
            steps += 1;
        }
        steps
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl OpCode {
    fn new(opcode: U3) -> OpCode {
        match opcode.0 {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            n => panic!("invalid opcode {n}"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Register {
    A,
    B,
    C,
}

#[derive(Clone, Copy, PartialEq, Eq, derive_more::Debug, Display)]
#[debug("{:?}", _0)]
#[display("{:?}", _0)]
struct U3(u8);

impl U3 {
    fn new(n: u8) -> Self {
        if n > 7 {
            panic!("Invalid u3: {}", n);
        }
        U3(n)
    }

    fn as_i64(&self) -> i64 {
        self.0 as i64
    }

    fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

fn parse_register(input: &str) -> IResult<&str, (Register, i64)> {
    let (input, (_, name, _, value)) = tuple((
        tag("Register "),
        complete::alpha1, // Register name
        tag(": "),
        complete::i64,
    ))(input)?;
    match name {
        "A" => Ok((input, (Register::A, value))),
        "B" => Ok((input, (Register::B, value))),
        "C" => Ok((input, (Register::C, value))),
        n => panic!("invalid register {n}"),
    }
}

fn parse_program(input: &str) -> IResult<&str, Vec<u8>> {
    preceded(tag("\nProgram: "), separated_list1(tag(","), complete::u8))(input)
}

fn parse(input: &str) -> IResult<&str, Computer> {
    let (input, (regs, program)) = separated_pair(
        separated_list1(line_ending, parse_register),
        line_ending,
        // complete::alpha1,
        parse_program,
    )(input)?;
    let a = regs
        .iter()
        .find_map(|(r, v)| matches!(r, Register::A).then_some(*v))
        .unwrap();
    let b = regs
        .iter()
        .find_map(|(r, v)| matches!(r, Register::B).then_some(*v))
        .unwrap();
    let c = regs
        .iter()
        .find_map(|(r, v)| matches!(r, Register::C).then_some(*v))
        .unwrap();

    let instructions = program.into_iter().map(U3::new).collect_vec();

    Ok((input, Computer::new(0, a, b, c, instructions)))
}

pub fn part_one(input: &str) -> Option<String> {
    let mut steps = 0;
    let (_, mut computer) = parse(input).unwrap();
    while computer.instruction_pointer < computer.instructions.len() {
        computer.step();
        steps += 1;
    }
    println!("steps: {steps}");

    println!("opcodes {:#?}", computer.op_count);
    Some(computer.output.into_iter().join(","))
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, mut computer) = parse(input).unwrap();

    computer.a = 0;
    let goal = computer.instructions.clone();
    let a = loop {
        let mut c = computer.clone();
        dbg!(&c.a);
        c.run();
        println!("{}", c.output.clone().into_iter().join(","));
        if c.output == goal {
            break computer.a
        } else {
            computer.a += 1;
            dbg!(computer.a);
        }
    };
    Some(a)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computer1() {
        let mut c = Computer {
            c: 9,
            instructions: vec![U3(2), U3(6)],
            ..Computer::default()
        };
        c.step();
        assert_eq!(c.b, 1);
    }
    #[test]
    fn test_computer2() {
        let mut c = Computer {
            a: 10,
            instructions: vec![5, 0, 5, 1, 5, 4]
                .into_iter()
                .map(U3::new)
                .collect_vec(),
            ..Computer::default()
        };
        while c.step() {}

        // assert_eq!(c.output, "")
    }

    #[test]
    fn test_computer3() {
        let mut c = Computer {
            a: 2024,
            instructions: vec![0, 1, 5, 4, 3, 0]
                .into_iter()
                .map(U3::new)
                .collect_vec(),
            ..Computer::default()
        };
        while c.step() {
            dbg!(&c);
        }
        // assert_eq!(c.output, "");
        assert_eq!(c.a, 0);
    }
    #[test]
    fn test_computer4() {
        let mut c = Computer {
            b: 29,
            instructions: vec![1, 7].into_iter().map(U3::new).collect_vec(),
            ..Computer::default()
        };
        while c.step() {
            dbg!(&c);
        }
        assert_eq!(c.b, 26);
    }

    #[test]
    fn test_computer5() {
        let mut c = Computer {
            b: 2024,
            c: 43690,
            instructions: vec![4, 0].into_iter().map(U3::new).collect_vec(),
            ..Computer::default()
        };
        while c.step() {
            dbg!(&c);
        }
        assert_eq!(c.b, 44354);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {

        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let result = part_two(input);
        assert_eq!(result, Some(117440));
    }
}
