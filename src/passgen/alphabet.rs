use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Alphabet {
    Full,
    LowerCase,
    UpperCase,
    Digits,
    SpecialChars,
}

const FULL: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
const LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SPECIAL_CHARS: &str = "!@#$%^&*";

impl Alphabet {
    pub fn as_str(&self) -> &str {
        match self {
            Alphabet::Full => FULL,
            Alphabet::LowerCase => LOWER_CASE,
            Alphabet::UpperCase => UPPER_CASE,
            Alphabet::Digits => DIGITS,
            Alphabet::SpecialChars => SPECIAL_CHARS,
        }
    }
}
