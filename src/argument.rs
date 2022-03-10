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
            exception::CommandLineExceptionType::FileFailedToRead,
            format!("Configs are not yet supported"),
            full_arguments.clone(),
            full_arguments.len() - arguments.len()
        ).dump_error();
    }

    parse_flags(call_argument, full_arguments, arguments);
}


fn parse_flags(call_argument : String, full_arguments : Vec<String>, arguments : Vec<String>) {
    panic!("Flags");
}
