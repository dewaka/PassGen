pub mod alphabet;
pub mod checker;
pub mod generate;
pub mod passphrase;
pub mod wordlist;

#[derive(Debug, PartialEq)]
pub struct Password {
    pub value: String,
}
