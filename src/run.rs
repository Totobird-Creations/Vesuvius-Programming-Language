use crate::lexer;
use crate::parser;



pub fn run(filename : String, script: String) -> () {

    let tokens = lexer::Lexer::calculate(filename, script);

    let nodes  = parser::Parser::calculate(tokens);
    for node in nodes {
        println!("{}", node);
    }
    
}
