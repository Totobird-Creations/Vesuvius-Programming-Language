use crate::lexer;



pub fn run(filename : String, script: String) -> () {

    let tokens = lexer::Lexer::calculate(filename, script);
    for token in tokens {
        println!("{}", token);
    }
    
}
