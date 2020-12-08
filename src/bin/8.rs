use std::{
    collections::HashSet,
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let stdin = io::stdin();

    let mut console = GameConsole::new(&mut stdin.lock()).unwrap();
    let _ = run_and_detect_cycle(&mut console);

    println!("Part 1: {}", console.acc());

    console.reset();

    let mut patched_console = console.clone();
    while let Some(inst) = console.peek() {
        let pc = patched_console.pc();
        match inst {
            &Instruction::Jmp(arg) => {
                patched_console.instructions[pc] = Instruction::Nop(arg);
                if !run_and_detect_cycle(&mut patched_console) {
                    break;
                }
            }
            &Instruction::Nop(arg) => {
                patched_console.instructions[pc] = Instruction::Jmp(arg);
                if !run_and_detect_cycle(&mut patched_console) {
                    break;
                }
            }
            _ => {}
        }

        console.run_step();
        patched_console = console.clone();
    }

    println!("Part 2: {}", patched_console.acc());
}

fn run_and_detect_cycle(console: &mut GameConsole) -> bool {
    let mut seen_states = HashSet::new();
    let mut pc = console.pc();

    while console.peek().is_some() {
        seen_states.insert(pc);
        console.run_step();
        pc = console.pc();
        if seen_states.contains(&pc) {
            return true;
        }
    }

    false
}

#[derive(Debug, Clone)]
struct GameConsole {
    pc: usize,
    acc: i64,
    instructions: Vec<Instruction>,
}

impl GameConsole {
    fn new<R>(program: &mut R) -> Result<GameConsole, Error>
    where
        R: BufRead,
    {
        Ok(GameConsole {
            pc: 0,
            acc: 0,
            instructions: program
                .lines()
                .map(|line| {
                    line.map_err(|e| {
                        Error::InstructionDecodeError(format!("bad instruction: {:?}", e))
                    })
                    .and_then(|line| Instruction::from_str(&line))
                })
                .collect::<Result<Vec<Instruction>, Error>>()?,
        })
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }

    fn pc(&self) -> usize {
        self.pc
    }

    fn acc(&self) -> i64 {
        self.acc
    }

    fn peek(&self) -> Option<&Instruction> {
        self.instructions.get(self.pc)
    }

    fn run_inst(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Acc(arg) => {
                self.acc += *arg as i64;
                self.pc += 1;
            }
            Instruction::Jmp(arg) => {
                if arg.is_positive() {
                    self.pc += *arg as usize;
                } else {
                    self.pc -= arg.abs() as usize;
                }
            }
            Instruction::Nop(_) => {
                self.pc += 1;
            }
        }
    }

    fn run_step(&mut self) {
        if let Some(&inst) = self.peek() {
            self.run_inst(&inst);
        }
    }

    fn run(&mut self) {
        while self.peek().is_some() {
            self.run_step();
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i16),
    Jmp(i16),
    Nop(i16),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let opcode = iter
            .next()
            .ok_or_else(|| Error::InstructionDecodeError(format!("missing opcode: {}", s)))?;
        let arg = iter
            .next()
            .ok_or_else(|| Error::InstructionDecodeError(format!("missing argument: {}", s)))?;
        let arg: i16 = arg
            .parse()
            .map_err(|_| Error::InstructionDecodeError(format!("invalid argument: {}", arg)))?;

        match opcode {
            "acc" => Ok(Instruction::Acc(arg)),
            "jmp" => Ok(Instruction::Jmp(arg)),
            "nop" => Ok(Instruction::Nop(arg)),
            x => Err(Error::InstructionDecodeError(format!(
                "invalid opcode: {}",
                x
            ))),
        }
    }
}

#[derive(Debug, Clone)]
enum Error {
    InstructionDecodeError(String),
}
