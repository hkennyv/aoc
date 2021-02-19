#![allow(clippy::upper_case_acronyms)]

use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OperationEnum {
    ACC,
    JMP,
    NOP,
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction {
    pub operation: OperationEnum,
    pub argument: i32,
}

#[derive(Debug)]
pub struct VM {
    acc: i32,
    idx: usize,
    instructions: Vec<Instruction>,
    has_run: HashSet<usize>,
}

impl VM {
    /// Creates a new VM instance given a vector of instructions. It
    /// initializes the acc, idx to 0 with an empty has_run set.
    pub fn new(instructions: Vec<Instruction>) -> VM {
        VM {
            acc: 0,
            idx: 0,
            instructions,
            has_run: HashSet::new(),
        }
    }

    /// Runs the VM. It will terminate once an instruction that has already
    /// run is reached
    pub fn run(&mut self) -> i32 {
        loop {
            if self.has_run.contains(&self.idx) {
                break;
            }

            let instruction = self.instructions[self.idx];
            self.execute_instruction(&instruction);
        }

        self.acc
    }

    /// Executes a single instruction operation with its respective argument
    fn execute_instruction(&mut self, instruction: &Instruction) {
        // mark the execution as run
        self.has_run.insert(self.idx);

        match instruction.operation {
            OperationEnum::ACC => {
                self.acc += instruction.argument;
                self.idx += 1;
            }
            OperationEnum::JMP => {
                self.idx = (self.idx as i32 + instruction.argument) as usize;
            }
            OperationEnum::NOP => {
                self.idx += 1;
            }
        }
    }

    /// Attempts to change the nop and jmp instructions until it finds a
    /// vector of instructions that successfully terminates
    pub fn fix_instructions(&mut self) -> i32 {
        let nop_and_jmps: Vec<usize> = self
            .instructions
            .iter()
            .enumerate()
            .filter(|(_, instr)| {
                instr.operation == OperationEnum::JMP || instr.operation == OperationEnum::NOP
            })
            .map(|(idx, _)| idx)
            .collect();

        for idx in nop_and_jmps {
            let instruction = self.instructions[idx];

            // flip instruction operation
            match instruction.operation {
                OperationEnum::JMP => {
                    self.instructions[idx].operation = OperationEnum::NOP;
                }
                OperationEnum::NOP => {
                    self.instructions[idx].operation = OperationEnum::JMP;
                }
                _ => {}
            }

            // see if the sequence terminates
            // if yes, return the accumulator, otherwise flip the operation
            // back to the original, reset, and continue looping
            if self.does_terminate() {
                return self.acc;
            } else {
                match instruction.operation {
                    OperationEnum::JMP => {
                        self.instructions[idx].operation = OperationEnum::JMP;
                    }
                    OperationEnum::NOP => {
                        self.instructions[idx].operation = OperationEnum::NOP;
                    }
                    _ => {}
                }

                self.acc = 0;
                self.idx = 0;
                self.has_run.clear();
            }
        }

        0
    }

    /// returns true when a sequence of instructions successfully terminates
    fn does_terminate(&mut self) -> bool {
        loop {
            if self.has_run.contains(&self.idx) {
                return false;
            }

            let instruction = self.instructions[self.idx];
            self.execute_instruction(&instruction);

            if self.idx == self.instructions.len() {
                return true;
            }
        }
    }
}
