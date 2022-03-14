use crate::data;



#[derive(Clone)]
pub struct Token {
    pub token : TokenType,
    pub range : data::Range
}
impl Token {
    pub fn new(token : TokenType, range : data::Range) -> Token {
        return Token {
            token : token,
            range : range
        }
    }
    pub fn new_void() -> Token {
        return Token {
            token : TokenType::Eof,
            range : data::Range::new_void()
        }
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "<{}>", self.token);
    }
}

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
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
    Equals,
    DoubleColon,
    Period,
    Comma,

    Plus,
    Minus,
    Astrisk,
    Slash,
    DoubleAstrisk,

    Bang,

    Identifier(String),
    Character(char),
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
            TokenType::Equals           => String::from("="),
            TokenType::DoubleColon      => String::from("::"),
            TokenType::Period           => String::from("."),
            TokenType::Comma            => String::from(","),

            TokenType::Plus             => String::from("+"),
            TokenType::Minus            => String::from("-"),
            TokenType::Astrisk          => String::from("*"),
            TokenType::Slash            => String::from("/"),
            TokenType::DoubleAstrisk    => String::from("**"),

            TokenType::Bang             => String::from("!"),

            TokenType::Identifier(name) => name.clone(),
            TokenType::Character(ch)    => format!("\'{}\'", ch),
            TokenType::String(text)     => format!("\"{}\"", text),
            TokenType::Integer(value)   => value.to_string(),
            TokenType::Float(value)     => value.to_string(),

            TokenType::Eol              => String::from(";"),
            TokenType::Eof              => String::from("Eof")

        });
    }
}