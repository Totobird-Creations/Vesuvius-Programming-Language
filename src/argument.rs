use crate::exception;
use crate::exception::Exception;



pub fn parse(call_argument : String, full_arguments : Vec<String>) -> () {
    if (full_arguments.len() == 0) {
        crate::version();
    }

    parse_config(call_argument, full_arguments.clone(), full_arguments);
}


fn parse_config(call_argument : String, full_arguments : Vec<String>, arguments : Vec<String>) -> () {
    if (arguments[0].starts_with("--cfg.")) {
        exception::CommandLineException::new(
            exception::CommandLineExceptionType::FutureFeature,
            format!("Configs are not yet supported."),
            full_arguments.clone(),
            full_arguments.len() - arguments.len()
        ).dump_error();
    }

    parse_flags(call_argument, full_arguments, arguments);
}


fn parse_flags(call_argument : String, full_arguments : Vec<String>, arguments : Vec<String>) {
    if (arguments[0].starts_with("-")) {
        if (arguments.len() >= 2) {
            exception::CommandLineException::new(
                exception::CommandLineExceptionType::Argument,
                format!("Non config flags take 0 arguments. {} given.", arguments.len() - 1),
                full_arguments.clone(),
                full_arguments.len() - arguments.len()
            ).dump_error();
        }
        if (["-h", "--help"].contains(&arguments[0].as_str())) {
            crate::help(call_argument);
        }
        else if (["-v", "--version"].contains(&arguments[0].as_str())) {
            crate::version();
            std::process::exit(0);
        } else {
            exception::CommandLineException::new(
                exception::CommandLineExceptionType::Argument,
                format!("Invalid flag `{}`.", arguments[0]),
                full_arguments.clone(),
                full_arguments.len() - arguments.len()
            ).dump_error();
        }
    }

    if (arguments.len() >= 2) {
        exception::CommandLineException::new(
            exception::CommandLineExceptionType::Argument,
            format!("Non config flags take 0 arguments. {} given.", arguments.len() - 1),
            full_arguments.clone(),
            full_arguments.len() - arguments.len()
        ).dump_error();
    }
    crate::run(full_arguments, 0)
}
