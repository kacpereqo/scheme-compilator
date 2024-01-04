#[path = "types.rs"]
pub(crate) mod types;

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
            row: 0,
            column: 0,
            ch: ' ',
        }
    }

    fn next_char(&mut self) -> Self {
        if self.position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.position as usize).unwrap();
        }

        self.position += 1;
        self.column = self.position;

        if self.ch == '\n' {
            self.row += 1;
            self.column = 0;
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
        while self.ch.is_alphanumeric() || self.ch == '_' {
            identifier.push(self.ch);
            self.next_char();
        }

        match identifier.as_str() {
            "true" => return "0".to_string(),
            "false " => return "0".to_string(),
            _ => (),
        };
        identifier
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while self.ch.is_numeric() || self.ch == '.' {
            number.push(self.ch);
            self.next_char();
        }
        number
    }

    fn read_number_type(&self, number: String) -> Tokens {
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

    fn read_operator(&self, ch: char) -> Tokens {
        match ch {
            '+' => return Tokens::Operator(Operators::Plus),
            '-' => return Tokens::Operator(Operators::Minus),
            '*' => return Tokens::Operator(Operators::Asterisk),
            '/' => return Tokens::Operator(Operators::Slash),
            _ => panic!("Unknown operator"),
        }
    }

    fn lookup_identifier(&self, identifier: &str) -> Tokens {
        match identifier {
            "begin" => return Tokens::Keyword(Keywords::Begin),
            "end" => return Tokens::Keyword(Keywords::End),
            "if" => return Tokens::Keyword(Keywords::If),
            "then" => return Tokens::Keyword(Keywords::Then),
            "else" => return Tokens::Keyword(Keywords::Else),
            "true" => return Tokens::Var(Types::Bool),
            "false" => return Tokens::Var(Types::Bool),
            _ => return Tokens::Var(Types::Unknown),
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
                        value: Some(value),
                    };
                }
                '0'..='9' => {
                    value = self.read_number();
                    token = self.read_number_type(value.clone());
                    return LexerToken {
                        token,
                        value: Some(value.clone()),
                    };
                }
                '(' | ')' => {
                    token = self.read_punctuation(self.ch);
                    return LexerToken {
                        token,
                        value: Some(self.ch.to_string()),
                    };
                }
                '"' => {
                    value = self.read_string();
                    token = Tokens::Var(Types::String);
                    return LexerToken {
                        token,
                        value: Some(value.clone()),
                    };
                }

                '+' | '-' | '*' | '/' => {
                    token = self.read_operator(self.ch);
                    return LexerToken {
                        token,
                        value: Some(self.ch.to_string()),
                    };
                }
                '#' => {
                    self.skip_comment();
                }
                _ => {
                    return LexerToken {
                        token: Tokens::Eof,
                        value: None,
                    }
                }
            };
        }
    }
}
