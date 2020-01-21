// OPCODE
use Opcode::*;
#[derive(Debug, PartialEq)]
pub enum Opcode<A> {
    Add(A, A, A),
    Mul(A, A, A),
    Inp(A),
    Out(A),
    Jnz(A, A),
    Jz(A, A),
    Lt(A, A, A),
    Equ(A, A, A),
    Rbs(A),
    Hlt,
}

use Parameter::*;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Parameter<A> {
    Pos(A),
    Imm(A),
    Rel(A),
}

use std::convert::TryFrom;
impl TryFrom<i64> for Parameter<()> {
    type Error = String;

    fn try_from(item: i64) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(Pos(())),
            1 => Ok(Imm(())),
            2 => Ok(Rel(())),
            m => Err(format!("Unknown mode {:?}", m).to_string()),
        }
    }
}

impl<T> Parameter<T> {
    #[allow(dead_code)]
    pub fn map<B, F: Fn(&T) -> B>(&self, f: F) -> Parameter<B> {
        match self {
            Pos(x) => Pos(f(x)),
            Imm(x) => Imm(f(x)),
            Rel(x) => Rel(f(x)),
        }
    }

    pub fn try_map<B, E, F>(&self, f: F) -> Result<Parameter<B>, E>
    where
        F: Fn(&T) -> Result<B, E>,
    {
        Ok(match self {
            Pos(x) => Pos(f(x)?),
            Imm(x) => Imm(f(x)?),
            Rel(x) => Rel(f(x)?),
        })
    }
}

fn digit(x: i64, i: u32) -> i64 {
    x / 10_i64.pow(i) % 10
}

impl<T> Opcode<T> {
    pub fn mut_map<B>(&self, f: &mut impl FnMut(&T) -> B) -> Opcode<B> {
        match self {
            Add(a, b, c) => Add(f(a), f(b), f(c)),
            Mul(a, b, c) => Mul(f(a), f(b), f(c)),
            Out(a) => Out(f(a)),
            Inp(a) => Inp(f(a)),
            Jnz(a, b) => Jnz(f(a), f(b)),
            Jz(a, b) => Jz(f(a), f(b)),
            Lt(a, b, c) => Lt(f(a), f(b), f(c)),
            Equ(a, b, c) => Equ(f(a), f(b), f(c)),
            Rbs(a) => Rbs(f(a)),
            _ => Hlt,
        }
    }

    #[allow(dead_code)]
    fn map<B, F: Fn(&T) -> B>(&self, f: F) -> Opcode<B> {
        match self {
            Add(a, b, c) => Add(f(a), f(b), f(c)),
            Mul(a, b, c) => Mul(f(a), f(b), f(c)),
            Out(a) => Out(f(a)),
            Inp(a) => Inp(f(a)),
            Jnz(a, b) => Jnz(f(a), f(b)),
            Jz(a, b) => Jz(f(a), f(b)),
            Lt(a, b, c) => Lt(f(a), f(b), f(c)),
            Equ(a, b, c) => Equ(f(a), f(b), f(c)),
            _ => Hlt,
        }
    }
}

use std::convert::TryInto;
impl TryFrom<i64> for Opcode<Parameter<()>> {
    type Error = String;

    fn try_from(x: i64) -> Result<Opcode<Parameter<()>>, Self::Error> {
        let p = |i| digit(x / 100, i).try_into();

        let a: Parameter<()> = p(0)?;
        let b: Parameter<()> = p(1)?;
        let c: Parameter<()> = p(2)?;
        let op = match x % 100 {
            1 => Add(a, b, c),
            2 => Mul(a, b, c),
            3 => Inp(c),
            4 => Out(a),
            5 => Jnz(a, b),
            6 => Jz(a, b),
            7 => Lt(a, b, c),
            8 => Equ(a, b, c),
            9 => Rbs(a),
            99 => Hlt,
            o => Err(format!("Unknown operation {:?}", o).to_string())?,
        };
        Ok(op)
    }
}
