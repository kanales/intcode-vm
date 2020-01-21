use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Intcode(pub Vec<i32>);

use std::str::FromStr;
impl FromStr for Intcode {
    type Err = String;

    fn from_str(input: &str) -> Result<Intcode, Self::Err> {
        let res: Result<Vec<i32>, ParseIntError> = input.split(",").map(|x| x.parse()).collect();
        match res {
            Ok(v) => Ok(Intcode(v)),
            Err(_) => Err("Invalid intcode".to_string()),
        }
    }
}

use std::convert::Into;
impl Into<Vec<i32>> for Intcode {
    fn into(self) -> Vec<i32> {
        match self {
            Intcode(c) => c,
        }
    }
}

impl Intcode {
    pub fn replace(&self, noun: i32, verb: i32) -> Intcode {
        let Intcode(arr) = self;
        let mut newarr = arr.clone();
        newarr[1] = noun;
        newarr[2] = verb;
        Intcode(newarr)
    }
}
