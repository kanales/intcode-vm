use intcode_vm::{Intcode, Interpreter};

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::str::FromStr;
fn get_code(file: &str) -> Result<Intcode, <Intcode as FromStr>::Err> {
    let mut file = File::open(&file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Intcode::from_str(&contents)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected intcode program file as only argument");
    }
    let code: Intcode = match get_code(&args[1]) {
        Ok(code) => code,
        Err(e) => panic!(e),
    };

    let stdin = io::stdin();

    let inputs: Result<Vec<i64>, _> = stdin
        .lock()
        .lines()
        .map(|line| -> Result<i64, std::io::Error> {
            match line?.parse::<i64>() {
                Ok(int) => Ok(int),
                Err(_) => Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
            }
        })
        .collect();

    let mut inputiter = match inputs {
        Ok(is) => is.into_iter(),
        Err(_) => Vec::new().into_iter(),
    };

    let mut interpreter = Interpreter::new(code);
    for out in interpreter.execute(&mut inputiter) {
        println!("{}", out);
    }
}
