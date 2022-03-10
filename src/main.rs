#![allow(unused_parens)]
#![feature(let_chains)]

use std;
use colored::Colorize;

mod argument;
mod data;
mod exception;
use exception::Exception;
mod run;
mod lexer;
mod parser;



fn main() -> () {
    let arguments = std::env::args().collect::<Vec<String>>();
    argument::parse(arguments[0].clone(), arguments[1..(arguments.len())].to_vec());
}


fn run(full_arguments : Vec<String>, index : usize) -> ! {
    run::run(full_arguments[index].clone(), read(full_arguments, index));
    std::process::exit(0);
}


fn version() -> ! {
    println!("{a}\n {} {}\n {}\n{a}",
        "Vesuvius Programming Language".red().bold(),
        format!("v{}", get_version_number().bold()).yellow(),
        "Totobird Creations".bright_green(),
        a = "══════════════════════════════════════".magenta().dimmed()
    );
    std::process::exit(0);
}


fn help(arguments : Vec<String>) -> ! {
    println!("{}\n\n{}:\n  {}\n  {}\n\n{}:\n  {}\n  {}",
        "Vesuvius Parser & Interpreter".red().bold(),
        "USAGE".blue().bold(),
        format!("{} {}", arguments[0], "[OPTIONS]*").cyan(),
        format!("{} {} {}", arguments[0], "[FILENAME]", "[OPTIONS]*").cyan(),
        "OPTIONS".blue().bold(),
        format!("{} : {}", "-h".bold(), "Display this help message.").cyan(),
        format!("{} : {}", "-v".bold(), "Display the version number.").cyan()
    );
    std::process::exit(0);
}



fn get_version_number() -> String {
    return String::from(env!("CARGO_PKG_VERSION"));
}

fn read(arguments : Vec<String>, filename_index : usize) -> String {
    let contents = match std::fs::read_to_string(arguments[filename_index].clone()) {
        Ok(contents) => contents,
        Err(_e)      => {
            exception::CommandLineException::new(
                exception::CommandLineExceptionType::FileFailedToRead,
                format!("File `{}` was not found", arguments[filename_index]),
                arguments,
                filename_index
            ).dump_error();
        }
    };

    return contents;
}
