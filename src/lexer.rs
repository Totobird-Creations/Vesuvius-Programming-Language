use crate::data;
use crate::exception;
use crate::exception::Exception;



#[derive(Clone)]
pub struct Lexer {
    pub filename : String,
        script   : String,
        index    : usize,
        ch       : char,
        tokens   : Vec<data::Token>,
        end      : bool
}
impl Lexer {

    pub fn calculate(filename : String, script : String) -> Vec<data::Token> {
        let mut lexer = Lexer {
            filename : filename,
            script   : script,
            index    : 0,
            ch       : ' ',
            tokens   : Vec::new(),
            end      : false
        };
        lexer.update();
        lexer.start();
        return lexer.tokens;
        
    }

    fn advance(&mut self) -> () {
        self.index += 1;
        self.update();
    }

    fn retreat(&mut self) -> () {
        self.index -= 1;
        self.update();
    }

    fn update(&mut self) -> () {
        if (self.index < self.script.len()) {
            self.ch  = self.script.chars().nth(self.index).unwrap();
            self.end = false;
        } else {
            self.ch  = ' ';
            self.end = true;
        };
    }


    fn start(&mut self) -> () {
        while (! self.end) {
            
            if ([' ', '\t', '\n', '\r'].contains(&self.ch)) {
                self.advance();
            }

            else if (self.ch == '#') {
                self.push_token(data::TokenType::Hash);
                self.advance();
            }
            else if (self.ch == '(') {
                self.push_token(data::TokenType::LParenthesis);
                self.advance();
            }
            else if (self.ch == ')') {
                self.push_token(data::TokenType::RParenthesis);
                self.advance();
            }
            else if (self.ch == '[') {
                self.push_token(data::TokenType::LBracket);
                self.advance();
            }
            else if (self.ch == ']') {
                self.push_token(data::TokenType::RBracket);
                self.advance();
            }
            else if (self.ch == '{') {
                self.push_token(data::TokenType::LBrace);
                self.advance();
            }
            else if (self.ch == '}') {
                self.push_token(data::TokenType::RBrace);
                self.advance();
            }
            else if (self.ch == '<') {
                self.push_token(data::TokenType::LCarat);
                self.advance();
            }
            else if (self.ch == '>') {
                self.push_token(data::TokenType::RCarat);
                self.advance();
            }
            else if (self.ch == ':') {
                let start = self.index;
                self.advance();
                if (self.ch == ':') {
                    self.push_token_start(data::TokenType::DoubleColon, start);
                    self.advance();
                } else {
                    self.push_token_end(data::TokenType::Colon, start);
                }
            }
            else if (self.ch == '.') {
                self.push_token(data::TokenType::Colon);
                self.advance();
            }

            else if (self.ch == '=') {
                self.push_token(data::TokenType::Equals);
                self.advance();
            }

            else if (self.ch == '+') {
                self.push_token(data::TokenType::Plus);
                self.advance();
            }
            else if (self.ch == '-') {
                self.push_token(data::TokenType::Minus);
                self.advance();
            }
            else if (self.ch == '*') {
                let start = self.index;
                self.advance();
                if (self.ch == '*') {
                    self.push_token_start(data::TokenType::DoubleAstrisk, start);
                    self.advance();
                } else {
                    self.push_token_end(data::TokenType::Astrisk, start);
                }
            }
            else if (self.ch == '/') {
                let start = self.index;
                self.advance();
                if (self.ch == '/') {
                    self.start_eol_comment();
                } else {
                    self.push_token_end(data::TokenType::Slash, start);
                }
            }

            else if (data::ALPHABETIC.contains(self.ch)) {
                self.start_identifier();
            }

            else if (self.ch == '\'') {
                self.start_character();
            }

            else if (self.ch == '"') {
                self.start_string();
            }

            else if (data::NUMERIC.contains(self.ch)) {
                self.start_number();
            }

            else if (self.ch == ';') {
                self.push_token(data::TokenType::Eol);
                self.advance();
            }

            else {
                exception::LexerException::new(
                    self.clone(),
                    exception::LexerExceptionType::IllegalCharacter,
                    format!("Illegal character `{}` found.", self.ch)
                ).dump();
            };

        };

        self.push_token(data::TokenType::Eof);
    }

    fn start_identifier(&mut self) -> () {
        let     start      : usize  = self.index;
        let mut end        : usize  = self.index;
        let mut identifier : String = String::new();
        while ((! self.end) && (
            (String::from(data::ALPHABETIC) + "_").contains(self.ch)
        )) {
            identifier += self.ch.to_string().as_str();
            end = self.index;
            self.advance();
        }
        self.push_token_start_end(
            data::TokenType::Identifier(identifier),
            start, end
        );
    }

