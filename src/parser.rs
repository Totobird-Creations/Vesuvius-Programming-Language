use crate::data;
use crate::exception;
use crate::exception::Exception;



#[derive(Clone)]
pub struct ParserData {
    pub allow_assign  : bool,
    pub allow_mutable : bool
}
impl ParserData {

    pub fn new() -> ParserData {
        return ParserData {
            allow_assign  : true,
            allow_mutable : true
        }
    }
    
}



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
                        self.token.range.clone()
                    ).dump_error();
                };
                let end = self.token.range.max.clone();
                self.advance();
                return data::Node::new(
                    data::NodeType::ExternalImport(name),
                    data::Range::new(start, end)
                );
            }

            else if (keyword.as_str() == "use") {
                let start = self.token.range.min.clone();
                self.advance();
                let name = if (let data::TokenType::Identifier(name) = self.token.token.clone()) {
                    name
                } else {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        String::from("Expected Identifier not found."),
                        self.token.range.clone()
                    ).dump_error();
                };
                let end = self.token.range.max.clone();
                self.advance();
                return data::Node::new(
                    data::NodeType::LocalImport(name),
                    data::Range::new(start, end)
                );
            }
        }
        
        return self.start_statement(data);

    }



    fn start_statement(&mut self, data : ParserData) -> data::Node {

        if (let data::TokenType::Identifier(keyword) = self.token.token.clone()) {
            if (keyword == String::from("let")) {
                let mut new_data = data.clone();
                new_data.allow_mutable = false;
                return self.start_initialize_variable(new_data);
            }
        }

        let mut header_list = Vec::new();
        while (matches!(self.token.token, data::TokenType::Hash)) {
            header_list.push((self.start_statement_header(), self.token.range.clone()));
        }

        if (let data::TokenType::Identifier(keyword) = self.token.token.clone()) {
            if (keyword == String::from("func")) {
                let mut function = self.start_statement_function(data);
                function.headers = data::NodeHeaders::from(header_list);
                return function;
            }
        }

        exception::ParserException::new(
            exception::ParserExceptionType::MissingToken,
            format!("Expected {}`#`, `func` not found.", if (header_list.len() >= 1) {""} else {"`let`, "}),
            self.token.range.clone()
        ).dump_error();

    }



    fn start_statement_header(&mut self) -> String {

        if (! matches!(self.token.token, data::TokenType::Hash)) {
            exception::ParserException::new(
                exception::ParserExceptionType::MissingToken,
                String::from("Expected `#` not found."),
                self.token.range.clone()
            ).dump_error();
        }
        self.advance();

        if (! matches!(self.token.token, data::TokenType::LBracket)) {
            exception::ParserException::new(
                exception::ParserExceptionType::MissingToken,
                String::from("Expected `[` not found."),
                self.token.range.clone()
            ).dump_error();
        }
        self.advance();

        let name = if (let data::TokenType::Identifier(name) = self.token.token.clone()) {
            name
        } else {
            exception::ParserException::new(
                exception::ParserExceptionType::MissingToken,
                String::from("Expected Identifier not found."),
                self.token.range.clone()
            ).dump_error();
        };
        self.advance();

        if (! matches!(self.token.token, data::TokenType::RBracket)) {
            exception::ParserException::new(
                exception::ParserExceptionType::MissingToken,
                String::from("Expected `]` not found."),
                self.token.range.clone()
            ).dump_error();
        }
        self.advance();

        return name;

    }



    fn start_statement_function(&mut self, data : ParserData) -> data::Node {

        if (let data::TokenType::Identifier(keyword) = self.token.token.clone()) {
            let start = self.token.range.min.clone();

            if (keyword == String::from("func")) {

                self.advance();

                let name = if (let data::TokenType::Identifier(name) = self.token.token.clone()) {
                    name
                } else {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        String::from("Expected Identifier not found."),
                        self.token.range.clone()
                    ).dump_error();
                };
                self.advance();
                if (! matches!(self.token.token, data::TokenType::LParenthesis)) {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        String::from("Expected `(` not found."),
                        self.token.range.clone()
                    ).dump_error();
                }
                self.advance();

                let mut args = Vec::new();
                if (! matches!(self.token.token, data::TokenType::RParenthesis)) {
                    args.push(self.start_statement_function_argument(data.clone()));
                    while (matches!(self.token.token, data::TokenType::Comma)) {
                        self.advance();
                        args.push(self.start_statement_function_argument(data.clone()));
                    }
                }

                if (! matches!(self.token.token, data::TokenType::RParenthesis)) {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        String::from("Expected `,`, `)` not found."),
                        self.token.range.clone()
                    ).dump_error();
                }
                self.advance();

                if (! matches!(self.token.token, data::TokenType::Colon)) {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        String::from("Expected `:` not found."),
                        self.token.range.clone()
                    ).dump_error();
                }
                self.advance();

                let return_type = self.start_type(data.clone());

                if (! matches!(self.token.token, data::TokenType::LBrace)) {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        String::from("Expected `{` not found."),
                        self.token.range.clone()
                    ).dump_error();
                }
                self.advance();

                let mut content = Vec::new();

                while (! matches!(self.token.token, data::TokenType::RBrace)) {
                    content.push(self.start_expression_base(data.clone()));
                }

                let end = self.token.range.max.clone();
                if (! matches!(self.token.token, data::TokenType::RBrace)) {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        String::from("Expected `}` not found."),
                        self.token.range.clone()
                    ).dump_error();
                }
                self.advance();

                return data::Node::new(
                    data::NodeType::DefineFunction(
                        name, Box::new(args), Box::new(return_type), Box::new(content)
                    ),
                    data::Range::new(start, end)
                );

            }

        }

        exception::ParserException::new(
            exception::ParserExceptionType::MissingToken,
            String::from("Expected `func` not found."),
            self.token.range.clone()
        ).dump_error();

    }


    fn start_statement_function_argument(&mut self, data : ParserData) -> (String, data::Node) {

        if (let data::TokenType::Identifier(name) = self.token.token.clone()) {
            let start = self.token.range.min.clone();
            
            self.advance();
            if (! matches!(self.token.token, data::TokenType::Colon)) {
                exception::ParserException::new(
                    exception::ParserExceptionType::MissingToken,
                    String::from("Expected `:` not found."),
                    self.token.range.clone()
                ).dump_error();
            }
            self.advance();

            let typ = self.start_type(data);

            return (name, data::Node::new(
                typ.node.clone(),
                data::Range::new(start, typ.range.max)
            ));
            
        }
        
        exception::ParserException::new(
            exception::ParserExceptionType::MissingToken,
            String::from("Expected Identifier not found."),
            self.token.range.clone()
        ).dump_error();

    }





    fn start_expression_base(&mut self, data : ParserData) -> data::Node {

        if (let data::TokenType::Identifier(keyword) = self.token.token.clone()) {
            //let start = self.token.range.min.clone();

            if (keyword == String::from("let")) {
                let mut new_data = data.clone();
                new_data.allow_mutable = true;
                return self.start_initialize_variable(new_data);
            }

        }

        let mut new_data = data.clone();
        new_data.allow_assign = true;
        let node = self.start_expression(new_data);

        if (! matches!(self.token.token, data::TokenType::Eol)) {
            exception::ParserException::new(
                exception::ParserExceptionType::MissingToken,
                String::from("Expected `;` not found."),
                self.token.range.clone()
            ).dump_error();
        }
        self.advance();

        return node;

    }



    fn start_expression(&mut self, data : ParserData) -> data::Node {
        return self.start_expression_addition(data);
    }



    fn start_expression_addition(&mut self, data : ParserData) -> data::Node {

        let mut left = self.start_expression_multiplication(data.clone());
        while ([data::TokenType::Plus, data::TokenType::Minus].contains(&self.token.token)) {
            let operation = self.token.token.clone();
            self.advance();
            let right     = self.start_expression_multiplication(data.clone());
            left = data::Node::new(
                match (operation) {
                    data::TokenType::Plus  => data::NodeType::AdditionOperation(Box::new(left.clone()), Box::new(right.clone())),
                    data::TokenType::Minus => data::NodeType::SubtractionOperation(Box::new(left.clone()), Box::new(right.clone())),
                    _                      => {
                        exception::InternalException::new(
                            String::from("Invalid Addition Operation")
                        ).dump_critical();
                    }
                },
                data::Range::new(left.range.min, right.range.max)
            );
        }
        return left;

    }



    fn start_expression_multiplication(&mut self, data : ParserData) -> data::Node {

        let mut left = self.start_expression_power(data.clone());
        while ([data::TokenType::Astrisk, data::TokenType::Slash].contains(&self.token.token)) {
            let operation = self.token.token.clone();
            self.advance();
            let right     = self.start_expression_power(data.clone());
            left = data::Node::new(
                match (operation) {
                    data::TokenType::Astrisk => data::NodeType::MultiplicationOperation(Box::new(left.clone()), Box::new(right.clone())),
                    data::TokenType::Slash   => data::NodeType::DivisionOperation(Box::new(left.clone()), Box::new(right.clone())),
                    _                        => {
                        exception::InternalException::new(
                            String::from("Invalid Multiplication Operation")
                        ).dump_critical();
                    }
                },
                data::Range::new(left.range.min, right.range.max)
            );
        }
        return left;

    }



    fn start_expression_power(&mut self, data : ParserData) -> data::Node {
        
        let mut left = self.start_term(data.clone());
        while ([data::TokenType::DoubleAstrisk].contains(&self.token.token)) {
            let operation = self.token.token.clone();
            self.advance();
            let right     = self.start_term(data.clone());
            left = data::Node::new(
                match (operation) {
                    data::TokenType::Plus  => data::NodeType::PowerOperation(Box::new(left.clone()), Box::new(right.clone())),
                    _                      => {
                        exception::InternalException::new(
                            String::from("Invalid Power Operation")
                        ).dump_critical();
                    }
                },
                data::Range::new(left.range.min, right.range.max)
            );
        }
        return left;
    }





    fn start_term(&mut self, data : ParserData) -> data::Node {
    
        let mut left = self.start_atom(data.clone());

        loop {

            if (matches!(self.token.token, data::TokenType::DoubleColon)) {
                self.advance();
                if (let data::TokenType::Identifier(name) = self.token.token.clone()) {
                    left = data::Node::new(
                        data::NodeType::ModuleMember(Box::new(left.clone()), name),
                        data::Range::new(left.range.min, self.token.range.max.clone())
                    );
                    self.advance();
                } else {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        String::from("Expected Identifier not found."),
                        self.token.range.clone()
                    ).dump_error();
                }
            }

            else if (matches!(self.token.token, data::TokenType::Period)) {
                self.advance();
                if (let data::TokenType::Identifier(name) = self.token.token.clone()) {
                    left = data::Node::new(
                        data::NodeType::ClassMember(Box::new(left.clone()), name),
                        data::Range::new(left.range.min, self.token.range.max.clone())
                    );
                    self.advance();
                } else {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        String::from("Expected Identifier not found."),
                        self.token.range.clone()
                    ).dump_error();
                }
            }

            else if (matches!(self.token.token, data::TokenType::LBracket)) {
                self.advance();
                let mut new_data = data.clone();
                new_data.allow_assign = false;
                let right = self.start_expression(new_data);
                if (! matches!(self.token.token, data::TokenType::RBracket)) {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        String::from("Expected `]` not found."),
                        self.token.range.clone()
                    ).dump_error();
                }
                left = data::Node::new(
                    data::NodeType::Slice(Box::new(left.clone()), Box::new(right.clone())),
                    data::Range::new(left.range.min, self.token.range.max.clone())
                );
                self.advance();
            }

            else if (matches!(self.token.token, data::TokenType::LParenthesis)) {
                self.advance();
                let mut new_data = data.clone();
                new_data.allow_assign = false;
                let mut args = Vec::new();
                if (! matches!(self.token.token, data::TokenType::RParenthesis)) {
                    args.push(self.start_expression(new_data.clone()));
                    while (matches!(self.token.token, data::TokenType::Comma)) {
                        self.advance();
                        args.push(self.start_expression(new_data.clone()));
                    }
                }
                if (! matches!(self.token.token, data::TokenType::RParenthesis)) {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        String::from("Expected `,`, `)` not found."),
                        self.token.range.clone()
                    ).dump_error();
                }
                let end = self.token.range.max.clone();
                self.advance();
                return data::Node::new(
                    data::NodeType::Call(Box::new(left.clone()), Box::new(args)),
                    data::Range::new(left.range.min, end)
                );
            }

            else {
                break;
            }

        }

        return left;

    }



    /*fn start_term_identifier_action(&mut self, _data : ParserData) -> data::Node {
        panic!("Term Identifier Action");
    }*/



    fn start_initialize_variable(&mut self, data : ParserData) -> data::Node {
    
        if (let data::TokenType::Identifier(keyword) = self.token.token.clone()) {
            let start = self.token.range.min.clone();

            if (keyword == String::from("let")) {

                self.advance();
                let mut mutable = false;
                let mut name    = None;

                if (let data::TokenType::Identifier(mut_or_name) = self.token.token.clone()) {

                    if (mut_or_name == String::from("mut")) {
                        if (data.allow_mutable) {
                            mutable = true;
                        } else {
                            exception::ParserException::new(
                                exception::ParserExceptionType::InvalidMutability,
                                String::from("Mutable name not allowed in this position."),
                                self.token.range.clone()
                            ).dump_error();
                        }
                    }
                    else {
                        name = Some(mut_or_name)
                    }
                    self.advance();

                }

                if (matches!(name, None)) {

                    name = if (let data::TokenType::Identifier(name) = self.token.token.clone()) {
                        Some(name)
                    } else {
                        exception::ParserException::new(
                            exception::ParserExceptionType::MissingToken,
                            String::from("Expected Identifier not found."),
                            self.token.range.clone()
                        ).dump_error();
                    };
                    self.advance();

                }

                let mut end = self.token.range.max.clone();

                let (type_set, typ) = if (matches!(self.token.token, data::TokenType::Colon)) {
                    self.advance();
                    let typ = self.start_type(data.clone());
                    end = typ.range.max.clone();
                    (true, typ)
                } else {
                    (false, data::Node::new(
                        data::NodeType::Type(data::Type::Inferred, Vec::new()),
                        data::Range::new_void()
                    ))
                };

                let mut value = None;
                if (matches!(self.token.token, data::TokenType::Equals)) {
                    self.advance();

                    let mut new_data = data.clone();
                    new_data.allow_assign = false;
                    let expr = self.start_expression(new_data);
                    value = Some(expr.clone());
                    end = expr.range.max.clone();
                }
                else if (! mutable) {
                    exception::ParserException::new(
                        exception::ParserExceptionType::InvalidMutability,
                        String::from("Non-mutable name must be set on initialization."),
                        self.token.range.clone()
                    ).dump_error();
                }

                if (! matches!(self.token.token, data::TokenType::Eol)) {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        format!("Expected {}{}`;` not found.",
                            if (matches!(value, None) && ! type_set) {"`:`, "} else {""},
                            if (matches!(value, None)) {"`=`, "} else {""}
                        ),
                        self.token.range.clone()
                    ).dump_error();
                }
                self.advance();
                
                return data::Node::new(
                    data::NodeType::InitializeVariable(mutable, name.unwrap(), Box::new(typ), Box::new(value)),
                    data::Range::new(start, end)
                );

            }
        }

        exception::ParserException::new(
            exception::ParserExceptionType::MissingToken,
            String::from("Expected `let` not found."),
            self.token.range.clone()
        ).dump_error();

    }





    fn start_type(&mut self, data : ParserData) -> data::Node {

        let     start = self.token.range.min.clone();
        let mut end   = self.token.range.max.clone();

        if (let data::TokenType::Identifier(name) = self.token.token.clone()) {
            self.advance();

            let mut bases = vec![name];

            while (matches!(self.token.token, data::TokenType::DoubleColon)) {
                self.advance();
                if (let data::TokenType::Identifier(sub_name) = self.token.token.clone()) {
                    bases.push(sub_name);
                    end = self.token.range.max.clone();
                    self.advance();
                } else {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        String::from("Expected Identifier not found."),
                        self.token.range.clone()
                    ).dump_error();
                }
            }

            let mut arguments = Vec::new();

            if (matches!(self.token.token, data::TokenType::LCarat)) {
                self.advance();

                if (! matches!(self.token.token, data::TokenType::RCarat)) {
                    arguments.push(self.start_type(data.clone()));
                    while (matches!(self.token.token, data::TokenType::Comma)) {
                        self.advance();
                        arguments.push(self.start_type(data.clone()));
                    }
                }

                if (! matches!(self.token.token, data::TokenType::RCarat)) {
                    exception::ParserException::new(
                        exception::ParserExceptionType::MissingToken,
                        format!("Expected {}, `>` not found.", if (arguments.len() >= 1) {"`,`"} else {"type"}),
                        self.token.range.clone()
                    ).dump_error();
                }
                self.advance();

            }

            return data::Node::new(
                data::NodeType::Type(data::Type::Base(bases), arguments),
                data::Range::new(start, end)
            );

        }

        exception::ParserException::new(
            exception::ParserExceptionType::MissingToken,
            String::from("Expected Identifier not found."),
            self.token.range.clone()
        ).dump_error();

    }





    fn start_atom(&mut self, data : ParserData) -> data::Node {
        let start = self.token.range.min.clone();
        
        if (matches!(self.token.token, data::TokenType::Minus)) {
            self.advance();
            let value = self.start_atom(data);
            return data::Node::new(
                data::NodeType::OppositeOperation(Box::new(value.clone())),
                data::Range::new(start, value.range.max)
            );
        }
        
        else if (matches!(self.token.token, data::TokenType::Bang)) {
            self.advance();
            let value = self.start_atom(data);
            return data::Node::new(
                data::NodeType::InvertOperation(Box::new(value.clone())),
                data::Range::new(start, value.range.max)
            );
        }

        let value = self.start_literal(data.clone());

        return value;

    }



    fn start_literal(&mut self, _data : ParserData) -> data::Node {
        let range = self.token.range.clone();
        let node  = match (self.token.token.clone()) {
            data::TokenType::Identifier(name) => data::NodeType::Literal(data::Literal::Name(name)),
            data::TokenType::Character(ch)    => data::NodeType::Literal(data::Literal::Character(ch)),
            data::TokenType::String(text)     => data::NodeType::Literal(data::Literal::String(text)),
            data::TokenType::Integer(value)   => data::NodeType::Literal(data::Literal::Integer(value)),
            data::TokenType::Float(value)     => data::NodeType::Literal(data::Literal::Float(value)),
            _                                 => {
                exception::ParserException::new(
                    exception::ParserExceptionType::MissingToken,
                    String::from("Expected Identifier, Character, String, Integer, Float not found."),
                    self.token.range.clone()
                ).dump_error();
            }
        };
        self.advance();
        return data::Node::new(
            node,
            range
        );
    }

}