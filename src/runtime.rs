use std::{fmt::Error, os::raw};

use crate::parser::{Argument, LiteralVariable};

#[derive(Debug, Clone)]
pub struct Runtime {
    expressions: Vec<Argument>,
}

impl Runtime {
    pub fn new(expressions: Vec<Argument>) -> Self {
        Self {
            expressions: expressions,
        }
    }

    pub fn display(&mut self, args: Vec<Argument>) -> Option<Argument> {
        for arg in args {
            let value = self.eval(arg);
            match value {
                Some(arg) => match arg {
                    Argument::LiteralVariable(literal) => {
                        let lines = literal.value.split("\\n");
                        for line in lines {
                            println!("{}", line);
                        }
                    }
                    _ => panic!("Unknown argument"),
                },
                None => print!("None"),
            }
        }
        None
    }

    pub fn begin(&mut self, args: Vec<Argument>) -> Option<Argument> {
        for arg in args {
            self.eval(arg);
        }

        None
    }

    pub fn run(&mut self) {
        for expression in &self.expressions {
            let mut runtime = self.clone();
            match expression {
                Argument::Expression(expr) => match expr.function.as_str() {
                    "display" => {
                        runtime.display(expr.arguments.clone());
                    }
                    "begin" => {
                        runtime.begin(expr.arguments.clone());
                    }
                    _ => panic!("Unknown function"),
                },
                Argument::LiteralVariable(_) => panic!("Unknown function"),
            }
        }
    }

    // pub fn operator_plus() {}

    pub fn eval(&mut self, arg: Argument) -> Option<Argument> {
        match &arg {
            Argument::Expression(expr) => match expr.function.as_str() {
                "display" => self.display(expr.arguments.clone()),
                "begin" => self.begin(expr.arguments.clone()),
                "+" => {
                    println!("Additioaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaan");
                    None
                }
                "*" => {
                    println!("Multiplication");
                    None
                }
                "newline" => {
                    print!("\n");
                    None
                }
                "" => None,
                _ => panic!("Unknown function"),
            },
            Argument::LiteralVariable(_) => return Some(arg.clone()),
        }
    }
}
