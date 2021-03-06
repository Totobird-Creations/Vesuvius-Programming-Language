use std;
use colored;
use colored::Colorize;

use crate::data;



#[derive(Clone, PartialEq)]
pub enum ExceptionLevel {
    Warning,
    Error,
    Critical
}



pub trait Exception {
    fn dump(&self, level : ExceptionLevel) -> () {
        let level_name          = match (level) {
            ExceptionLevel::Warning  => "Warning",
            ExceptionLevel::Error    => "Exception",
            ExceptionLevel::Critical => "CriticalException"
        };
        /*let     position            = self.get_position();
        let     range               = self.get_range();
        let     real_text           = self.get_text();
        let     leading_erase_count = get_leading_erase_count(real_text.clone());
        let mut text                = real_text[leading_erase_count..(real_text.len())].to_string();
        let     min_column          = range.min.column;
        let     max_column          = if (range.max.line == range.min.line) {range.max.column} else {text.len() + 1};
        // Line 4
        while ([' ', '\t'].contains(&text.chars().nth(text.len() - 1).unwrap())) {
            text.pop();
        }
        let     left                = &text[0..(min_column - leading_erase_count - 1)];
        let     center              = &text[(min_column - leading_erase_count - 1)..(max_column - leading_erase_count)];
        let     right               = &text[(max_column - leading_erase_count)..(text.len())];
        // Line 5
        let     underline_count     = max_column - min_column + 1;
        let     underline           = "▔".repeat(underline_count);
        // Print
        println!("\n{}\n  {} `{}`, {} {},\n  {} {}, {} {}\n    {}{}{}\n    {}{}\n{}\n",
            self.colourize(format!(" ═ {} {} ", prefix.bold(), "═".repeat(std::cmp::max(repeat - prefix.len(), 1))), level.clone()),
            "File".blue(), self.get_filename().blue().bold(), "In".blue(), self.get_context().blue().bold(),
            "Line".cyan(), (position.1 + 1).to_string().cyan().bold(), "Column".cyan(), (position.0).to_string().cyan().bold(),
            left.green(), center.green().bold(), right.green(),
            " ".repeat(min_column - leading_erase_count - 1), underline.green(),
            self.colourize(format!(" ═ {} {} ", suffix.bold(), "═".repeat(std::cmp::max(repeat - suffix.len(), 1))), level)
        );*/
        let prefix = format!("{}{}", self.get_prefix(), level_name);
        let suffix = format!("{}{}: {}", self.get_title(), level_name, self.get_message());
        let repeat = std::cmp::max(prefix.len(), suffix.len()) + 1;
        println!("\n{}\n{}\n",
            self.colourize(format!(" ═ {} {} ", prefix.bold(), "═".repeat(std::cmp::max(repeat - prefix.len(), 1))), level.clone()),
            self.colourize(format!(" ═ {} {} ", suffix.bold(), "═".repeat(std::cmp::max(repeat - suffix.len(), 1))), level)
    )
    }
    fn dump_warning(&self) -> () {
        self.dump(ExceptionLevel::Warning);
    }
    fn dump_invalid(&self) -> () {
        self.dump(ExceptionLevel::Error);
    }
    fn dump_error(&self) -> ! {
        self.dump(ExceptionLevel::Error);
        std::process::exit(1);
    }
    fn dump_critical(&self) -> ! {
        self.dump(ExceptionLevel::Critical);
        std::process::exit(1);
    }
    fn colourize(&self, text : String, level : ExceptionLevel) -> colored::ColoredString {
        return match (level) {
            ExceptionLevel::Warning  => text.yellow(),
            ExceptionLevel::Error    => text.red(),
            ExceptionLevel::Critical => text.white().on_red()
        };
    }
    fn get_prefix(&self) -> String;
    fn get_filename(&self) -> String;
    fn get_context(&self) -> data::Context;
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
        return String::from("Internal");
    }
    fn get_filename(&self) -> String {
        return String::from("<Void>");
    }
    fn get_context(&self) -> data::Context {
        return data::Context::new(String::from("<Void>"), None);
    }
    fn get_position(&self) -> (usize, usize) {
        return (0, 0);
    }
    fn get_text(&self) -> String {
        return String::new();
    }
    fn get_range(&self) -> data::Range {
        return data::Range {
            min : data::Position::new(0, 0, 0, String::from("<Void>"), String::from("<Void>")),
            max : data::Position::new(0, 0, 0, String::from("<Void>"), String::from("<Void>"))
        };
    }
    fn get_title(&self) -> String {
        return String::from("Internal");
    }
    fn get_message(&self) -> String {
        return self.message.clone();
    }
}



