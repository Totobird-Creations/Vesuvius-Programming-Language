use crate::lexer;
use crate::parser;



pub fn run(filename : String, script: String) -> () {

    let tokens = lexer::Lexer::calculate(filename, script.clone());

    let nodes  = parser::Parser::calculate(script, tokens);
    for node in nodes {
        println!("{}", node);
    }
    
}
