use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Alphabet {
    Full,
    LowerCase,
    UpperCase,
    Digits,
    SpecialChars,
    #[clap(skip)]
    Custom(String),
}

const FULL: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
const LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SPECIAL_CHARS: &str = "!@#$%^&*";

impl Default for Alphabet {
    fn default() -> Self {
        Alphabet::Full
    }
}

impl Alphabet {
    pub fn as_str(&self) -> &str {
        match self {
            Alphabet::Full => FULL,
            Alphabet::LowerCase => LOWER_CASE,
            Alphabet::UpperCase => UPPER_CASE,
            Alphabet::Digits => DIGITS,
            Alphabet::SpecialChars => SPECIAL_CHARS,
            Alphabet::Custom(s) => s,
        }
    }

    pub fn contains(&self, c: char) -> bool {
        self.as_str().contains(c)
    }

    pub fn len(&self) -> usize {
        self.as_str().len()
    }
}
