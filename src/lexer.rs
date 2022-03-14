use crate::data;
use crate::exception;
use crate::exception::Exception;



#[derive(Clone)]
pub struct Lexer {
        script   : String,
    pub position : data::Position,
        ch       : char,
        tokens   : Vec<data::Token>,
        end      : bool
}
impl Lexer {

    pub fn calculate(filename : String, script : String) -> Vec<data::Token> {
        let mut lexer = Lexer {
            script   : script.replace("\r\n", "\n").replace("\r", "\n"),
            position : data::Position::new(
                0,
                0,
                0,
                filename,
                script
            ),
            ch     : ' ',
            tokens : Vec::new(),
            end    : false
        };
        lexer.update();
        lexer.start();
        return lexer.tokens;
        
    }

    fn advance(&mut self) -> () {
        self.position.index  += 1;
        self.position.column += 1;
        self.update();
        if (self.ch == '\n') {
            self.position.line   += 1;
            self.position.column  = 0;
        }
    }

    fn update(&mut self) -> () {
        if (self.position.index < self.script.len()) {
            self.ch  = self.script.chars().nth(self.position.index).unwrap();
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

            else if (self.ch == '=') {
                self.push_token(data::TokenType::Equals);
                self.advance();
            }
            else if (self.ch == ':') {
                let start = self.position.clone();
                self.advance();
                if (self.ch == ':') {
                    self.push_token_start(data::TokenType::DoubleColon, start);
                    self.advance();
                } else {
                    self.push_token_end(data::TokenType::Colon, start);
                }
            }
            else if (self.ch == '.') {
                self.push_token(data::TokenType::Period);
                self.advance();
            }
            else if (self.ch == ',') {
                self.push_token(data::TokenType::Comma);
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
                let start = self.position.clone();
                self.advance();
                if (self.ch == '*') {
                    self.push_token_start(data::TokenType::DoubleAstrisk, start);
                    self.advance();
                } else {
                    self.push_token_end(data::TokenType::Astrisk, start);
                }
            }
            else if (self.ch == '/') {
                let start = self.position.clone();
                self.advance();
                if (self.ch == '/') {
                    self.start_eol_comment();
                } else {
                    self.push_token_end(data::TokenType::Slash, start);
                }
            }
            else if (self.ch == '!') {
                self.push_token(data::TokenType::Bang);
                self.advance();
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
                    exception::LexerExceptionType::IllegalCharacter,
                    format!("Illegal character `{}` found.", self.ch),
                    data::Range::new(self.position.clone(), self.position.clone())
                ).dump_error();
            };

        };

        self.push_token(data::TokenType::Eof);
    }

    fn start_identifier(&mut self) -> () {
        let     start      = self.position.clone();
        let mut end        = self.position.clone();
        let mut identifier = String::new();
        while ((! self.end) && (
            (String::from(data::ALPHABETIC) + data::NUMERIC + "_").contains(self.ch)
        )) {
            identifier += self.ch.to_string().as_str();
            end = self.position.clone();
            self.advance();
        }
        self.push_token_start_end(
            data::TokenType::Identifier(identifier),
            start, end
        );
    }

    fn start_character(&mut self) -> () {
        let     start = self.position.clone();
        let mut ch    = ' ';
        if (self.ch != '\'') {
            exception::LexerException::new(
                exception::LexerExceptionType::MissingCharacter,
                format!("Expected character `'` not found."),
                data::Range::new(start, self.position.clone())
            ).dump_error();
        };
        self.advance();
        if (self.ch == '\\') {
            let ch_start = self.position.clone();
            self.advance();
            match (self.calculate_escape()) {
                Ok(new_ch) => {
                    ch = new_ch;
                },
                Err(new_ch) => {
                    exception::LexerException::new(
                        exception::LexerExceptionType::InvalidEscape,
                        format!("Discarded escape `{}` not allowed in character literal.", new_ch.replace("\\","\\\\").replace("`","\\`")),
                        data::Range::new(ch_start, self.position.clone())
                    ).dump_error();
                }
            };
            self.advance();
        }
        if (self.ch != '\'') {
            exception::LexerException::new(
                exception::LexerExceptionType::MissingCharacter,
                format!("Expected character `'` not found."),
                data::Range::new(start, self.position.clone())
            ).dump_error();
        };
        self.push_token_start(
            data::TokenType::Character(ch),
            start
        );
        self.advance();
        
    }

    fn start_string(&mut self) -> () {
        let     start  = self.position.clone();
        let mut string = String::new();
        if (self.ch != '"') {
            exception::LexerException::new(
                exception::LexerExceptionType::MissingCharacter,
                format!("Expected character `\"` not found."),
                data::Range::new(start, self.position.clone())
            ).dump_error();
        };
        self.advance();
        let mut escape       = false;
        let mut escape_start = self.position.clone();
        while ((! self.end) && (
            escape || self.ch != '"'
        )) {
            if (escape) {
                match (self.calculate_escape()) {
                    Ok(ch) => {
                        string += ch.to_string().as_str();
                    },
                    Err(ch) => {
                        exception::LexerException::new(
                            exception::LexerExceptionType::InvalidEscape,
                            format!("Discarded escape `{}`.", ch.replace("\\","\\\\").replace("`","\\`")),
                            data::Range::new(escape_start.clone(), self.position.clone())
                        ).dump_warning();
                        string += "\\";
                        string += self.ch.to_string().as_str();
                    }
                };
                escape = false;
            } else {
                if (self.ch == '\\') {
                    escape = true;
                    escape_start = self.position.clone();
                } else {
                    string += self.ch.to_string().as_str();
                }
            }
            self.advance();
        }
        if (self.end) {
            exception::LexerException::new(
                exception::LexerExceptionType::MissingCharacter,
                format!("Expected character `\"` not found."),
                data::Range::new(start, self.position.clone())
            ).dump_error();
        };
        self.push_token_start(
            data::TokenType::String(string),
            start
        );
        self.advance();
        
    }

    pub fn calculate_escape(&mut self) -> Result<char, String> {
        return match (self.ch) {
            '\\' => {
                Ok('\\')
            },
            'n' => {
                Ok('\n')
            },
            't' => {
                Ok('\t')
            },
            '"' => {
                Ok('"')
            },
            '\'' => {
                Ok('\'')
            },
            'r' => {
                Ok('\r')
            }
    
            _ => {
                Err(self.ch.to_string())
            }
        };
    }

    fn start_number(&mut self) -> () {
        let     start  = self.position.clone();
        let mut end    = self.position.clone();
        let mut number = String::new();
        let mut dots   = 0;
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
            end = self.position.clone();
            self.advance();
        }
        if (number.chars().nth(number.len() - 1).unwrap() == '.') {
            number += "0";
        }
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
            data::Range::new(self.position.clone(), self.position.clone())
        ));
    }

    fn push_token_start(&mut self, token : data::TokenType, start : data::Position) -> () {
        self.tokens.push(data::Token::new(
            token,
            data::Range::new(start, self.position.clone())
        ));
    }

    fn push_token_start_end(&mut self, token : data::TokenType, start : data::Position, end : data::Position) -> () {
        self.tokens.push(data::Token::new(
            token,
            data::Range::new(start, end)
        ));
    }

    fn push_token_end(&mut self, token : data::TokenType, end : data::Position) -> () {
        self.tokens.push(data::Token::new(
            token,
            data::Range::new(end.clone(), end)
        ));
    }

}
