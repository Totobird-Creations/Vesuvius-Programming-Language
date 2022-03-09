use std;
use colored::Colorize;

use crate::data;
use crate::lexer;



pub trait Exception {
    fn dump(&self) -> ! {
        let     position            = self.get_position();
        let     range               = self.get_range();
        let     real_text           = self.get_text();
        let     leading_erase_count = get_leading_erase_count(real_text.clone());
        let mut text                = real_text[leading_erase_count..(real_text.len())].to_string();
        while ([' ', '\t'].contains(&text.chars().nth(text.len() - 1).unwrap())) {
            text.pop();
        }
        let     left                = &text[0..(range.min.column - leading_erase_count - 1)];
        let     center              = &text[(range.min.column - leading_erase_count - 1)..(range.max.column - leading_erase_count)];
        let     right               = &text[(range.max.column - leading_erase_count)..(text.len())];
        let     prefix              = format!("{}", self.get_prefix());
        let     suffix              = format!("{}: {}", self.get_title(), self.get_message());
        let     repeat              = std::cmp::max(prefix.len(), suffix.len()) + 1;
        println!("\n{}\n   {} `{}`, {} {},\n{} {} {}, {} {}\n{}    {}{}{}\n{}    {}{}\n{} {}",
            format!("═ {} {}", prefix.bold(), "═".repeat(std::cmp::max(repeat - prefix.len(), 1))).red(),
            "File".blue(), self.get_filename().blue().bold(), "In".blue(), self.get_context().blue().bold(),
            "╔═".purple(), "Line".green(), (position.1 + 1).to_string().green().bold(), "Column".green(), (position.0).to_string().green().bold(),
            "║".purple(), left.yellow(), center.yellow().bold(), right.yellow(),
            "║".purple(), " ".repeat(range.min.column - leading_erase_count - 1), "^".repeat((range.max.column - range.min.column + 1) as usize).yellow(),
            "╚".purple(), format!("{} {}", suffix.bold(), "═".repeat(std::cmp::max(repeat - suffix.len(), 1))).red()
        );
        std::process::exit(1);
    }
    fn get_prefix(&self) -> String;
    fn get_filename(&self) -> String;
    fn get_context(&self) -> String;
    fn get_position(&self) -> (usize, usize);
    fn get_text(&self) -> String;
    fn get_range(&self) -> data::Range;
    fn get_title(&self) -> String;
    fn get_message(&self) -> String;
}
pub trait ExceptionType {
    fn get_name(&self) -> String;
}



pub struct InternalException {
    message : String
}
impl InternalException {
    pub fn new(message : String) -> InternalException {
        return InternalException {
            message : message
        };
    }
}
impl Exception for InternalException {
    fn get_prefix(&self) -> String {
        return String::from("InternalException");
    }
    fn get_filename(&self) -> String {
        return String::from("<Void>");
    }
    fn get_context(&self) -> String {
        return String::from("<Void>");
    }
    fn get_position(&self) -> (usize, usize) {
        return (0, 0);
    }
    fn get_text(&self) -> String {
        return String::new();
    }
    fn get_range(&self) -> data::Range {
        return data::Range {
            min : data::Position::new(0, 0, 0, String::from("<Void>")),
            max : data::Position::new(0, 0, 0, String::from("<Void>"))
        };
    }
    fn get_title(&self) -> String {
        return String::from("Internal Exception");
    }
    fn get_message(&self) -> String {
        return self.message.clone();
    }
}



pub struct CommandLineException<T : ExceptionType> {
    exception_type : T,
    message        : String,
    arguments      : crate::Arguments,
    index          : usize
}
impl<T : ExceptionType> CommandLineException<T> {
    pub fn new(exception_type : T, message : String, arguments : crate::Arguments, index : usize) -> CommandLineException<T> {
        return CommandLineException {
            exception_type : exception_type,
            message        : message,
            arguments      : arguments,
            index          : index
        };
    }
}
impl<T : ExceptionType> Exception for CommandLineException<T> {
    fn get_prefix(&self) -> String {
        return String::from("CommandLineException");
    }
    fn get_filename(&self) -> String {
        return String::from("<Void>");
    }
    fn get_context(&self) -> String {
        return String::from("Command Line");
    }
    fn get_position(&self) -> (usize, usize) {
        return (0, self.arguments.get_column(self.index));
    }
    fn get_text(&self) -> String {
        return self.arguments.to_string();
    }
    fn get_range(&self) -> data::Range {
        let column = self.arguments.get_column(self.index);
        return data::Range {
            min : data::Position::new(column, 0, column, String::from("<Void>")),
            max : data::Position::new(column + self.arguments.get_length(self.index) - 1, 0, column + self.arguments.get_length(self.index) - 1, String::from("<Void>"))
        };
    }
    fn get_title(&self) -> String {
        return self.exception_type.get_name();
    }
    fn get_message(&self) -> String {
        return self.message.clone();
    }
}

