use std;

use crate::data;
use crate::exception;
use crate::exception::Exception;



pub struct Validator {}
impl Validator {

    pub fn calculate(nodes : Vec<data::Node>) -> () {
        let mut validator = Validator {};
        let     context   = data::Context::new(String::from("Global"), None);
        for node in nodes {
            validator.start(context.clone(), node);
        }
    }

    pub fn start(&mut self, context : data::Context, node : data::Node) -> data::Object {
        return match (node.node) {
            data::NodeType::ExternalImport(name)                          => self.start_external_import(context, node.range, name),
            data::NodeType::LocalImport(name)                             => self.start_local_import(context, node.range, name),
            data::NodeType::DefineFunction(name, args, return_type, body) => self.start_define_function(context, node.range, name, *args, *return_type, *body),
            data::NodeType::InitializeVariable(mutable, name, typ, value) => self.start_initialize_variable(context, node.range, mutable, name, *typ, *value),
            _                                                             => {
                exception::InternalException::new(
                    String::from("Invalid global node.")
                ).dump_critical();
            }
        };
    }
 


    pub fn start_external_import(&mut self, context : data::Context, range : data::Range, name : String) -> data::Object {
        /*if (self.contains_name(name.clone())) {
            exception::ValidatorException::new(
                exception::ValidatorExceptionType::Name,
                format!("Name `{}` is already defined.", name),
                range.clone()
            ).dump_invalid();
        }*/

        panic!("");
    }
 


    pub fn start_local_import(&mut self, context : data::Context, range : data::Range, name : String) -> data::Object {
        /*if (self.contains_name(name.clone())) {
            exception::ValidatorException::new(
                exception::ValidatorExceptionType::Name,
                format!("Name `{}` is already defined.", name),
                range.clone()
            ).dump_invalid();
        }*/

        panic!("");
    }
 


    pub fn start_define_function(&mut self, context : data::Context, range : data::Range, name : String, args : Vec<(String, data::Node)>, return_type : data::Node, body : Vec<data::Node>) -> data::Object {
        panic!("");
    }
 


    pub fn start_initialize_variable(&mut self, context : data::Context, range : data::Range, mutable : bool, name : String, typ : data::Node, value : Option<data::Node>) -> data::Object {
        panic!("");
    }

}