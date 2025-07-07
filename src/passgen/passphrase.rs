// Or for lazy loading:

use crate::passgen::Password;
use crate::passgen::wordlist::WordList;
use rand::Rng;
use std::borrow::Cow;

pub fn generate_passphrase(
    word_count: usize,
    separator: &str,
    wordlist: &WordList,
) -> Password<'static> {
    let words = wordlist.words();
    if words.is_empty() || word_count == 0 {
        return Password {
            value: Cow::Borrowed(""),
        };
    }

    let mut rng = rand::rng();
    let passphrase_parts: Vec<&str> = (0..word_count)
        .map(|_| {
            let idx = rng.random_range(0..words.len());
            words[idx]
        })
        .collect();

    Password {
        value: Cow::Owned(passphrase_parts.join(separator)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::passgen::wordlist::WordList;

    #[test]
    fn test_generate_passphrase_basic() {
        let custom_words = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ];
        let wordlist = WordList::from_custom(custom_words);

        let passphrase = generate_passphrase(3, "-", &wordlist);

        assert!(!passphrase.value.is_empty());
        assert_eq!(passphrase.value.matches('-').count(), 2); // 3 words = 2 separators

        // Check that all parts are valid words
        let parts: Vec<&str> = passphrase.value.split('-').collect();
        assert_eq!(parts.len(), 3);
        for part in parts {
            assert!(["apple", "banana", "cherry"].contains(&part));
        }
    }

    #[test]
    fn test_generate_passphrase_custom_separator() {
        let custom_words = vec!["word1".to_string(), "word2".to_string()];
        let wordlist = WordList::from_custom(custom_words);

        let passphrase = generate_passphrase(2, "_", &wordlist);

        assert!(passphrase.value.contains('_'));
        assert!(!passphrase.value.contains('-'));
    }

    #[test]
    fn test_generate_passphrase_single_word() {
        let custom_words = vec!["single".to_string()];
        let wordlist = WordList::from_custom(custom_words);

        let passphrase = generate_passphrase(1, "-", &wordlist);

        assert_eq!(passphrase.value.as_ref(), "single");
        assert!(!passphrase.value.contains('-'));
    }

    #[test]
    fn test_generate_passphrase_empty_wordlist() {
        let empty_words = vec![];
        let wordlist = WordList::from_custom(empty_words);

        let passphrase = generate_passphrase(3, "-", &wordlist);

        assert!(passphrase.value.is_empty());
    }

    #[test]
    fn test_generate_passphrase_zero_words() {
        let custom_words = vec!["test".to_string()];
        let wordlist = WordList::from_custom(custom_words);

        let passphrase = generate_passphrase(0, "-", &wordlist);

        assert!(passphrase.value.is_empty());
    }

    #[test]
    fn test_generate_passphrase_randomness() {
        let custom_words = vec![
            "word1".to_string(),
            "word2".to_string(),
            "word3".to_string(),
            "word4".to_string(),
            "word5".to_string(),
            "word6".to_string(),
        ];
        let wordlist = WordList::from_custom(custom_words);

        // Generate multiple passphrases and check they're not all identical
        let passphrases: Vec<String> = (0..10)
            .map(|_| generate_passphrase(3, "-", &wordlist).value.into_owned())
            .collect();

        // With 6 words choosing 3, we should get some variation
        let unique_passphrases: std::collections::HashSet<_> = passphrases.into_iter().collect();
        assert!(
            unique_passphrases.len() > 1,
            "Generated passphrases should show randomness"
        );
    }
}
