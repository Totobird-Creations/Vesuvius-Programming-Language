use std;

use crate::data;
use crate::exception;
use crate::exception::Exception;


pub struct Object {
    value : ObjectType
}
impl Object {

    pub fn new(value : ObjectType) -> Object {
        return Object {
            value : value
        };
    }

}


pub enum ObjectType {

    Module(std::collections::HashMap<String, ObjectType>),

    Function(Vec<(String, data::Node)>, data::Node, Vec<data::Node>)

}
