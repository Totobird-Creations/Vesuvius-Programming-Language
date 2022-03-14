use crate::lexer;
use crate::parser;
use crate::validator;



pub fn run(filename : String, script: String) -> () {

    let tokens = lexer::Lexer::calculate(filename, script);

    let nodes  = parser::Parser::calculate(tokens);

    validator::Validator::calculate(nodes);
    
}