pub enum CommandLineExceptionType {
    
    FileFailedToRead

}
impl ExceptionType for CommandLineExceptionType {
    fn get_name(&self) -> String {
        return String::from(match (self) {

            CommandLineExceptionType::FileFailedToRead => "FileFailedToRead"

        });
    }
}



pub struct LexerException<T : ExceptionType> {
    exception_type : T,
    message        : String,
    script         : String,
    range          : data::Range
}
impl<T : ExceptionType> LexerException<T> {
    pub fn new(lexer : lexer::Lexer, exception_type : T, message : String, script : String, range : data::Range) -> LexerException<T> {
        return LexerException {
            exception_type : exception_type,
            message        : message,
            script         : script,
            range          : range
        };
    }
}
impl<T : ExceptionType> Exception for LexerException<T> {
    fn get_prefix(&self) -> String {
        return String::from("LexerException");
    }
    fn get_filename(&self) -> String {
        return String::from(self.range.min.filename.clone());
    }
    fn get_context(&self) -> String {
        return String::from("<Void>");
    }
    fn get_position(&self) -> (usize, usize) {
        return (self.range.min.column, self.range.min.line);
    }
    fn get_text(&self) -> String {
        return String::from(self.script.split("\n").collect::<Vec<&str>>()[self.range.min.line]);
    }
    fn get_range(&self) -> data::Range {
        return data::Range {
            min : data::Position::new(self.range.min.index, 0, self.range.min.column, self.range.min.filename.clone()),
            max : data::Position::new(self.range.max.index, 0, self.range.max.column, self.range.min.filename.clone())
        };
    }
    fn get_title(&self) -> String {
        return self.exception_type.get_name();
    }
    fn get_message(&self) -> String {
        return self.message.clone();
    }
}

pub enum LexerExceptionType {

    IllegalCharacter,
    MissingCharacter,
    InvalidEscape

}
impl ExceptionType for LexerExceptionType {
    fn get_name(&self) -> String {
        return String::from(match (self) {

            LexerExceptionType::IllegalCharacter => "IllegalCharacter",
            LexerExceptionType::MissingCharacter => "MissingCharacter",
            LexerExceptionType::InvalidEscape    => "InvalidEscape"

        });
    }
}



pub struct ParserException<T : ExceptionType> {
    exception_type : T,
    message        : String,
    script         : String,
    range          : data::Range
}
impl<T : ExceptionType> ParserException<T> {
    pub fn new(exception_type : T, message : String, script : String, range : data::Range) -> ParserException<T> {
        return ParserException {
            exception_type : exception_type,
            message        : message,
            script         : script,
            range          : range
        };
    }
}
impl<T : ExceptionType> Exception for ParserException<T> {
    fn get_prefix(&self) -> String {
        return String::from("ParserException");
    }
    fn get_filename(&self) -> String {
        return String::from(self.range.min.filename.clone());
    }
    fn get_context(&self) -> String {
        return String::from("<Void>");
    }
    fn get_position(&self) -> (usize, usize) {
        return (self.range.min.column, self.range.min.line);
    }
    fn get_text(&self) -> String {
        return String::from(self.script.split("\n").collect::<Vec<&str>>()[self.range.min.line]);
    }
    fn get_range(&self) -> data::Range {
        return data::Range {
            min : data::Position::new(self.range.min.index, 0, self.range.min.column, self.range.min.filename.clone()),
            max : data::Position::new(self.range.max.index, 0, self.range.max.column, self.range.min.filename.clone())
        };
    }
    fn get_title(&self) -> String {
        return self.exception_type.get_name();
    }
    fn get_message(&self) -> String {
        return self.message.clone();
    }
}

pub enum ParserExceptionType {

    MissingToken

}
impl ExceptionType for ParserExceptionType {
    fn get_name(&self) -> String {
        return String::from(match (self) {

            ParserExceptionType::MissingToken => "MissingToken"

        });
    }
}



pub fn get_leading_erase_count(text : String) -> usize {
    let     chars = text.chars().collect::<Vec<char>>();
    let mut count = 0;
    while ((count < chars.len()) && ([' ','\t'].contains(&chars[count]))) {
        count += 1;
    }
    return count;
}
