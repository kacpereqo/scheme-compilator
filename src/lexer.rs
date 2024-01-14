#[path = "types.rs"]
pub(crate) mod types;

use core::panic;

use types::*;

#[derive(Debug, Clone)]
pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub row: usize,
    pub column: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input: &String) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
            row: 1,
            column: 1,
            ch: ' ',
        }
    }

    fn peek_char(&self) -> char {
        if self.position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.position).unwrap()
        }
    }

    fn next_char(&mut self) -> Self {
        if self.ch == '\0' {
            panic!("End of file");
        }
        if self.position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.position).unwrap();
        }

        self.position += 1;
        self.column += 1;

        if self.ch == '\n' {
            self.row += 1;
            self.column = 1;
        }
        self.clone()
    }

    fn skip_whitespace(&mut self) -> Self {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.next_char();
        }
        self.clone()
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while self.peek_char().is_alphanumeric() {
            identifier.push(self.ch);
            self.next_char();
        }
        identifier.push(self.ch);

        match identifier.as_str() {
            "true" => return "0".to_string(),
            "false " => return "0".to_string(),
            _ => (),
        };
        identifier
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        number.push(self.ch);

        let mut peeked = self.peek_char();

        while peeked.is_numeric() || peeked == '.' {
            self.next_char();
            number.push(self.ch);
            peeked = self.peek_char();
        }
        number
    }

    fn read_numbervar_type(&self, number: String) -> Tokens {
        if number.contains(".") {
            return Tokens::Var(Types::Float);
        }
        return Tokens::Var(Types::Int);
    }

    fn read_punctuation(&self, ch: char) -> Tokens {
        match ch {
            '(' => return Tokens::Punctuation(Punctuations::LParen),
            ')' => return Tokens::Punctuation(Punctuations::RParen),
            _ => panic!("Unknown punctuation"),
        }
    }

    fn read_single_operator(&self, ch: char) -> Tokens {
        match ch {
            '+' => return Tokens::Operator(Operators::Plus),
            '-' => return Tokens::Operator(Operators::Minus),
            '*' => return Tokens::Operator(Operators::Asterisk),
            '/' => return Tokens::Operator(Operators::Slash),
            '<' => return Tokens::Operator(Operators::Lt),
            '>' => return Tokens::Operator(Operators::Gt),
            '=' => return Tokens::Operator(Operators::Eq),
            '%' => return Tokens::Operator(Operators::Percent),
            _ => panic!("Unknown operator"),
        }
    }

    fn read_double_operator(&self, ch: &str) -> Tokens {
        match ch {
            "<=" => return Tokens::Operator(Operators::Le),
            ">=" => return Tokens::Operator(Operators::Ge),
            "!=" => return Tokens::Operator(Operators::Ne),
            _ => panic!("Unknown operator"),
        }
    }

    fn get_numbervar_type(&mut self, number: String) -> Types {
        if number.contains(".") {
            return Types::Float;
        }
        return Types::Int;
    }

    fn lookup_identifier(&self, identifier: &str) -> Tokens {
        match identifier {
            "newline" => Tokens::Keyword(Keywords::Newline),
            "display" => Tokens::Keyword(Keywords::Display),
            "define" => Tokens::Keyword(Keywords::Define),
            "read" => Tokens::Keyword(Keywords::Read),
            "begin" => Tokens::Keyword(Keywords::Begin),
            "false" => Tokens::Var(Types::Bool),
            "then" => Tokens::Keyword(Keywords::Then),
            "else" => Tokens::Keyword(Keywords::Else),
            "true" => Tokens::Var(Types::Bool),
            "end" => Tokens::Keyword(Keywords::End),
            "if" => Tokens::Keyword(Keywords::If),
            _ => Tokens::Var(Types::Unknown),
        }
    }

    fn skip_comment(&mut self) {
        while self.ch != '\n' {
            self.next_char();
        }

        self.skip_whitespace();
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.next_char();

        while self.ch != '"' {
            string.push(self.ch);
            self.next_char();
        }

        string
    }

    pub fn next_token(&mut self) -> LexerToken {
        let token: Tokens;
        let value: String;
        self.next_char();

        loop {
            match self.ch {
                ' ' | '\t' | '\n' | '\r' => {
                    self.skip_whitespace();
                }
                'a'..='z' | 'A'..='Z' => {
                    value = self.read_identifier();
                    token = self.lookup_identifier(&value);
                    return LexerToken {
                        token,
                        var_type: Types::Keyword,
                        value: Some(value),
                    };
                }
                '0'..='9' => {
                    value = self.read_number();
                    let var_type = self.get_numbervar_type(value.clone());
                    token = self.read_numbervar_type(value.clone());
                    return LexerToken {
                        token,
                        var_type,
                        value: Some(value.clone()),
                    };
                }
                '(' | ')' => {
                    token = self.read_punctuation(self.ch);
                    return LexerToken {
                        token,
                        var_type: Types::Unknown,
                        value: Some(self.ch.to_string()),
                    };
                }
                '"' => {
                    value = self.read_string();
                    token = Tokens::Var(Types::String);
                    return LexerToken {
                        var_type: Types::String,
                        token,
                        value: Some(value.clone()),
                    };
                }

                '+' | '-' | '*' | '/' | '=' | '>' | '<' | '%' => {
                    let peeked = self.peek_char();

                    match peeked {
                        '=' => {
                            let mut double_operator = String::new();
                            double_operator.push(self.ch);
                            double_operator.push(peeked);
                            self.next_char();

                            token = self.read_double_operator(&double_operator);
                            return LexerToken {
                                var_type: Types::Unknown,
                                token,
                                value: Some(double_operator),
                            };
                        }
                        _ => {
                            token = self.read_single_operator(self.ch);
                            return LexerToken {
                                var_type: Types::Unknown,
                                token,
                                value: Some(self.ch.to_string()),
                            };
                        }
                    };
                }
                ';' => {
                    self.skip_comment();
                }
                _ => {
                    return LexerToken {
                        var_type: Types::Unknown,
                        token: Tokens::Eof,
                        value: None,
                    }
                }
            };
        }
    }
}
