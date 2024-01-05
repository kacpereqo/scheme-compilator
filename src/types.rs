#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Tokens {
    Eof,
    Var(Types),
    Punctuation(Punctuations),
    Keyword(Keywords),
    Operator(Operators),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Keywords {
    Begin,
    End,
    If,
    Then,
    Else,
    True,
    False,
    Display,
    Newline,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Types {
    Int,
    Float,
    String,
    Bool,
    Function,
    Unknown,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Punctuations {
    LParen,
    RParen,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct LexerToken {
    pub token: Tokens,
    pub value: Option<String>,
}
