pub mod alphabet;
pub mod checker;
pub mod generate;

#[derive(Debug, PartialEq)]
pub struct Password {
    pub value: String,
}
