use crate::passgen::Password;
use crate::passgen::alphabet::Alphabet;

#[derive(Debug, PartialEq)]
pub enum Classification {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl Password {
    pub fn entropy(&self, alphabet: usize) -> f64 {
        let length = self.value.len() as f64;
        if length == 0.0 || alphabet == 0 {
            return 0.0;
        }
        let entropy = length * (alphabet as f64).log2();
        entropy
    }

    pub fn classify(&self, alphabet: &Alphabet) -> Result<Classification, anyhow::Error> {
        if !self.value.chars().all(|c| alphabet.contains(c)) {
            return Err(anyhow::anyhow!(
                "Password contains characters not in the specified alphabet"
            ));
        }

        let alphabet = alphabet.len();

        let entropy = self.entropy(alphabet);
        if entropy < 28.0 {
            Ok(Classification::Weak)
        } else if entropy < 40.0 {
            Ok(Classification::Medium)
        } else if entropy < 60.0 {
            Ok(Classification::Strong)
        } else {
            Ok(Classification::VeryStrong)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy() {
        let password = Password {
            value: "password".to_string(),
        };
        // rewrite above with an epsilon comparison
        assert!((password.entropy(26) - 37.6).abs() < 0.01);
    }

    #[test]
    fn test_classify() {
        use crate::passgen::alphabet::Alphabet;

        // Test Weak classification (entropy < 28)
        let weak_password = Password {
            value: "abc".to_string(), // 3 chars, lowercase only: ~14.1 entropy
        };
        assert_eq!(
            weak_password.classify(&Alphabet::LowerCase).unwrap(),
            Classification::Weak
        );

        // Test Medium classification (28 <= entropy < 40)
        let medium_password = Password {
            value: "abcdef".to_string(), // 6 chars, lowercase only: ~28.2 entropy
        };
        assert_eq!(
            medium_password.classify(&Alphabet::LowerCase).unwrap(),
            Classification::Medium
        );

        // Test Strong classification (40 <= entropy < 60)
        let strong_password = Password {
            value: "password".to_string(), // 8 chars, lowercase only: ~37.6 entropy
        };
        assert_eq!(
            strong_password.classify(&Alphabet::LowerCase).unwrap(),
            Classification::Medium
        );

        let strong_password2 = Password {
            value: "Password".to_string(), // 8 chars, mixed case: ~45.6 entropy
        };
        assert_eq!(
            strong_password2.classify(&Alphabet::Full).unwrap(),
            Classification::Strong
        );

        // Test VeryStrong classification (entropy >= 60)
        let very_strong_password = Password {
            value: "Password123!".to_string(), // 12 chars, full alphabet: ~79.6 entropy
        };
        assert_eq!(
            very_strong_password.classify(&Alphabet::Full).unwrap(),
            Classification::VeryStrong
        );

        // Test empty password
        let empty_password = Password {
            value: "".to_string(),
        };
        assert_eq!(
            empty_password.classify(&Alphabet::Full).unwrap(),
            Classification::Weak
        );

        // Test digits only
        let digits_password = Password {
            value: "12345678".to_string(), // 8 chars, digits only: ~26.6 entropy
        };
        assert_eq!(
            digits_password.classify(&Alphabet::Digits).unwrap(),
            Classification::Weak
        );

        // Test special characters
        let special_password = Password {
            value: "!@#$".to_string(), // 4 chars, special chars: ~12 entropy
        };
        assert_eq!(
            special_password.classify(&Alphabet::SpecialChars).unwrap(),
            Classification::Weak
        );
    }

    #[test]
    fn test_classify_invalid_characters() {
        use crate::passgen::alphabet::Alphabet;

        // Test error case - password contains characters not in alphabet
        let invalid_password = Password {
            value: "Password123!".to_string(),
        };
        assert!(invalid_password.classify(&Alphabet::LowerCase).is_err());

        // Test uppercase characters in lowercase-only alphabet
        let invalid_uppercase = Password {
            value: "Password".to_string(),
        };
        assert!(invalid_uppercase.classify(&Alphabet::LowerCase).is_err());

        // Test lowercase characters in uppercase-only alphabet
        let invalid_lowercase = Password {
            value: "password".to_string(),
        };
        assert!(invalid_lowercase.classify(&Alphabet::UpperCase).is_err());

        // Test letters in digits-only alphabet
        let invalid_letters_in_digits = Password {
            value: "123abc".to_string(),
        };
        assert!(
            invalid_letters_in_digits
                .classify(&Alphabet::Digits)
                .is_err()
        );

        // Test digits in special-chars-only alphabet
        let invalid_digits_in_special = Password {
            value: "!@#123".to_string(),
        };
        assert!(
            invalid_digits_in_special
                .classify(&Alphabet::SpecialChars)
                .is_err()
        );

        // Test special characters not in the defined special chars set
        let invalid_special = Password {
            value: "password~`".to_string(), // ~ and ` are not in SPECIAL_CHARS
        };
        assert!(invalid_special.classify(&Alphabet::SpecialChars).is_err());

        // Test unicode characters
        let invalid_unicode = Password {
            value: "café".to_string(), // é is not in any alphabet
        };
        assert!(invalid_unicode.classify(&Alphabet::Full).is_err());

        // Test space character
        let invalid_space = Password {
            value: "pass word".to_string(), // space is not in any alphabet
        };
        assert!(invalid_space.classify(&Alphabet::Full).is_err());
    }
}
