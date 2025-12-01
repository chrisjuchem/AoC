use crate::aoc2019::intcode::ProgramError::{
    BadAddr, BadOpcode, BadParamMode, MissingInput, WriteToImm,
};
use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::ops::Deref;
use std::str::FromStr;

pub enum ParamMode {
    Parameter,
    Immediate,
}

impl TryFrom<i64> for ParamMode {
    type Error = ProgramError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => ParamMode::Parameter,
            1 => ParamMode::Immediate,
            _ => return Err(BadParamMode(value)),
        })
    }
}

enum Value<'a> {
    Ref(&'a mut i64),
    Imm(i64),
}
impl Deref for Value<'_> {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        match self {
            Value::Ref(r) => r,
            Value::Imm(i) => i,
        }
    }
}
impl Value<'_> {
    fn write(&mut self, val: i64) -> R<()> {
        match self {
            Value::Ref(r) => {
                **r = val;
                Ok(())
            }
            Value::Imm(_) => Err(WriteToImm),
        }
    }
}

pub enum OpCode {
    Add,
    Mul,
    ReadInput,
    WriteOutput,
    Halt,
}

impl TryFrom<i64> for OpCode {
    type Error = ProgramError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            3 => OpCode::ReadInput,
            4 => OpCode::WriteOutput,
            99 => OpCode::Halt,
            _ => return Err(BadOpcode(value)),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pc: usize,
    data: Vec<i64>,
    input: VecDeque<i64>,
    output: Vec<i64>,
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .split(",")
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map_err(|_| ())
            .map(Program::new)
    }
}

impl Program {
    fn new(data: Vec<i64>) -> Self {
        Self {
            pc: 0,
            data,
            input: VecDeque::new(),
            output: vec![],
        }
    }

    pub fn with_input(mut self, input: Vec<i64>) -> Self {
        self.input = VecDeque::from(input);
        self
    }

    fn read(&mut self, code: &mut i64) -> R<Value<'_>> {
        self.pc += 1;
        let mode = *code % 10;
        *code = *code / 10;

        let val = self.get(self.pc - 1)?;

        Ok(match ParamMode::try_from(mode).unwrap() {
            ParamMode::Parameter => Value::Ref(self.get_mut(val)?),
            ParamMode::Immediate => Value::Imm(val),
        })
    }

    pub fn step(&mut self) -> R<StepResult> {
        let code = *self.read(&mut 1)?;
        let op = OpCode::try_from(code % 100)?;
        let modes = &mut (code / 100);

        match op {
            OpCode::Add => {
                let a = *self.read(modes)?;
                let b = *self.read(modes)?;
                self.read(modes)?.write(a + b)?;
            }
            OpCode::Mul => {
                let a = *self.read(modes)?;
                let b = *self.read(modes)?;
                self.read(modes)?.write(a * b)?;
            }
            OpCode::ReadInput => {
                let val = self.input.pop_front().ok_or(MissingInput)?;
                let mut x = self.read(modes)?;
                x.write(val)?;
            }
            OpCode::WriteOutput => {
                let x = *self.read(modes)?;
                self.output.push(x);
            }
            OpCode::Halt => return Ok(StepResult::Halt),
        }
        Ok(StepResult::Continue)
    }

    pub fn run(&mut self) -> R<&[i64]> {
        let mut result = Ok(StepResult::Continue);
        loop {
            match result {
                Ok(StepResult::Continue) => result = self.step(),
                Ok(StepResult::Halt) => return Ok(&self.output),
                Err(e) => return Err(e),
            };
        }
    }
    pub fn run_0(&mut self) -> R<i64> {
        match self.run() {
            Ok(_) => self.get(0),
            Err(e) => Err(e),
        }
    }

    pub fn set(&mut self, addr: impl N, val: i64) -> R<()> {
        *self
            .data
            .get_mut(addr.try_into().or(Err(BadAddr))?)
            .ok_or(BadAddr)? = val;
        Ok(())
    }
    pub fn get(&self, addr: impl N) -> R<i64> {
        self.data
            .get(addr.try_into().or(Err(BadAddr))?)
            .copied()
            .ok_or(BadAddr)
    }
    pub fn get_mut(&mut self, addr: impl N) -> R<&mut i64> {
        self.data
            .get_mut(addr.try_into().or(Err(BadAddr))?)
            .ok_or(BadAddr)
    }
}

pub trait N: TryInto<usize, Error: Debug> {}
impl<T> N for T where T: TryInto<usize, Error: Debug> {}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StepResult {
    Continue,
    Halt,
}
#[derive(Debug, Eq, PartialEq)]
pub enum ProgramError {
    BadAddr,
    WriteToImm,
    BadParamMode(i64),
    BadOpcode(i64),
    MissingInput,
}
type R<T> = Result<T, ProgramError>;
