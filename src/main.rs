use intcode_vm::intcode::{Intcode, Interpreter};

fn main() {
    use std::str::FromStr;
    let input = Intcode::from_str("3,0,3,1,3,2,4,2,4,1,4,0,99").unwrap();
    let mut interpreter = Interpreter::new(input);
    let inputs = vec![1, 2, 3];
    let mut inputs_iter = inputs.into_iter();
    let out = interpreter.execute(&mut inputs_iter);
    for el in out {
        println!("{}", el);
    }
}
