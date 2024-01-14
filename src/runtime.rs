use std::collections::HashMap;

use crate::lexer::types::Types;
use crate::parser::{Argument, LiteralVariable};

use std::io;
use std::io::Write;

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

        let value;
        let var_type;
        match self.eval(args[1].clone()).unwrap() {
            Argument::LiteralVariable(literal) => {
                value = literal.value;
                var_type = literal.var_type;
            }
            _ => panic!("Unknown argument"),
        };

        if self.variables.contains_key(&name) {
            let variable = self.variables.get_mut(&name).unwrap();
            variable.value = value;
        } else {
            self.variables.insert(
                name,
                LiteralVariable {
                    var_type: var_type,
                    value: value,
                },
            );
        }

        None
    }
    pub fn read_line(&mut self) -> Option<Argument> {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input.pop();

        return Some(Argument::LiteralVariable(LiteralVariable {
            var_type: Types::String,
            value: input.to_string(),
        }));
    }

    pub fn read(&mut self) -> Option<Argument> {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input.pop();
        if !input.chars().all(char::is_numeric) {
            panic!("Input is not a number")
        }

        if input.contains(".") {
            return Some(Argument::LiteralVariable(LiteralVariable {
                var_type: Types::Float,
                value: input,
            }));
        } else {
            return Some(Argument::LiteralVariable(LiteralVariable {
                var_type: Types::Int,
                value: input,
            }));
        }
    }

    pub fn display(&mut self, args: Vec<Argument>) -> Option<Argument> {
        let mut result = String::new();
        for arg in args {
            let value = self.eval(arg);
            match value {
                Some(arg) => match arg {
                    Argument::LiteralVariable(mut literal) => {
                        if literal.value.ends_with("\n") {
                            literal.value.pop();
                        }
                        result.push_str(format!("{}", literal.value).as_str());
                    }
                    _ => panic!("Unknown argument"),
                },
                None => print!("None"),
            }
        }
        let vec = result.split("\\n").collect::<Vec<&str>>();
        for line in &vec {
            if vec.len() < 2 {
                print!("{} ", line);
                io::stdout().flush().unwrap();
            } else {
                if line.to_string() == "" {
                    continue;
                }
                println!("{}", line);
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

        match value1 {
            Argument::LiteralVariable(literal) => match value2 {
                Argument::LiteralVariable(literal2) => {
                    let equal = literal.value == literal2.value
                        && match (literal.var_type, literal2.var_type) {
                            (Types::Int, Types::Int)
                            | (Types::Float, Types::Int)
                            | (Types::Int, Types::Float)
                            | (Types::Float, Types::Float)
                            | (Types::String, Types::String) => true,
                            _ => false,
                        };
                    return Some(Argument::LiteralVariable(LiteralVariable {
                        var_type: Types::Int,
                        value: if equal {
                            "1".to_string()
                        } else {
                            "0".to_string()
                        },
                    }));
                }
                _ => panic!("Unknown argument"),
            },
            _ => panic!("Unknown argument"),
        }
        None
    }

    fn operator_ne(&mut self, args: Vec<Argument>) -> Option<Argument> {
        None
    }

    fn if_statement(&mut self, args: Vec<Argument>) -> Option<Argument> {
        let condition = self.eval(args[0].clone()).unwrap();
        let true_branch = args[1].clone();
        if args.len() == 2 {
            match condition {
                Argument::LiteralVariable(literal) => {
                    if literal.value == "1" {
                        return self.eval(true_branch);
                    }
                }
                _ => panic!("Unknown argument"),
            }
            return None;
        }
        let false_branch = args[2].clone();

        match condition {
            Argument::LiteralVariable(literal) => {
                if literal.value == "1" {
                    return self.eval(true_branch);
                } else {
                    return self.eval(false_branch);
                }
            }
            _ => panic!("Unknown argument"),
        }
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
                "read-line" => self.read_line(),
                "if" => self.if_statement(expr.arguments.clone()),
                "true" | "#t" => Some(Argument::LiteralVariable(LiteralVariable {
                    var_type: Types::Int,
                    value: "1".to_string(),
                })),
                "false" | "#f" => Some(Argument::LiteralVariable(LiteralVariable {
                    var_type: Types::Int,
                    value: "0".to_string(),
                })),
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
