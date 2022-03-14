use crate::data;



#[derive(Clone)]
pub struct Context {
    pub name      : String,
    pub parent    : Box<Option<(data::Range, Context)>>,
}
impl Context {

    pub fn new(name : String, parent : Option<(data::Range, Context)>) -> Context {
        return Context {
            name   : name,
            parent : Box::new(parent)
        };
    }

}
