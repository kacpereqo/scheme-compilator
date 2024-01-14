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

    pub fn read(&mut self) -> Option<Argument> {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        Some(Argument::LiteralVariable(LiteralVariable {
            var_type: Types::String,
            value: input,
        }))
    }

    pub fn display(&mut self, args: Vec<Argument>) -> Option<Argument> {
        let mut result = String::new();
        for arg in args {
            let value = self.eval(arg);
            match value {
                Some(arg) => match arg {
                    Argument::LiteralVariable(literal) => {
                        result.push_str(format!("{}", literal.value).as_str());
                    }
                    _ => panic!("Unknown argument"),
                },
                None => print!("None"),
            }
        }
        // vec by splitting on \n
        let vec = result.split("\\n");
        for line in vec {
            print!("{}\n", line);
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
                    "read" => {
                        runtime.read();
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

    pub fn newline(&mut self) -> Option<Argument> {
        println!();
        None
    }

    fn operator_percent(&mut self, args: Vec<Argument>) -> Option<Argument> {
        let value1 = self.eval(args[0].clone()).unwrap();
        let value2 = self.eval(args[1].clone()).unwrap();

        match value1 {
            Argument::LiteralVariable(literal) => match value2 {
                Argument::LiteralVariable(literal2) => {
                    let modulo = literal.value.parse::<f64>().unwrap()
                        % literal2.value.parse::<f64>().unwrap();
                    return Some(Argument::LiteralVariable(LiteralVariable {
                        var_type: Types::Float,
                        value: modulo.to_string(),
                    }));
                }
                _ => panic!("Unknown argument"),
            },
            _ => panic!("Unknown argument"),
        }
    }

    fn operator_lt(&mut self, args: Vec<Argument>) -> Option<Argument> {
        None
    }

    fn operator_le(&mut self, args: Vec<Argument>) -> Option<Argument> {
        None
    }

    fn operator_gt(&mut self, args: Vec<Argument>) -> Option<Argument> {
        None
    }

    fn operator_ge(&mut self, args: Vec<Argument>) -> Option<Argument> {
        None
    }

    fn operator_eq(&mut self, args: Vec<Argument>) -> Option<Argument> {
        let value1 = self.eval(args[0].clone()).unwrap();
        let value2 = self.eval(args[1].clone()).unwrap();

        // return true if value is equal and types are equal
        None
    }

    fn operator_ne(&mut self, args: Vec<Argument>) -> Option<Argument> {
        None
    }

    pub fn eval(&mut self, arg: Argument) -> Option<Argument> {
        match &arg {
            Argument::Expression(expr) => match expr.function.as_str() {
                // keywords
                "display" => self.display(expr.arguments.clone()),
                "begin" => self.begin(expr.arguments.clone()),
                "newline" => self.newline(),
                "define" => self.define(expr.arguments.clone()),
                "read" => self.read(),
                // operators
                "+" => self.operator_plus(expr.arguments.clone()),
                "*" => self.operator_asterisk(expr.arguments.clone()),
                "-" => self.operator_minus(expr.arguments.clone()),
                "/" => self.operator_slash(expr.arguments.clone()),
                "<" => self.operator_lt(expr.arguments.clone()),
                "<=" => self.operator_le(expr.arguments.clone()),
                ">" => self.operator_gt(expr.arguments.clone()),
                ">=" => self.operator_ge(expr.arguments.clone()),
                "=" => self.operator_eq(expr.arguments.clone()),
                "!=" => self.operator_ne(expr.arguments.clone()),
                "%" => self.operator_percent(expr.arguments.clone()),
                "" => None,
                // unknown
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
