use crate::passgen::Password;
use crate::passgen::alphabet::Alphabet;
use rand::Rng;

impl Password {
    pub fn generate(len: usize, alphabet: &Alphabet) -> Self {
        let mut rng = rand::rng();
        let password: String = (0..len)
            .map(|_| {
                let idx = rng.random_range(0..alphabet.len());
                alphabet.as_str().chars().nth(idx).unwrap()
            })
            .collect();
        Password { value: password }
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
