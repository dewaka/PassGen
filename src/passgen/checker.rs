use crate::passgen::alphabet::Alphabet;
use crate::passgen::commonwords::CommonWords;
use crate::passgen::password::Password;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Classification {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl<'a> Password<'a> {
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

    // Assumes words are lowercase and checks if the password can be formed by concatenating words from the provided list
    fn is_combination_of_word_set(&self, word_set: &HashSet<&str>) -> bool {
        let password = self.value.to_lowercase();

        let mut dp = vec![false; password.len() + 1];
        dp[0] = true; // Empty string can always be formed
        for i in 1..=password.len() {
            for j in 0..i {
                if dp[j] && word_set.contains(&password[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[password.len()]
    }

    #[allow(dead_code)]
    fn is_combination_of_words(&self, words: &[&str]) -> bool {
        let word_set = words.iter().cloned().collect::<HashSet<_>>();
        self.is_combination_of_word_set(&word_set)
    }

    pub fn is_safe(&self, common_words: &CommonWords) -> bool {
        // If the password is empty, it's considered not safe
        if self.value.is_empty() {
            return false;
        }

        let word_set = common_words.words().iter().cloned().collect::<HashSet<_>>();
        let lowercase_password = self.value.to_lowercase();

        // Check if the password is a common word
        if word_set.contains(lowercase_password.as_str()) {
            return false;
        }

        // Check if the password is a combination of common words
        if self.is_combination_of_word_set(&word_set) {
            return false;
        }

        // Check if the password contains any of the common words as substrings.
        // This is a simple check and might not be what is desired for all cases.
        // For example, "mypassword" would be unsafe if "password" is a common word.
        // The current logic in `is_combination_of_word_set` already handles substrings
        // that form the whole password. This check is for partial containment.
        // A more robust implementation might be needed depending on desired behavior.
        // For now, the combination check is the primary logic.

        true // If no checks failed, the password is safe
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy() {
        let password = Password::new("password");
        // rewrite above with an epsilon comparison
        assert!((password.entropy(26) - 37.6).abs() < 0.01);
    }

    #[test]
    fn test_classify() {
        use crate::passgen::alphabet::Alphabet;

        // Test Weak classification (entropy < 28)
        let weak_password = Password::new("abc"); // 3 chars, lowercase only: ~14.1 entropy
        assert_eq!(
            weak_password.classify(&Alphabet::LowerCase).unwrap(),
            Classification::Weak
        );

        // Test Medium classification (28 <= entropy < 40)
        let medium_password = Password::new("abcdef"); // 6 chars, lowercase only: ~28.2 entropy
        assert_eq!(
            medium_password.classify(&Alphabet::LowerCase).unwrap(),
            Classification::Medium
        );

        // Test Strong classification (40 <= entropy < 60)
        let strong_password = Password::new("password"); // 8 chars, lowercase only: ~37.6 entropy
        assert_eq!(
            strong_password.classify(&Alphabet::LowerCase).unwrap(),
            Classification::Medium
        );

        let strong_password2 = Password::new("Password"); // 8 chars, mixed case: ~45.6 entropy
        assert_eq!(
            strong_password2.classify(&Alphabet::Full).unwrap(),
            Classification::Strong
        );

        // Test VeryStrong classification (entropy >= 60)
        let very_strong_password = Password::new("Password123!"); // 12 chars, full alphabet: ~79.6 entropy
        assert_eq!(
            very_strong_password.classify(&Alphabet::Full).unwrap(),
            Classification::VeryStrong
        );

        // Test empty password
        let empty_password = Password::new("");
        assert_eq!(
            empty_password.classify(&Alphabet::Full).unwrap(),
            Classification::Weak
        );

        // Test digits only
        let digits_password = Password::new("12345678"); // 8 chars, digits only: ~26.6 entropy
        assert_eq!(
            digits_password.classify(&Alphabet::Digits).unwrap(),
            Classification::Weak
        );

        // Test special characters
        let special_password = Password::new("!@#$"); // 4 chars, special chars: ~12 entropy
        assert_eq!(
            special_password.classify(&Alphabet::SpecialChars).unwrap(),
            Classification::Weak
        );

        // Test Custom alphabet - binary
        let custom_binary = Alphabet::Custom("01".to_string());
        let binary_password = Password::new("1010110011"); // 10 chars, binary: ~10 entropy
        assert_eq!(
            binary_password.classify(&custom_binary).unwrap(),
            Classification::Weak
        );

        // Test Custom alphabet - hex
        let custom_hex = Alphabet::Custom("0123456789abcdef".to_string());
        let hex_password = Password::new("deadbeef"); // 8 chars, hex: ~32 entropy
        assert_eq!(
            hex_password.classify(&custom_hex).unwrap(),
            Classification::Medium
        );

        // Test Custom alphabet - strong
        let custom_large = Alphabet::Custom(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()+-="
                .to_string(),
        );
        let custom_strong_password = Password::new("CustomP@ss123"); // 13 chars, large custom alphabet
        assert_eq!(
            custom_strong_password.classify(&custom_large).unwrap(),
            Classification::VeryStrong
        );
    }

    #[test]
    fn test_classify_invalid_characters() {
        use crate::passgen::alphabet::Alphabet;

        // Test error case - password contains characters not in alphabet
        let invalid_password = Password::new("Password123!");
        assert!(invalid_password.classify(&Alphabet::LowerCase).is_err());

        // Test uppercase characters in lowercase-only alphabet
        let invalid_uppercase = Password::new("Password");
        assert!(invalid_uppercase.classify(&Alphabet::LowerCase).is_err());

        // Test lowercase characters in uppercase-only alphabet
        let invalid_lowercase = Password::new("password");
        assert!(invalid_lowercase.classify(&Alphabet::UpperCase).is_err());

        // Test letters in digits-only alphabet
        let invalid_letters_in_digits = Password::new("123abc");
        assert!(
            invalid_letters_in_digits
                .classify(&Alphabet::Digits)
                .is_err()
        );

        // Test digits in special-chars-only alphabet
        let invalid_digits_in_special = Password::new("!@#123");
        assert!(
            invalid_digits_in_special
                .classify(&Alphabet::SpecialChars)
                .is_err()
        );

        // Test special characters not in the defined special chars set
        let invalid_special = Password::new("password~`"); // ~ and ` are not in SPECIAL_CHARS
        assert!(invalid_special.classify(&Alphabet::SpecialChars).is_err());

        // Test unicode characters
        let invalid_unicode = Password::new("café"); // é is not in any alphabet
        assert!(invalid_unicode.classify(&Alphabet::Full).is_err());

        // Test space character
        let invalid_space = Password::new("pass word"); // space is not in any alphabet
        assert!(invalid_space.classify(&Alphabet::Full).is_err());

        // Test Custom alphabet - invalid characters
        let custom_vowels = Alphabet::Custom("aeiou".to_string());
        let invalid_custom = Password::new("hello"); // contains 'h', 'l' which are not vowels
        assert!(invalid_custom.classify(&custom_vowels).is_err());

        // Test Custom alphabet - valid characters
        let valid_custom = Password::new("aeio"); // only vowels
        assert!(valid_custom.classify(&custom_vowels).is_ok());

        // Test Custom alphabet - empty alphabet
        let empty_custom = Alphabet::Custom("".to_string());
        let any_password = Password::new("a");
        assert!(any_password.classify(&empty_custom).is_err());
    }

    #[test]
    fn test_is_combination_of() {
        let password = Password::new("applebanana");
        let words = vec!["apple", "banana", "cherry"];
        assert!(password.is_combination_of_words(&words));

        let password2 = Password::new("applecherry");
        assert!(password2.is_combination_of_words(&words));

        let password3 = Password::new("appleorange");
        assert!(!password3.is_combination_of_words(&words));

        let password4 = Password::new("banana");
        assert!(password4.is_combination_of_words(&words));

        let password5 = Password::new("grape");
        assert!(!password5.is_combination_of_words(&words));
        let password6 = Password::new("apple");
        assert!(password6.is_combination_of_words(&words));
        let password7 = Password::new("applebananaorange");
        assert!(!password7.is_combination_of_words(&words));
        let password8 = Password::new("applebananaapple");
        assert!(password8.is_combination_of_words(&words));
        let password9 = Password::new("applebananaapplecherry");
        assert!(password9.is_combination_of_words(&words));
        let password10 = Password::new("APPLEBANANAAPPLECHERRYGRAPE");
        assert!(!password10.is_combination_of_words(&words));
        let password11 = Password::new("APPLEbanana");
        assert!(password11.is_combination_of_words(&words));
    }

    #[test]
    fn test_is_safe() {
        let words = vec!["mary".to_string(), "lisa".to_string()];
        let password12 = Password::new("marylisa");
        assert!(!password12.is_safe(&CommonWords::Custom(words)));
    }

    #[test]
    fn test_is_safe_comprehensive() {
        let common_words = vec![
            "password".to_string(),
            "admin".to_string(),
            "user".to_string(),
            "test".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "apple".to_string(),
            "banana".to_string(),
        ];
        let custom_words = CommonWords::Custom(common_words);

        // Test exact match with common word - should be unsafe
        let common_password = Password::new("password");
        assert!(!common_password.is_safe(&custom_words));

        let admin_password = Password::new("admin");
        assert!(!admin_password.is_safe(&custom_words));

        // Test combination of common words - should be unsafe
        let combo_password1 = Password::new("helloworld");
        assert!(!combo_password1.is_safe(&custom_words));

        let combo_password2 = Password::new("applebanana");
        assert!(!combo_password2.is_safe(&custom_words));

        let combo_password3 = Password::new("testuser");
        assert!(!combo_password3.is_safe(&custom_words));

        // Test multiple word combinations
        let combo_password4 = Password::new("helloworldtest");
        assert!(!combo_password4.is_safe(&custom_words));

        let combo_password5 = Password::new("applehellobanana");
        assert!(!combo_password5.is_safe(&custom_words));

        // Test safe passwords - should be safe
        let safe_password1 = Password::new("mySecurePassword123");
        assert!(safe_password1.is_safe(&custom_words));

        let safe_password2 = Password::new("ComplexP@ssw0rd!");
        assert!(safe_password2.is_safe(&custom_words));

        let safe_password3 = Password::new("randomstring");
        assert!(safe_password3.is_safe(&custom_words));

        // Test partial matches that are not exact - should be safe
        let partial_password1 = Password::new("passwords"); // contains "password" but not exact
        assert!(partial_password1.is_safe(&custom_words));

        let partial_password2 = Password::new("mypassword"); // contains "password" but has prefix
        assert!(partial_password2.is_safe(&custom_words));

        // Test case sensitivity
        let case_password1 = Password::new("PASSWORD"); // uppercase version of common word
        assert!(!case_password1.is_safe(&custom_words)); // Should be unsafe due to case-insensitive check

        let case_password2 = Password::new("HelloWorld"); // mixed case combination
        assert!(!case_password2.is_safe(&custom_words));

        // Test empty password
        let empty_password = Password::new("");
        assert!(!empty_password.is_safe(&custom_words));

        // Test single character passwords
        let single_char = Password::new("a");
        assert!(single_char.is_safe(&custom_words));

        // Test passwords that contain common words but are not combinations
        let contains_but_not_combo = Password::new("mytestpassword"); // contains "test" and "password" but not as clean combination
        assert!(contains_but_not_combo.is_safe(&custom_words));
    }

    #[test]
    fn test_is_safe_edge_cases() {
        let edge_words = vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "x".to_string(),
            "xy".to_string(),
        ];
        let custom_words = CommonWords::Custom(edge_words);

        // Test single character combinations
        let single_combo = Password::new("ax");
        assert!(!single_combo.is_safe(&custom_words));

        // Test overlapping patterns
        let overlap_password = Password::new("abx"); // "ab" + "x" but also contains "a"
        assert!(!overlap_password.is_safe(&custom_words));

        // Test repeated words
        let repeated_password = Password::new("aaaa");
        assert!(!repeated_password.is_safe(&custom_words));

        // Test complex combinations
        let complex_combo = Password::new("abcxy"); // "abc" + "xy"
        assert!(!complex_combo.is_safe(&custom_words));

        // Test safe patterns
        let safe_edge = Password::new("xyz"); // contains "xy" but not as combination with other words
        assert!(safe_edge.is_safe(&custom_words));
    }

    #[test]
    fn test_is_safe_empty_wordlist() {
        let empty_words = vec![];
        let custom_words = CommonWords::Custom(empty_words);

        let any_password = Password::new("anythinggoeshere");
        assert!(any_password.is_safe(&custom_words));
    }

    #[test]
    fn test_is_safe_case_insensitive() {
        let common_words = vec![
            "password".to_string(),
            "admin".to_string(),
            "user".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "apple".to_string(),
            "banana".to_string(),
        ];
        let custom_words = CommonWords::Custom(common_words);

        // Test uppercase versions of common words - should be unsafe
        let uppercase_password = Password::new("PASSWORD");
        assert!(!uppercase_password.is_safe(&custom_words));

        let uppercase_admin = Password::new("ADMIN");
        assert!(!uppercase_admin.is_safe(&custom_words));

        // Test mixed case versions - should be unsafe
        let mixed_case1 = Password::new("Password");
        assert!(!mixed_case1.is_safe(&custom_words));

        let mixed_case2 = Password::new("AdMiN");
        assert!(!mixed_case2.is_safe(&custom_words));

        let mixed_case3 = Password::new("uSeR");
        assert!(!mixed_case3.is_safe(&custom_words));

        // Test case insensitive combinations - should be unsafe
        let mixed_combo1 = Password::new("HelloWorld");
        assert!(!mixed_combo1.is_safe(&custom_words));

        let mixed_combo2 = Password::new("HELLOWORLD");
        assert!(!mixed_combo2.is_safe(&custom_words));

        let mixed_combo3 = Password::new("AppleBanana");
        assert!(!mixed_combo3.is_safe(&custom_words));

        let mixed_combo4 = Password::new("APPLEBANANA");
        assert!(!mixed_combo4.is_safe(&custom_words));

        // Test complex mixed case combinations
        let complex_mixed1 = Password::new("HelloWORLD");
        assert!(!complex_mixed1.is_safe(&custom_words));

        let complex_mixed2 = Password::new("aPpLeBaNaNa");
        assert!(!complex_mixed2.is_safe(&custom_words));

        let complex_mixed3 = Password::new("PassWordAdminUser");
        assert!(!complex_mixed3.is_safe(&custom_words));

        // Test alternating case patterns
        let alternating1 = Password::new("pAsSwOrD");
        assert!(!alternating1.is_safe(&custom_words));

        let alternating2 = Password::new("HeLlOwOrLd");
        assert!(!alternating2.is_safe(&custom_words));

        // Test that truly safe passwords remain safe regardless of case
        let safe_mixed = Password::new("MySecureP@ssw0rd123");
        assert!(safe_mixed.is_safe(&custom_words));

        let safe_upper = Password::new("COMPLEXSECURESTRING");
        assert!(safe_upper.is_safe(&custom_words));

        // Test edge case: single character case variations
        let single_words = vec!["a".to_string(), "i".to_string()];
        let single_custom = CommonWords::Custom(single_words);

        let upper_single = Password::new("A");
        assert!(!upper_single.is_safe(&single_custom));

        let upper_combo = Password::new("AI");
        assert!(!upper_combo.is_safe(&single_custom));
    }
}
