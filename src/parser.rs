use crate::lexer::types::LexerToken;
use crate::lexer::types::Punctuations;
use crate::lexer::types::Tokens;
use crate::lexer::types::Types;

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<LexerToken>,
    position: usize,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Argument {
    Expression(Expression),
    LiteralVariable(LiteralVariable),
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Expression {
    pub function: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]

pub struct LiteralVariable {
    pub var_type: Types,
    pub value: String,
}

impl Parser {
    pub fn new(tokens: Vec<LexerToken>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    #[allow(dead_code)]
    fn next_token(&mut self) -> LexerToken {
        if self.position >= self.tokens.len() {
            return LexerToken {
                var_type: Types::Unknown,
                token: Tokens::Eof,
                value: None,
            };
        }

        let token = self.tokens[self.position].clone();
        self.position += 1;
        token
    }

    fn find_args(&mut self, token: LexerToken) -> Argument {
        {
            let mut arguments = Vec::new();
            loop {
                let peeked = self.next_token();
                // println!("{:?}", peeked);

                match peeked.token {
                    Tokens::Punctuation(Punctuations::RParen) => {
                        return Argument::Expression(Expression {
                            function: token.value.unwrap(),
                            arguments: arguments,
                        });
                    }
                    Tokens::Punctuation(Punctuations::LParen) => {
                        arguments.push(self.parse_expression());
                    }
                    Tokens::Eof => {
                        return Argument::Expression(Expression {
                            function: token.value.unwrap(),
                            arguments: arguments,
                        });
                    }
                    Tokens::Var(_) => arguments.push(Argument::LiteralVariable(LiteralVariable {
                        var_type: peeked.var_type,
                        value: peeked.value.unwrap(),
                    })),
                    Tokens::Keyword(_) => todo!(),
                    Tokens::Operator(_) => todo!(),
                }
            }
        }
    }

    #[allow(dead_code)]
    fn peek_token(&mut self) -> LexerToken {
        self.tokens[self.position].clone()
    }

    pub fn parse(&mut self) -> Vec<Argument> {
        let mut expressions: Vec<Argument> = Vec::new();
        while self.position < self.tokens.len() {
            let expr = self.parse_expression();
            expressions.push(expr);
        }
        expressions
    }

    fn parse_expression(&mut self) -> Argument {
        let token = self.next_token();
        match token.token {
            Tokens::Punctuation(Punctuations::LParen) => {
                let function = self.next_token();

                match function.token {
                    Tokens::Keyword(_) => {
                        let mut arguments = Vec::new();
                        loop {
                            let peeked = self.next_token();
                            match peeked.token {
                                Tokens::Punctuation(Punctuations::RParen) => {
                                    return Argument::Expression(Expression {
                                        function: function.value.unwrap(),
                                        arguments: arguments,
                                    });
                                }
                                Tokens::Punctuation(Punctuations::LParen) => {
                                    arguments.push(self.parse_expression());
                                }
                                Tokens::Eof => {
                                    return Argument::Expression(Expression {
                                        function: function.value.unwrap(),
                                        arguments: arguments,
                                    });
                                }
                                Tokens::Var(_) => {
                                    arguments.push(Argument::LiteralVariable(LiteralVariable {
                                        var_type: peeked.var_type,
                                        value: peeked.value.unwrap(),
                                    }))
                                }
                                Tokens::Keyword(_) => todo!(),
                                Tokens::Operator(_) => todo!(),
                            }
                        }
                    }
                    _ => panic!("Expected LParen"),
                }
            }
            Tokens::Keyword(_) => self.find_args(token),
            Tokens::Operator(_) => self.find_args(token),
            Tokens::Punctuation(Punctuations::RParen) => {
                return Argument::Expression(Expression {
                    function: "".to_string(),
                    arguments: Vec::new(),
                });
            }
            Tokens::Eof => todo!(),
            Tokens::Var(_) => {
                return Argument::LiteralVariable(LiteralVariable {
                    var_type: token.var_type,
                    value: token.value.unwrap(),
                })
            }
        }
    }
}
