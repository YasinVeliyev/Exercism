pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
use std::collections::VecDeque;

pub struct Forth {
    s: Vec<Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self { s: Vec::new() }
    }

    pub fn stack(&self) -> &[Value] {
        &self.s
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut v: Vec<i32> = vec![];
        let binding = input.to_ascii_lowercase();
        println!("'{}'", binding);
        for i in binding.split(" ") {
            if ["+", "*", "-", "/"].contains(&i) {
                match do_math(&mut v, i) {
                    Ok(_) => continue,
                    Err(err) => return Err(err),
                }
            } else {
                match i {
                    "drop" => {
                        if v.len() < 1 {
                            return Err(Error::StackUnderflow);
                        }
                        v.pop();
                    }
                    "dup" => {
                        if v.len() == 0 {
                            return Err(Error::StackUnderflow);
                        }
                        let l = v.pop().unwrap();
                        v.append(&mut vec![l, l]);
                    }
                    "swap" => {
                        if v.len() < 2 {
                            return Err(Error::StackUnderflow);
                        }
                        let l = v.len();
                        v.swap(l - 2, l - 1)
                    }
                    "over" => {
                        if v.len() < 2 {
                            return Err(Error::StackUnderflow);
                        }
                        v.push(v[v.len() - 2])
                    }
                    _ => {
                        println!("{}", i);
                        match i.parse::<i32>() {
                            Ok(s) => v.push(s),
                            Err(_) => return Err(Error::InvalidWord),
                        }
                    }
                }
            }
        }
        self.s = v;
        Ok(())
    }
}

fn do_math(v: &mut Vec<i32>, operator: &str) -> Result {
    println!("{:?} {}", v, operator);
    if v.len() >= 2 {
        match operator {
            "*" => {
                let second = v.pop().unwrap();
                let first = v.pop().unwrap();
                v.push(first * second);
            }
            "-" => {
                let second = v.pop().unwrap();
                let first = v.pop().unwrap();
                v.push(first - second);
            }
            "+" => {
                let second = v.pop().unwrap();
                let first = v.pop().unwrap();
                v.push(first + second);
            }
            "/" => {
                let second = v.pop().unwrap();
                let first = v.pop().unwrap();
                if second == 0 {
                    println!("{:?}", second);
                    return Err(Error::DivisionByZero);
                } else {
                    v.push(first / second);
                }
            }
            _ => {
                return Err(Error::UnknownWord);
            }
        };
        Ok(())
    } else {
        return Err(Error::StackUnderflow);
    }
}
