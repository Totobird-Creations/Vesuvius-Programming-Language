use crate::data;



#[derive(Clone)]
pub struct Validator {
    nodes           : Vec<data::Node>
}
impl Validator {

    pub fn calculate(nodes : Vec<data::Node>) -> () {
        let mut validator = Validator {
            nodes : nodes
        };
        
    }

}