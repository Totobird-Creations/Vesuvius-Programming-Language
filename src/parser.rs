use crate::data;
use crate::exception;
use crate::exception::Exception;



#[derive(Clone)]
pub struct ParserData {
    pub allow_assign : bool
}
impl ParserData {

    pub fn new() -> ParserData {
        return ParserData {
            allow_assign : true
        }
    }
    
}



#[derive(Clone)]
pub struct Parser {
    script : String,
    tokens : Vec<data::Token>,
    index  : usize,
    token  : data::Token,
    nodes  : Vec<data::Node>,
    end    : bool
}
impl Parser {

    pub fn calculate(script : String, tokens : Vec<data::Token>) -> Vec<data::Node> {
        let mut parser = Parser {
            script : script,
            tokens : tokens,
            index  : 0,
            token  : data::Token::new_void(),
            nodes  : Vec::new(),
            end    : false
        };
        parser.update();
        parser.start();
        return parser.nodes;
        
    }

    fn advance(&mut self) -> () {
        self.index += 1;
        self.update();
    }

    fn update(&mut self) -> () {
        if (self.index < self.tokens.len()) {
            self.token = self.tokens[self.index].clone();
            self.end   = false;
        } else {
            self.token = data::Token::new_void();
            self.end   = true;
        };
    }


    fn start(&mut self) -> () {
        while ((! self.end) && (
            ! matches!(self.token.token, data::TokenType::Eof)
        )) {
            while (matches!(self.token.token, data::TokenType::Eol)) {
                self.advance();
            }
            if (matches!(self.token.token, data::TokenType::Eof)) {
                break;
            }
            let node = self.start_statement_global(ParserData::new());
            self.nodes.push(node);
        }
    }


    fn start_statement_global(&mut self, data : ParserData) -> data::Node {
        if (let data::TokenType::Identifier(keyword) = self.token.token.clone()) {
            if (keyword.as_str() == "extern") {
                let start = self.token.range.min.clone();
                self.advance();
                let name = if (let data::TokenType::Identifier(name) = self.token.token.clone()) {
                    name
                } else {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        String::from("Expected Identifier not found."),
                        self.script.clone(),
                        self.token.range.clone()
                    ).dump();
                };
                let end = self.token.range.max.clone();
                self.advance();
                return data::Node::new(
                    data::NodeType::ExternalImport(name),
                    data::Range::new(start, end)
                );
            }
        }
        panic!("other");
    }

}