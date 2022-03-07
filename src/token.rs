use crate::data;



pub struct Token {
    tokentype : TokenType,
    range     : data::Range
}



pub enum TokenType {
    Hash,
    LParen,
    RParen,
    LBrack,
    RBrack,
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

    String(String),
    Int(i64),
    Float(f64)
}