    fn start_character(&mut self) -> () {
        let     start : usize = self.index;
        let mut ch    : char  = ' ';
        if (self.ch != '\'') {
            exception::LexerException::new(
                self.clone(),
                exception::LexerExceptionType::MissingCharacter,
                format!("Expected character `'` not found.")
            ).dump();
        };
        self.advance();
        if (self.ch == '\\') {
            self.advance();
            match (data::calculate_escape(self.ch)) {
                Some(new_ch) => {
                    ch = new_ch;
                },
                None => {
                    exception::LexerException::new(
                        self.clone(),
                        exception::LexerExceptionType::InvalidEscape,
                        format!("Character `{}{}` can not be escaped.", if (self.ch == '`') {"\\"} else {""}, self.ch)
                    ).dump();
                }
            };
            self.advance();
        }
        if (self.ch != '\'') {
            exception::LexerException::new(
                self.clone(),
                exception::LexerExceptionType::MissingCharacter,
                format!("Expected character `'` not found.")
            ).dump();
        };
        self.push_token_start(
            data::TokenType::Char(ch),
            start
        );
        self.advance();
        
    }

    fn start_string(&mut self) -> () {
        let     start  : usize  = self.index;
        let mut ch     : char   = ' ';
        let mut string : String = String::new();
        if (self.ch != '"') {
            exception::LexerException::new(
                self.clone(),
                exception::LexerExceptionType::MissingCharacter,
                format!("Expected character `\"` not found.")
            ).dump();
        };
        self.advance();
        let mut escape = false;
        while ((! self.end) && (
            escape || self.ch != '"'
        )) {
            if (escape) {
                match (data::calculate_escape(self.ch)) {
                    Some(ch) => {
                        string += ch.to_string().as_str();
                    },
                    None => {
                        exception::LexerException::new(
                            self.clone(),
                            exception::LexerExceptionType::InvalidEscape,
                            format!("Character `{}{}` can not be escaped.", if (self.ch == '`') {"\\"} else {""}, self.ch)
                        ).dump();
                    }
                };
                escape = false;
            } else {
                if (self.ch == '\\') {
                    escape = true;
                } else {
                    string += self.ch.to_string().as_str();
                }
            }
            self.advance();
        }
        if (self.end) {
            exception::LexerException::new(
                self.clone(),
                exception::LexerExceptionType::MissingCharacter,
                format!("Expected character `\"` not found.")
            ).dump();
        };
        self.push_token_start(
            data::TokenType::String(string),
            start
        );
        self.advance();
        
    }

    fn start_number(&mut self) -> () {
        let     start  : usize  = self.index;
        let mut end    : usize  = self.index;
        let mut number : String = String::new();
        let mut dots   : usize  = 0;
        while ((! self.end) && (
            (String::from(data::NUMERIC) + "_.").contains(self.ch)
        )) {
            if (self.ch == '.') {
                if (dots >= 1) {
                    break;
                }
                dots += 1;
                number += ".";
            } else if (self.ch != '_') {
                number += self.ch.to_string().as_str();
            }
            end = self.index;
            self.advance();
        }
        self.retreat();
        if (! ['.'].contains(&self.ch)) {
            self.advance();
        }
        if (number.chars().nth(number.len() - 1).unwrap() == '.') {
            let mut chars = number.chars();
            chars.next_back();
            number = chars.as_str().to_string();
        }
        number = number.replace("_", "");
        if (dots == 0) {
            self.push_token_start_end(
                data::TokenType::Integer(number.parse::<i64>().unwrap()),
                start, end
            );
        } else {
            self.push_token_start_end(
                data::TokenType::Float(number.parse::<f64>().unwrap()),
                start, end
            );
        }
    }

    fn start_eol_comment(&mut self) -> () {
        while ((! self.end) && (
            self.ch != '\n'
        )) {
            self.advance();
        }
    }

    fn push_token(&mut self, token : data::TokenType) -> () {
        self.tokens.push(data::Token::new(
            token,
            data::Range::new(self.index, self.index)
        ));
    }

    fn push_token_start(&mut self, token : data::TokenType, start : usize) -> () {
        self.tokens.push(data::Token::new(
            token,
            data::Range::new(start, self.index)
        ));
    }

    fn push_token_start_end(&mut self, token : data::TokenType, start : usize, end : usize) -> () {
        self.tokens.push(data::Token::new(
            token,
            data::Range::new(start, end)
        ));
    }

    fn push_token_end(&mut self, token : data::TokenType, end : usize) -> () {
        self.tokens.push(data::Token::new(
            token,
            data::Range::new(end, end)
        ));
    }

}
