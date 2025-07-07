use crate::passgen::Password;
use crate::passgen::alphabet::Alphabet;
use rand::Rng;
use std::borrow::Cow;

impl<'a> Password<'a> {
    pub fn generate(len: usize, alphabet: &Alphabet) -> Password<'static> {
        let mut rng = rand::rng();
        let alphabet_str = alphabet.as_str();
        let chars: Vec<char> = alphabet_str.chars().collect();
        if chars.is_empty() {
            return Password {
                value: Cow::Borrowed(""),
            };
        }
        let password: String = (0..len)
            .map(|_| {
                let idx = rng.random_range(0..chars.len());
                chars[idx]
            })
            .collect();
        Password {
            value: Cow::Owned(password),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::passgen::alphabet::Alphabet::Custom;

    #[test]
    fn test_generate() {
        let alphabet = Alphabet::Full;
        let password = Password::generate(12, &alphabet);
        assert_eq!(password.value.len(), 12);
        for c in password.value.chars() {
            assert!(alphabet.contains(c));
        }
    }

    #[test]
    fn test_generate_empty() {
        let alphabet = Custom("abc".to_string());
        let password = Password::generate(0, &alphabet);
        assert_eq!(password.value.len(), 0);
    }
}
