#![allow(unused_parens)]
#![feature(let_chains)]

use std;
use colored::Colorize;

mod data;
mod exception;
use exception::Exception;
mod run;
mod lexer;
mod parser;



pub struct Arguments {
    filename   : String,
    positioned : Vec<String>,
    named      : std::collections::HashMap<String, Option<String>>
}
impl Arguments {
    pub fn get() -> Arguments {
        let mut args     = std::env::args().collect::<Vec<String>>();
        let     filename = args[0].clone();
        args.remove(0);
        let mut positioned = Vec::new();
        let mut named      = std::collections::HashMap::new();
        for arg in args {
            if (arg.as_str().starts_with("-")) {
                let option = arg[1..arg.len()].split("=").collect::<Vec<&str>>();
                let value = if (option.len() >= 2) {
                    Some(option[1..(option.len() - 1)].join("="))
                } else {
                    None
                };
                named.insert(option[0].to_string(), value);
            } else {
                positioned.push(arg);
            }
        }
        return Arguments {
            filename   : filename,
            positioned : positioned,
            named      : named
        }
    }
    pub fn to_string(&self) -> String {
        let mut res = Vec::new();
        for value in self.positioned.clone() {
            res.push(value);
        };
        for key in self.named.keys() {
            let value = self.named[key].clone();
            res.push(String::from("-") + key.clone().as_str());
            match (value) {
                Some(text) => res.push(text),
                None       => ()
            };
        };
        return res.join(" ");
    }
    pub fn get_column(&self, mut index : usize) -> usize {
        let mut column = 0;
        for value in self.positioned.clone() {
            if (index <= 0) {
                return column;
            }
            index -= 1;
            column += value.len() + 1;
        }
        exception::InternalException::new(
            String::from("Invalid Index")
        ).dump();
    }
    pub fn get_length(&self, mut index : usize) -> usize {
        for value in self.positioned.clone() {
            if (index <= 0) {
                return value.len();
            }
            index -= 1;
        }
        exception::InternalException::new(
            String::from("Invalid Index")
        ).dump();
    }
}



fn main() -> () {
    let arguments = Arguments::get();
    if (arguments.named.contains_key("v") && matches!(arguments.named["v"], None)) {
        version();
    };
    if (arguments.named.contains_key("h") && matches!(arguments.named["h"], None)) {
        help(arguments);
    };
    if (arguments.positioned.len() <= 0) {
        version();
    };
    run::run(arguments.positioned[0].clone(), read(arguments));
}
fn version() -> ! {
    println!("{} {}",
        "Vesuvius Programming Language".red().bold(),
        format!("v{}", env!("CARGO_PKG_VERSION").bold()).yellow()
    );
    std::process::exit(0);
}
fn help(arguments : Arguments) -> ! {
    println!("{}\n\n{}:\n  {}\n  {}\n\n{}:\n  {}\n  {}",
        "Vesuvius Parser & Interpreter".red().bold(),
        "USAGE".blue().bold(),
        format!("{} {}", arguments.filename, "[OPTIONS]*").cyan(),
        format!("{} {} {}", arguments.filename, "[FILENAME]", "[OPTIONS]*").cyan(),
        "OPTIONS".blue().bold(),
        format!("{} : {}", "-h".bold(), "Display this help message.").cyan(),
        format!("{} : {}", "-v".bold(), "Display the version number.").cyan()
    );
    std::process::exit(0);
}



fn read(arguments : Arguments) -> String {
    let filename = arguments.positioned[0].clone();
    let contents = match std::fs::read_to_string(filename.clone()) {
        Ok(contents) => contents,
        Err(_e)      => {
            exception::CommandLineException::new(
                exception::CommandLineExceptionType::FileFailedToRead,
                format!("File `{}` was not found", filename),
                arguments,
                0
            ).dump();
        }
    };

    return contents;
}
