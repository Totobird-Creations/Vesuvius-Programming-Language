use crate::data;
use crate::exception;
use crate::exception::Exception;

#[derive(Clone)]
pub struct Node {
    pub node    : NodeType,
    pub range   : data::Range,
    pub headers : NodeHeaders
}
impl Node {
    pub fn new(node : NodeType, range : data::Range) -> Node {
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
    InitializeVariable(bool, String, Box<Node>, Box<Option<Node>>), // mutable, name, type, value
    AssignVariable(Box<Node>, Box<Node>), // node, value


    AdditionOperation(Box<Node>, Box<Node>), // left, right
    SubtractionOperation(Box<Node>, Box<Node>), // left, right
    MultiplicationOperation(Box<Node>, Box<Node>), // left, right
    DivisionOperation(Box<Node>, Box<Node>), // left, right
    PowerOperation(Box<Node>, Box<Node>), // left, right
    InvertOperation(Box<Node>), // value
    OppositeOperation(Box<Node>), // value


    ModuleMember(Box<Node>, String), // parent, child
    ClassMember(Box<Node>, String), // parent, child
    Slice(Box<Node>, Box<Node>), // parent, slice
    Call(Box<Node>, Box<Vec<Node>>), // parent, call


    Type(data::Type, Vec<Node>), // base, arguments
    Literal(data::Literal) // value

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
            NodeType::InitializeVariable(mutable, name, typ, value) => format!("let{} {}: {} {}", if (*mutable) {" mut"} else {""}, name, typ, if (let Some(val) = *value.clone()) {format!("= {}", val)} else {String::new()}),
            NodeType::AssignVariable(parent, value)                 => format!("{} = {}", parent, value),


            NodeType::AdditionOperation(left, right)       => format!("({} + {})", left, right),
            NodeType::SubtractionOperation(left, right)    => format!("({} - {})", left, right),
            NodeType::MultiplicationOperation(left, right) => format!("({} * {})", left, right),
            NodeType::DivisionOperation(left, right)       => format!("({} / {})", left, right),
            NodeType::PowerOperation(left, right)          => format!("({} ** {})", left, right),
            NodeType::OppositeOperation(value)             => format!("(- {})", value),
            NodeType::InvertOperation(value)               => format!("(! {})", value),


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
    pub fn from(array : Vec<(String, data::Range)>) -> NodeHeaders {
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
                        range
                    ).dump_warning();
                }
            }
        }
        return headers;
    }
}