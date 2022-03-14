use std;

mod token;
pub use token::{
    Token,
    TokenType
};
mod node;
pub use node::{
    Node,
    NodeType,
    NodeHeaders
};
mod value;
pub use value::{
    Literal,
    Type
};
mod object;
pub use object::{
    Object,
    ObjectType
};
mod context;
pub use context::{
    Context
};



pub const ALPHABETIC : &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const NUMERIC    : &'static str = "0123456789";



#[derive(Clone)]
pub struct Position {
    pub index    : usize,
    pub line     : usize,
    pub column   : usize,
    pub filename : String,
    pub script   : String
}
impl Position {
    pub fn new(index : usize, line : usize, column : usize, filename : String, script : String) -> Position {
        return Position {
            index    : index,
            line     : line,
            column   : column,
            filename : filename,
            script   : script
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
    pub fn new_void() -> Range {
        return Range::new(
            Position::new(0, 0, 0, String::new(), String::new()),
            Position::new(0, 0, 0, String::new(), String::new())
        );
    }
}
