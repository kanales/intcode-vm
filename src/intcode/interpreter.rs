use super::process::{Process, ProcessStatus};
use super::Intcode;

pub struct Interpreter {
    process: Process,
}

pub struct IntcodeOut<'a, I: Iterator<Item = i32>> {
    inputs: I,
    interpreter: &'a mut Interpreter,
}

impl<'a, I: Iterator<Item = i32>> Iterator for IntcodeOut<'a, I> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let proc = &mut self.interpreter.process;
        loop {
            match proc.resume() {
                ProcessStatus::Awaiting(_) => {
                    proc.feed(self.inputs.next()?);
                }
                ProcessStatus::Outputting(o) => return Some(o),
                ProcessStatus::Paused => {
                    unreachable!();
                }
                ProcessStatus::Exit => return None,
            }
        }
    }
}

use std::iter::IntoIterator;

impl Interpreter {
    pub fn new(code: Intcode) -> Self {
        Self {
            process: Process::new(code),
        }
    }

    pub fn execute<'a, Input>(&'a mut self, inputs: Input) -> IntcodeOut<'a, Input::IntoIter>
    where
        Input: IntoIterator<Item = i32>,
    {
        let it = inputs.into_iter();
        IntcodeOut {
            inputs: it,
            interpreter: self,
        }
    }
}

#[test]
fn interpreter_test() {
    use std::str::FromStr;
    let input = Intcode::from_str("3,0,3,1,3,2,4,2,4,1,4,0,99").unwrap();
    let mut interpreter = Interpreter::new(input);
    let inputs = vec![1, 2, 3];
    let mut inputs_iter = inputs.into_iter();
    let out = interpreter.execute(&mut inputs_iter);
    let out_vec: Vec<i32> = out.collect();
    assert_eq!(out_vec, vec![3, 2, 1]);
}
