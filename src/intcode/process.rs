use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fmt;

use super::intcode::Intcode;
use super::opcode::{Opcode, Parameter};

#[derive(Debug, Clone)]
pub struct Process {
    pc: usize,
    intcode: Vec<i32>,
    memory: BTreeMap<i32, i32>,
    relbase: i32,
    status: ProcessStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    Paused,
    Outputting(i32),
    Awaiting(Parameter<i32>),
    Exit,
}
use ProcessStatus::*;

impl ProcessStatus {}

impl Process {
    pub fn new(code: Intcode) -> Self {
        let v: Vec<i32> = code.into();
        Process {
            pc: 0,
            intcode: v,
            status: Paused,
            memory: BTreeMap::new(),
            relbase: 0,
        }
    }

    fn set(&mut self, param: &Parameter<i32>, value: i32) -> Result<(), String> {
        match param {
            Parameter::Pos(key) => Ok({
                self.memory.insert(*key, value);
            }),
            Parameter::Rel(x) => Ok({
                let key = self.relbase + x;
                self.memory.insert(key, value);
            }),
            Parameter::Imm(_) => Err("Can't set to immediate value.".to_owned()),
        }
    }

    // fn try_set(&mut self, param: &Parameter<i32>, value: i32) -> Result<(), String> {
    //     match *param {
    //         Parameter::Imm(_) => Err("Can't set to immediate value.".to_owned()),
    //         Parameter::Pos(p) => {
    //             let x: usize = p
    //                 .try_into()
    //                 .map_err(|_| format!("Invalid index from {:?}", param).to_owned())?;
    //             Ok(self.intcode[x] = value)
    //         }
    //     }
    // }

    fn get(&self, param: &Parameter<i32>) -> Result<i32, String> {
        match *param {
            Parameter::Imm(x) => Ok(x),
            // TODO join Pos and Rel
            Parameter::Pos(p) => {
                if let Some(x) = self.memory.get(&p) {
                    Ok(*x)
                } else {
                    let x: usize = p
                        .try_into()
                        .map_err(|_| format!("Invalid index from {:?}", param).to_owned())?;
                    Ok(self.intcode[x])
                }
            }
            Parameter::Rel(rel_p) => {
                let p = self.relbase + rel_p;
                if let Some(x) = self.memory.get(&p) {
                    Ok(*x)
                } else {
                    let x: usize = p
                        .try_into()
                        .map_err(|_| format!("Invalid index from {:?}", param).to_owned())?;
                    Ok(self.intcode[x])
                }
            }
        }
    }

    fn jmp(&mut self, pos: usize) {
        self.pc = pos;
    }

    fn inc(&mut self, steps: usize) {
        self.pc += steps;
    }

    fn inc_setter<'a>(&'a self) -> Box<dyn FnMut(&Parameter<()>) -> Parameter<i32> + 'a> {
        let mut i: i32 = 0;
        Box::new(move |m| {
            i += 1;
            let pc: Result<i32, _> = self.pc.try_into();
            match pc {
                Ok(position) => {
                    let value = self.get(&Parameter::Imm(position + i)).unwrap();
                    m.map(|_| value)
                }
                Err(_) => unreachable!(),
            }
        })
    }

    fn populate(&self, code: Opcode<Parameter<()>>) -> Opcode<Parameter<i32>> {
        code.mut_map(&mut self.inc_setter())
    }

    fn current(&self) -> Result<Opcode<Parameter<i32>>, String> {
        let code: Opcode<Parameter<()>> = self.intcode[self.pc].try_into()?;
        let op = self.populate(code);
        Ok(op)
    }

    fn eval(&mut self) -> Evaluation {
        match self.eval_inner() {
            Ok(ev) => ev,
            Err(s) => EvaluationError(s),
        }
    }

    pub fn resume(&mut self) -> ProcessStatus {
        let ev = match self.status {
            Paused | Outputting(_) => self.eval(),
            // Do nothing if not paused
            _ => return self.status,
        };
        match ev {
            Input(dest) => self.status = Awaiting(dest),
            Output(o) => self.status = Outputting(o),
            EvaluationError(s) => {
                println!("{}", s);
                self.status = Exit
            }
            Halt => self.status = Exit,
        };
        self.status
    }

    pub fn feed(&mut self, input: i32) -> ProcessStatus {
        match self.status {
            Awaiting(dest) => {
                self.set(&dest, input).unwrap();
                self.status = Paused;
                self.status
            }
            _ => panic!(format!("Trying to feed {:?}", self.status)),
        }
    }

    pub fn head(&self) -> i32 {
        self.intcode[0]
    }

    fn eval_inner(&mut self) -> Result<Evaluation, String> {
        use Opcode::*;
        let curr = self.current()?;
        let ev = match curr {
            Add(a, b, c) => {
                self.set(&c, self.get(&a)? + self.get(&b)?)?;
                self.inc(4);
                self.eval()
            }
            Mul(a, b, c) => {
                let p1 = self.get(&a)?;
                let p2 = self.get(&b)?;
                self.set(&c, p1 * p2)?;
                self.inc(4);
                self.eval()
            }
            Out(a) => {
                self.inc(2);
                Output(self.get(&a)?)
            }
            Inp(a) => {
                self.inc(2);
                Input(a.try_map(|&x| x.try_into()).unwrap())
            }
            Jnz(a, b) => {
                if self.get(&a)? != 0 {
                    self.jmp(self.get(&b)?.try_into().unwrap()); // TODO fix this
                } else {
                    self.inc(3);
                }
                self.eval()
            }
            Jz(a, b) => {
                if self.get(&a)? == 0 {
                    self.jmp(self.get(&b)?.try_into().unwrap()); // TODO fix this
                } else {
                    self.inc(3);
                }
                self.eval()
            }
            Lt(a, b, c) => {
                self.set(&c, if self.get(&a) < self.get(&b) { 1 } else { 0 })?;
                self.inc(4);
                self.eval()
            }
            Equ(a, b, c) => {
                self.set(&c, if self.get(&a) == self.get(&b) { 1 } else { 0 })?;
                self.inc(4);
                self.eval()
            }
            Rbs(a) => {
                let inc = self.get(&a).unwrap();
                self.relbase += inc;
                self.inc(2);
                self.eval()
            }
            Hlt => Halt,
        };
        Ok(ev)
    }

    pub fn status(&self) -> ProcessStatus {
        self.status
    }
}

use Evaluation::*;
pub enum Evaluation {
    Input(Parameter<i32>),
    Output(i32),
    Halt,
    EvaluationError(String),
}

impl<'a> fmt::Debug for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Input(dest) => write!(f, "Input({:?})", dest),
            Output(o) => write!(f, "Output({})", o),
            Halt => write!(f, "Halt"),
            EvaluationError(err) => write!(f, "EvaluationError {}", err),
        }
    }
}

pub trait Runnable {
    fn run(&mut self, input: i32) -> Option<i32>;
}
