use crate::data;
use crate::exception;
use crate::exception::Exception;



#[derive(Clone)]
pub struct Parser {
    tokens : Vec<data::Token>,
    index  : usize,
    token  : data::Token,
    nodes  : Vec<data::Node>,
    end    : bool
}
impl Parser {

    pub fn calculate(tokens : Vec<data::Token>) -> Vec<data::Node> {
        let mut parser = Parser {
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
    }

}