#[derive(Debug)]
#[allow(dead_code)]
pub enum Tokens {
    Eof,
    Var(Types),
    Punctuation(Punctuations),
    Keyword(Keywords),
    Operator(Operators),
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum Keywords {
    Begin,
    End,
    If,
    Then,
    Else,
    True,
    False,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Types {
    Int,
    Float,
    String,
    Bool,
    Function,
    Unknown,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Punctuations {
    LParen,
    RParen,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Operators {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,
}

#[derive(Debug)]
pub struct LexerToken {
    pub token: Tokens,
    pub value: Option<String>,
}
