#[allow(dead_code)]
#[derive(Clone)]
pub enum Literal {
    Name(String),
    Character(char),
    String(String),
    Integer(i64),
    Float(f64)
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", match (self) {

            Literal::Name(name)      => name.clone(),
            Literal::Character(ch)   => format!("'{}'", ch),
            Literal::String(text)    => format!("\"{}\'", text),
            Literal::Integer(number) => number.to_string(),
            Literal::Float(number)   => number.to_string()

        });
    }
}


#[derive(Clone)]
pub enum Type {
    Base(Vec<String>),
    Inferred,
    Cancelled
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", match (self) {
            Type::Base(parts) => parts.join("::"),
            Type::Inferred    => String::from("?"),
            Type::Cancelled   => String::from("Cancelled")
        });
    }
}
