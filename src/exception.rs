use std;
use colored::Colorize;

use crate::data;



pub trait Exception {
    fn dump(&self) -> ! {
        let position = self.get_position();
        let range    = self.get_range();
        let text     = self.get_text();
        let left     = &text[(0)..((range.min - if (range.min > 0) {1} else {0}) as usize)];
        let center   = &text[(range.min as usize)..((range.max + 1) as usize)];
        let right    = &text[((range.max + 1) as usize)..(text.len() as usize)];
        let prefix   = format!("{}", self.get_prefix());
        let suffix   = format!("{}: {}", self.get_title(), self.get_message());
        let repeat   = std::cmp::max(prefix.len(), suffix.len()) + 1;
        println!("\n{}\n  {} `{}`, {} {},\n  {} {}, {} {}\n    {}{}{}\n    {}{}\n{}",
            format!("= {} {}", prefix.bold(), "=".repeat(std::cmp::max(repeat - prefix.len(), 1))).red(),
            "File".blue(), self.get_filename().blue().bold(), "In".blue(), self.get_context().blue().bold(),
            "Line".green(), position.1.to_string().green().bold(), "Column".green(), position.0.to_string().green().bold(),
            left.yellow(), center.yellow().bold(), right.yellow(),
            " ".repeat(range.min as usize), "^".repeat((range.max - range.min + 1) as usize).yellow(),
            format!("= {} {}", suffix.bold(), "=".repeat(std::cmp::max(repeat - suffix.len(), 1))).red()
        );
        std::process::exit(1);
    }
    fn get_prefix(&self) -> String;
    fn get_filename(&self) -> String;
    fn get_context(&self) -> String;
    fn get_position(&self) -> (u64, u64);
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
    fn get_position(&self) -> (u64, u64) {
        return (0, 0);
    }
    fn get_text(&self) -> String {
        return String::new();
    }
    fn get_range(&self) -> data::Range {
        return data::Range {
            min : 0,
            max : 0
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
    index          : u64
}
impl<T : ExceptionType> CommandLineException<T> {
    pub fn new(exception_type : T, message : String, arguments : crate::Arguments, index : u64) -> CommandLineException<T> {
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
    fn get_position(&self) -> (u64, u64) {
        return (0, self.arguments.get_column(self.index));
    }
    fn get_text(&self) -> String {
        return self.arguments.to_string();
    }
    fn get_range(&self) -> data::Range {
        let column = self.arguments.get_column(self.index);
        return data::Range {
            min : column,
            max : column + self.arguments.get_length(self.index) - 1
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
