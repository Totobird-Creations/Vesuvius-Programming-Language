use std;



pub struct Range {
    pub min : u64,
    pub max : u64
}



pub struct Token {
    pub token : TokenType,
    pub range : Range
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "<{}>", self.token);
    }
}

#[allow(dead_code)]
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
