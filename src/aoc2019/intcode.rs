use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Program {
    pc: usize,
    data: Vec<u64>,
}
impl Program {
    pub fn read(&mut self) -> u64 {
        self.pc += 1;
        self.data[self.pc - 1]
    }

    pub fn step(&mut self) -> StepResult {
        let Ok(op) = OpCode::try_from(self.read()) else {
            return StepResult::Error;
        };

        match op {
            OpCode::Add => {
                let a = self.read();
                let b = self.read();
                let i = self.read();

                self.set(i, self.get(a) + self.get(b));

                StepResult::Continue
            }
            OpCode::Mul => {
                let a = self.read() as usize;
                let b = self.read() as usize;
                let i = self.read() as usize;

                self.set(i, self.get(a) * self.get(b));

                StepResult::Continue
            }
            OpCode::Halt => StepResult::Halt,
        }
    }

    pub fn run(&mut self) -> Result<u64, ()> {
        let mut result = StepResult::Continue;
        loop {
            match result {
                StepResult::Continue => result = self.step(),
                StepResult::Halt => return Ok(self.data[0]),
                StepResult::Error => return Err(()),
            };
        }
    }

    pub fn set(&mut self, addr: impl TryInto<usize, Error: Debug>, val: u64) {
        self.data[addr.try_into().unwrap()] = val;
    }
    pub fn get(&self, addr: impl TryInto<usize, Error: Debug>) -> u64 {
        self.data[addr.try_into().unwrap()]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StepResult {
    Continue,
    Halt,
    Error,
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .split(",")
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map_err(|_| ())
            .map(|i| Program { pc: 0, data: i })
    }
}

pub enum OpCode {
    Add,
    Mul,
    Halt,
}

impl TryFrom<u64> for OpCode {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            99 => OpCode::Halt,
            _ => return Err(()),
        })
    }
}
