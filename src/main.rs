use intcode_vm::intcode::{Intcode, Interpreter};

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

    let inputs: Result<Vec<i32>, _> = stdin
        .lock()
        .lines()
        .map(|line| -> Result<i32, std::io::Error> { Ok(line?.parse::<i32>().unwrap()) })
        .collect();

    if let Ok(is) = inputs {
        let mut interpreter = Interpreter::new(code);
        for out in interpreter.execute(&mut is.into_iter()) {
            println!("{}", out);
        }
    }
}
