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
    Define,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Types {
    Int,
    Float,
    String,
    Bool,
    Unknown,
    Keyword,
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
    Percent,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

#[derive(Debug, Clone)]
pub struct LexerToken {
    pub var_type: Types,
    pub token: Tokens,
    pub value: Option<String>,
}
