pub struct Lexer{
    input: String,
    position: u64,
    row: u64,
    column: u64,
    ch: char,    
}

enum Tokens{
    Var(String),
    Type(Types),
    Punctuation(Punctuation),
    Keyword,}

enum Keyword{
    Begin,}

enum Types{
    Int,
    Float,
    String,
    Bool,
    Function,}

enum Punctuation{
    LParen,
    Rparen,}

enum Operators{
    Plus,
    Minus,
    Asterisk,
    Slash,

    Lt,
    Gt,
    Eq,
    NotEq,}

struct LexerToken{
    token: Tokens,
    value?: String,}

impl Lexer{
    pub fn new(input: String) -> Self{
        let mut self = Self{
            input,
            position: 0,
            row: 0,
            column: 0,
            ch: '',
        };
        self
    }
    
    fn next_char(mut self &Self) -> char {
        let temp_char = input[position++];
        self.column = position;
    
        if (temp_char == "\n") self.row++, column = 0;
        
        temp_char
    }

    fn skip_whitespace(mut self &Self) {
        while (ch == " " || ch == "\t" || ch == "\n" || ch == "\r") {
            ch = next_char();
        }
    }

    fn read_identifier(mut self &Self) -> String {
        let mut identifier = String::new();
        while (ch.is_alphanumeric() || ch == "_") {
            identifier.push(ch);
            ch = next_char();
        }
        identifier
    }

    fn read_number(mut self &Self) -> String {
        let mut number = String::new();
        while (ch.is_numeric() || ch == ".") {
            number.push(ch);
            ch = next_char();
        }
        number
    }

    fn next_token(mut self &Self) -> LexerToken {
        let token: Tokens;
        let value: String;
        let ch = self.next_char();

        loop {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    self.skip_whitespace();
                    ch = self.next_char();
                },
                'a'...'z' | 'A'...'Z'  => {
                    value = self.read_identifier();
                    token = self.lookup_identifier(value);
                    return LexerToken{token, value};
                },
                '0'...'9' => {
                    value = self.read_number();
                    token = self.read_number_type(value);
                    return LexerToken{token, value};
                },
                '(' | ')' => {
                    token = self.read_punctuation(ch);
                    return LexerToken{token};
                },

                '+' | '-' | '*' | '/' => {
                    token = self.read_operator(ch);
                    return LexerToken{token};
                },              
                _ => break,
            }
        }
}}