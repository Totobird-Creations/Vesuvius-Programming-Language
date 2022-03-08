use std;



pub const ALPHABETIC : &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const NUMERIC    : &'static str = "0123456789";



#[derive(Clone)]
pub struct Position {
    pub index    : usize,
    pub line     : usize,
    pub column   : usize,
    pub filename : String
}
impl Position {
    pub fn new(index : usize, line : usize, column : usize, filename : String) -> Position {
        return Position {
            index    : index,
            line     : line,
            column   : column,
            filename : filename
        }
    }
}



#[derive(Clone)]
pub struct Range {
    pub min : Position,
    pub max : Position
}
impl Range {
    pub fn new(min : Position, max : Position) -> Range {
        return Range {
            min : min,
            max : max
        }
    }
}



#[derive(Clone)]
pub struct Token {
    pub token : TokenType,
    pub range : Range
}
impl Token {
    pub fn new(token : TokenType, range : Range) -> Token {
        return Token {
            token : token,
            range : range
        }
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "<{}>", self.token);
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum TokenType {
    Hash,
    LParenthesis,
    RParenthesis,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    LCarat,
    RCarat,
    Colon,
    DoubleColon,
    Period,

    Equals,

    Plus,
    Minus,
    Astrisk,
    Slash,
    DoubleAstrisk,

    Identifier(String),
    Char(char),
    String(String),
    Integer(i64),
    Float(f64),

    Eol,
    Eof
}
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", match (self) {

            TokenType::Hash             => String::from("#"),
            TokenType::LParenthesis     => String::from("("),
            TokenType::RParenthesis     => String::from(")"),
            TokenType::LBracket         => String::from("["),
            TokenType::RBracket         => String::from("]"),
            TokenType::LBrace           => String::from("{"),
            TokenType::RBrace           => String::from("}"),
            TokenType::LCarat           => String::from("<"),
            TokenType::RCarat           => String::from(">"),
            TokenType::Colon            => String::from(":"),
            TokenType::DoubleColon      => String::from("::"),
            TokenType::Period           => String::from("."),

            TokenType::Equals           => String::from("="),

            TokenType::Plus             => String::from("+"),
            TokenType::Minus            => String::from("-"),
            TokenType::Astrisk          => String::from("*"),
            TokenType::Slash            => String::from("/"),
            TokenType::DoubleAstrisk    => String::from("**"),

            TokenType::Identifier(name) => name.clone(),
            TokenType::Char(ch)         => format!("\'{}\'", ch),
            TokenType::String(text)     => format!("\"{}\"", text),
            TokenType::Integer(value)   => value.to_string(),
            TokenType::Float(value)     => value.to_string(),

            TokenType::Eol              => String::from(";"),
            TokenType::Eof              => String::from("Eof")

        });
    }
}



pub fn calculate_escape(ch : char) -> Option<char> {
    match (ch) {
        '\\' => return Some('\\'),
        'n'  => return Some('\n'),
        't'  => return Some('\t'),
        '"'  => return Some('"'),
        '\'' => return Some('\''),

        _    => return None
    };
}
