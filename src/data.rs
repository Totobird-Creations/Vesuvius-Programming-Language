use std;

use crate::exception;
use crate::exception::Exception;



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
    pub fn new_void() -> Range {
        return Range::new(
            Position::new(0, 0, 0, String::new()),
            Position::new(0, 0, 0, String::new())
        );
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
    pub fn new_void() -> Token {
        return Token {
            token : TokenType::Eof,
            range : Range::new_void()
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



#[derive(Clone)]
pub struct Node {
    pub node    : NodeType,
    pub range   : Range,
    pub headers : NodeHeaders
}
impl Node {
    pub fn new(node : NodeType, range : Range) -> Node {
        return Node {
            node    : node,
            range   : range,
            headers : NodeHeaders::new()
        }
    }
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self.node);
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum NodeType {

    ExternalImport(String), // module_name
    LocalImport(String),    // filename


    DefineFunction(String, Box<Vec<(String, Node)>>, Box<Node>, Box<Vec<Node>>), // name, args(name, type), return_type, content


    InitializeVariable(bool, String, Box<Node>, Box<Node>), // mutable, name, type, value
    AssignVariable(Box<Node>, Box<Node>),              // node, value

    AdditionOperation(Box<Node>, Box<Node>), // left, right
    SubtractionOperation(Box<Node>, Box<Node>), // left, right
    MultiplicationOperation(Box<Node>, Box<Node>), // left, right
    DivisionOperation(Box<Node>, Box<Node>), // left, right
    PowerOperation(Box<Node>, Box<Node>), // left, right
    InvertOperation(Box<Node>), // value


    ModuleMember(Box<Node>, String), // parent, child
    ClassMember(Box<Node>, String), // parent, child
    Slice(Box<Node>, Box<Node>), // parent, slice
    Call(Box<Node>, Box<Vec<Node>>), // parent, call


    Type(Type, Vec<Node>), // base, arguments
    Literal(Literal) // value

}
impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", match (self) {

            NodeType::ExternalImport(name) => format!("extern {}", name),
            NodeType::LocalImport(name)    => format!("use {}", name),


            NodeType::DefineFunction(target, args, return_type, body) => {
                let mut res_args = Vec::new();
                for i in 0..(args.len()) {
                    let (name, typ) = args[i].clone();
                    res_args.push(format!("{}: {}", name, typ));
                }
                let mut res_body = Vec::new();
                for i in 0..(body.len()) {
                    let expression = body[i].clone();
                    res_body.push(format!("{};", expression));
                }
                format!("func {}({}): {} {{{}}}", target, res_args.join(", "), return_type, res_body.join(" "))
            },


            NodeType::InitializeVariable(mutable, name, typ, value) => format!("let{} {}: {} = {}", if (*mutable) {" mut"} else {""}, name, typ, value),
            NodeType::AssignVariable(parent, value)                 => format!("{} = {}", parent, value),


            NodeType::AdditionOperation(left, right)       => format!("({} + {})", left, right),
            NodeType::SubtractionOperation(left, right)    => format!("({} - {})", left, right),
            NodeType::MultiplicationOperation(left, right) => format!("({} * {})", left, right),
            NodeType::DivisionOperation(left, right)       => format!("({} / {})", left, right),
            NodeType::PowerOperation(left, right)          => format!("({} ** {})", left, right),
            NodeType::InvertOperation(value)               => format!("(- {})", value),


            NodeType::ModuleMember(parent, name) => format!("{}::{}", parent, name),
            NodeType::ClassMember(parent, name)  => format!("{}.{}", parent, name),
            NodeType::Slice(parent, slice)       => format!("{}[{}]", parent, slice),
            NodeType::Call(parent, args)         => {
                let mut res_args = Vec::new();
                for i in 0..(args.len()) {
                    let arg = args[i].clone();
                    res_args.push(format!("{}", arg));
                }
                format!("{}({})", parent, res_args.join(", "))
            },


            NodeType::Type(base, arguments) => {
                let mut res_arguments = Vec::new();
                for i in 0..(arguments.len()) {
                    let base = arguments[i].clone();
                    res_arguments.push(format!("{}", base));
                }
                format!("{}{}", base, if (arguments.len() >= 1) {format!("{}", res_arguments.join(", "))} else {String::new()})
            }
            NodeType::Literal(value)        => format!("{}", value)

        });
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Literal {
    Name(String),
    Character(char),
    String(String),
    Integer(i64),
    Float(f64)
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", match (self) {

            Literal::Name(name)      => name.clone(),
            Literal::Character(ch)   => format!("'{}'", ch),
            Literal::String(text)    => format!("\"{}\'", text),
            Literal::Integer(number) => number.to_string(),
            Literal::Float(number)   => number.to_string()

        });
    }
}

#[derive(Clone)]
pub enum Type {
    Base(Vec<String>),
    Inferred
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", match (self) {
            Type::Base(parts) => parts.join("::"),
            Type::Inferred    => String::from("?")
        });
    }
}

#[derive(Clone)]
pub struct NodeHeaders {
    pub is_entry  : bool,
    pub is_static : bool,
    pub is_public : bool
}
impl NodeHeaders {
    pub fn new() -> NodeHeaders {
        return NodeHeaders {
            is_entry  : false,
            is_static : false,
            is_public : false
        }
    }
    pub fn from(array : Vec<(String, Range)>, script : String) -> NodeHeaders {
        let mut headers = NodeHeaders::new();
        for i in 0..array.len() {
            let (name, range) = array[i].clone();
            match (name.as_str()) {
                "entry"  => headers.is_entry  = true,
                "static" => headers.is_static = true,
                "public" => headers.is_public = true,
                _        => {
                    exception::ParserException::new(
                        exception::ParserExceptionType::InvalidHeader,
                        format!("Invalid header `{}`.", name),
                        script.clone(),
                        range
                    ).dump_warning();
                }
            }
        }
        return headers;
    }
}
