struct Lexer {
    script : String,
    index  : usize,
    ch     : Some<char>
}
impl Lexer {

    pub fn new(script : String) -> Lexer {
        let lexer = Lexer {
            script : script,
            index  : 0,
            ch     : None
        };
        lexer.update();
        return lexer;
        
    }

    fn advance(&mut self) -> () {
        self.index += 1;
        self.update();
    }

    fn update(&mut self) -> () {
        self.ch = if (self.index < self.script.len()) {
            Some(self.script[self.index])
        } else {
            None
        };
    }

}
