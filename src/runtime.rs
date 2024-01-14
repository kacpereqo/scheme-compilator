use std::collections::HashMap;

use crate::lexer::types::Types;
use crate::parser::{Argument, LiteralVariable};

#[derive(Debug, Clone)]
pub struct Runtime {
    expressions: Vec<Argument>,
    variables: HashMap<String, LiteralVariable>,
}

impl Runtime {
    pub fn new(expressions: Vec<Argument>) -> Self {
        Self {
            expressions: expressions,
            variables: HashMap::new(),
        }
    }
    pub fn define(&mut self, args: Vec<Argument>) -> Option<Argument> {
        let name = match args[0].clone() {
            Argument::LiteralVariable(literal) => literal.value,
            _ => panic!("Unknown argument"),
        };

        let value = match self.eval(args[1].clone()).unwrap() {
            Argument::LiteralVariable(literal) => literal.value,
            _ => panic!("Unknown argument"),
        };

        if self.variables.contains_key(&name) {
            let variable = self.variables.get_mut(&name).unwrap();
            variable.value = value;
        } else {
            self.variables.insert(
                name,
                LiteralVariable {
                    var_type: Types::Float,
                    value: value,
                },
            );
        }

        None
    }

    pub fn display(&mut self, args: Vec<Argument>) -> Option<Argument> {
        for arg in args {
            let value = self.eval(arg);
            match value {
                Some(arg) => match arg {
                    Argument::LiteralVariable(literal) => {
                        let lines = literal.value.split("\\n");
                        for line in lines {
                            println!("{}", line)
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
                    "define" => {
                        runtime.define(expr.arguments.clone());
                    }
                    _ => panic!("Unknown function"),
                },
                Argument::LiteralVariable(_) => panic!("Unknown function"),
            }
        }
    }

    pub fn operator_plus(&mut self, args: Vec<Argument>) -> Option<Argument> {
        let mut sum = 0.0;
        for arg in args {
            let value = self.eval(arg);
            match value {
                Some(arg) => match arg {
                    Argument::LiteralVariable(literal) => {
                        sum += literal.value.parse::<f64>().unwrap();
                    }
                    _ => panic!("Unknown argument"),
                },
                None => (),
            }
        }
        return Some(Argument::LiteralVariable(LiteralVariable {
            var_type: Types::Float,
            value: sum.to_string(),
        }));
    }

    pub fn operator_minus(&mut self, args: Vec<Argument>) -> Option<Argument> {
        let mut difference;
        match self.eval(args[0].clone()).unwrap() {
            Argument::LiteralVariable(literal) => {
                difference = literal.value.parse::<f64>().unwrap();
            }
            _ => panic!("Unknown argument"),
        }
        for arg in args[1..].to_vec() {
            let value = self.eval(arg);
            match value {
                Some(arg) => match arg {
                    Argument::LiteralVariable(literal) => {
                        difference -= literal.value.parse::<f64>().unwrap();
                    }
                    _ => panic!("Unknown argument"),
                },
                None => (),
            }
        }
        return Some(Argument::LiteralVariable(LiteralVariable {
            var_type: Types::Float,
            value: difference.to_string(),
        }));
    }

    pub fn operator_asterisk(&mut self, args: Vec<Argument>) -> Option<Argument> {
        let mut product = 1.0;
        for arg in args {
            let value = self.eval(arg);
            match value {
                Some(arg) => match arg {
                    Argument::LiteralVariable(literal) => {
                        product *= literal.value.parse::<f64>().unwrap();
                    }
                    _ => panic!("Unknown argument"),
                },
                None => (),
            }
        }
        return Some(Argument::LiteralVariable(LiteralVariable {
            var_type: Types::Float,
            value: product.to_string(),
        }));
    }

    pub fn operator_slash(&mut self, args: Vec<Argument>) -> Option<Argument> {
        let mut quotient;

        match self.eval(args[0].clone()).unwrap() {
            Argument::LiteralVariable(literal) => {
                quotient = literal.value.parse::<f64>().unwrap();
            }
            _ => panic!("Unknown argument"),
        }

        for arg in args[1..].to_vec() {
            let value = self.eval(arg);
            match value {
                Some(arg) => match arg {
                    Argument::LiteralVariable(literal) => {
                        quotient /= literal.value.parse::<f64>().unwrap();
                    }
                    _ => panic!("Unknown argument"),
                },
                None => (),
            }
        }
        return Some(Argument::LiteralVariable(LiteralVariable {
            var_type: Types::Float,
            value: quotient.to_string(),
        }));
    }

    // fn equal_operatr(&mut self, args: Vec<Argument>) -> Option<Argument> {
    //     let mut result =

    //     for arg in args {
    //         let value = self.eval(arg);
    //         match value {
    //             Some(arg) => {}
    //             None => (),
    //         }
    //     }
    //     return Some(Argument::LiteralVariable(LiteralVariable {
    //         var_type: Types::Bool,
    //         value: result.value,
    //     }));
    // }

    pub fn newline(&mut self) -> Option<Argument> {
        print!("\n");
        None
    }

    pub fn eval(&mut self, arg: Argument) -> Option<Argument> {
        match &arg {
            Argument::Expression(expr) => match expr.function.as_str() {
                "display" => self.display(expr.arguments.clone()),
                "begin" => self.begin(expr.arguments.clone()),
                "newline" => self.newline(),
                "define" => self.define(expr.arguments.clone()),
                "+" => self.operator_plus(expr.arguments.clone()),
                "*" => self.operator_asterisk(expr.arguments.clone()),
                "-" => self.operator_minus(expr.arguments.clone()),
                "/" => self.operator_slash(expr.arguments.clone()),
                "" => None,
                _ => panic!("Unknown function"),
            },
            Argument::LiteralVariable(_) => match arg {
                Argument::LiteralVariable(literal) => {
                    if self.variables.contains_key(&literal.value) {
                        let variable = self.variables.get(&literal.value).unwrap();
                        return Some(Argument::LiteralVariable(variable.clone()));
                    }
                    return Some(Argument::LiteralVariable(literal));
                }
                _ => panic!("Unknown argument"),
            },
        }
    }
}