pub struct CommandLineException {
    exception_type : CommandLineExceptionType,
    message        : String,
    arguments      : Vec<String>,
    index          : usize
}
impl CommandLineException {
    pub fn new(exception_type : CommandLineExceptionType, message : String, arguments : Vec<String>, index : usize) -> CommandLineException {
        return CommandLineException {
            exception_type : exception_type,
            message        : message,
            arguments      : arguments,
            index          : index
        };
    }
}
impl Exception for CommandLineException {
    fn get_prefix(&self) -> String {
        return String::from("CommandLine");
    }
    fn get_filename(&self) -> String {
        return String::from("<Void>");
    }
    fn get_context(&self) -> data::Context {
        return data::Context::new(String::from("<Command Line>"), None);
    }
    fn get_position(&self) -> (usize, usize) {
        let mut column = 0;
        for i in 0..(self.arguments.len()) {
            let argument = self.arguments[i].clone();
            if (i == self.index) {
                break;
            }
            column += argument.len() + 1;
        };
        return (0, column);
    }
    fn get_text(&self) -> String {
        return self.arguments.join(" ");
    }
    fn get_range(&self) -> data::Range {
        let mut column = 1;
        for i in 0..(self.arguments.len()) {
            let argument = self.arguments[i].clone();
            if (i == self.index) {
                break;
            }
            column += argument.len() + 1;
        };
        let arg_len = self.arguments[self.index].len();
        return data::Range {
            min : data::Position::new(column, 0, column, String::from("<Void>"), String::from("<Void>")),
            max : data::Position::new(column + arg_len - 1, 0, column + arg_len - 1, String::from("<Void>"), String::from("<Void>"))
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
    
    FileFailedToRead,
    Argument,

    FutureFeature

}
impl ExceptionType for CommandLineExceptionType {
    fn get_name(&self) -> String {
        return String::from(match (self) {

            CommandLineExceptionType::FileFailedToRead => "FileFailedToRead",
            CommandLineExceptionType::Argument         => "Argument",
            
            CommandLineExceptionType::FutureFeature    => "FutureFeature"

        });
    }
}



pub struct LexerException {
    exception_type : LexerExceptionType,
    message        : String,
    range          : data::Range
}
impl LexerException {
    pub fn new(exception_type : LexerExceptionType, message : String, range : data::Range) -> LexerException {
        return LexerException {
            exception_type : exception_type,
            message        : message,
            range          : range
        };
    }
}
impl Exception for LexerException {
    fn get_prefix(&self) -> String {
        return String::from("Lexer");
    }
    fn get_filename(&self) -> String {
        return String::from(self.range.min.filename.clone());
    }
    fn get_context(&self) -> data::Context {
        return data::Context::new(String::from("<Lexer>"), None);
    }
    fn get_position(&self) -> (usize, usize) {
        return (self.range.min.column, self.range.min.line);
    }
    fn get_text(&self) -> String {
        return String::from(self.range.min.script.split("\n").collect::<Vec<&str>>()[self.range.min.line]);
    }
    fn get_range(&self) -> data::Range {
        return data::Range {
            min : data::Position::new(self.range.min.index, 0, self.range.min.column, self.range.min.filename.clone(), String::from("<Void>")),
            max : data::Position::new(self.range.max.index, 0, self.range.max.column, self.range.min.filename.clone(), String::from("<Void>"))
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



pub struct ParserException {
    exception_type : ParserExceptionType,
    message        : String,
    range          : data::Range
}
impl ParserException {
    pub fn new(exception_type : ParserExceptionType, message : String, range : data::Range) -> ParserException {
        return ParserException {
            exception_type : exception_type,
            message        : message,
            range          : range
        };
    }
}
impl Exception for ParserException {
    fn get_prefix(&self) -> String {
        return String::from("Parser");
    }
    fn get_filename(&self) -> String {
        return String::from(self.range.min.filename.clone());
    }
    fn get_context(&self) -> data::Context {
        return data::Context::new(String::from("<Parser>"), None);
    }
    fn get_position(&self) -> (usize, usize) {
        return (self.range.min.column, self.range.min.line);
    }
    fn get_text(&self) -> String {
        return String::from(self.range.min.script.split("\n").collect::<Vec<&str>>()[self.range.min.line]);
    }
    fn get_range(&self) -> data::Range {
        return data::Range {
            min : data::Position::new(self.range.min.index, 0, self.range.min.column, self.range.min.filename.clone(), String::from("<Void>")),
            max : data::Position::new(self.range.max.index, 0, self.range.max.column, self.range.min.filename.clone(), String::from("<Void>"))
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

    MissingToken,
    InvalidHeader,
    InvalidMutability

}
impl ExceptionType for ParserExceptionType {
    fn get_name(&self) -> String {
        return String::from(match (self) {

            ParserExceptionType::MissingToken      => "MissingToken",
            ParserExceptionType::InvalidHeader     => "InvalidHeader",
            ParserExceptionType::InvalidMutability => "InvalidMutability"

        });
    }
}



pub struct ValidatorException {
    exception_type : ValidatorExceptionType,
    message        : String,
    range          : data::Range,
    context        : data::Context
}
impl ValidatorException {
    pub fn new(exception_type : ValidatorExceptionType, message : String, range : data::Range, context : data::Context) -> ValidatorException {
        return ValidatorException {
            exception_type : exception_type,
            message        : message,
            range          : range,
            context        : context
        };
    }
}
impl Exception for ValidatorException {
    fn get_prefix(&self) -> String {
        return String::from("Validator");
    }
    fn get_filename(&self) -> String {
        return String::from(self.range.min.filename.clone());
    }
    fn get_context(&self) -> data::Context {
        return self.context.clone();
    }
    fn get_position(&self) -> (usize, usize) {
        return (self.range.min.column, self.range.min.line);
    }
    fn get_text(&self) -> String {
        return String::from(self.range.min.script.split("\n").collect::<Vec<&str>>()[self.range.min.line]);
    }
    fn get_range(&self) -> data::Range {
        return data::Range {
            min : data::Position::new(self.range.min.index, 0, self.range.min.column, self.range.min.filename.clone(), String::from("<Void>")),
            max : data::Position::new(self.range.max.index, 0, self.range.max.column, self.range.min.filename.clone(), String::from("<Void>"))
        };
    }
    fn get_title(&self) -> String {
        return self.exception_type.get_name();
    }
    fn get_message(&self) -> String {
        return self.message.clone();
    }
}

pub enum ValidatorExceptionType {

    Name

}
impl ExceptionType for ValidatorExceptionType {
    fn get_name(&self) -> String {
        return String::from(match (self) {

            ValidatorExceptionType::Name  => "Name"

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
