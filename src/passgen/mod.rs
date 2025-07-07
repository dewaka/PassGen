pub mod alphabet;
pub mod checker;
pub mod commonwords;
pub mod generate;
pub mod passphrase;
pub mod wordlist;
use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub struct Password<'a> {
    pub value: Cow<'a, str>,
}
