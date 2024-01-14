use crate::lexer::types::Types;
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
        let mut difference = 0.0;
        for arg in args {
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

    pub fn operator_slash(&mut self) -> Option<Argument> {}

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
                "+" => self.operator_plus(expr.arguments.clone()),
                "*" => self.operator_asterisk(expr.arguments.clone()),
                "-" => self.operator_minus(),
                "/" => self.operator_slash(),
                "" => None,
                _ => panic!("Unknown function"),
            },
            Argument::LiteralVariable(_) => return Some(arg.clone()),
        }
    }
}
