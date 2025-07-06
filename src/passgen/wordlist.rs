// Or for lazy loading:
use clap::ValueEnum;
use std::sync::OnceLock;

#[derive(Debug, Clone, ValueEnum)]
pub enum WordList {
    EffLarge,
    EffShort1,
    EffShort2,
    #[clap(skip)]
    Custom(Vec<String>),
}

// Wordlist file contents
const EFF_LARGE_WORDLIST: &str = include_str!("../../resources/wordlist/eff_large_wordlist.txt");
const EFF_SHORT_WORDLIST_1: &str =
    include_str!("../../resources/wordlist/eff_short_wordlist_1.txt");
const EFF_SHORT_WORDLIST_2_0: &str =
    include_str!("../../resources/wordlist/eff_short_wordlist_2_0.txt");

// Static caches for lazy loading
static EFF_LARGE_CACHE: OnceLock<Vec<&'static str>> = OnceLock::new();
static EFF_SHORT1_CACHE: OnceLock<Vec<&'static str>> = OnceLock::new();
static EFF_SHORT2_CACHE: OnceLock<Vec<&'static str>> = OnceLock::new();

fn get_eff_large_wordlist() -> &'static [&'static str] {
    EFF_LARGE_CACHE.get_or_init(|| {
        EFF_LARGE_WORDLIST
            .lines()
            .filter_map(|line| parse_eff_line(line))
            .collect()
    })
}

fn get_eff_short1_wordlist() -> &'static [&'static str] {
    EFF_SHORT1_CACHE.get_or_init(|| {
        EFF_SHORT_WORDLIST_1
            .lines()
            .filter_map(|line| parse_eff_line(line))
            .collect()
    })
}

fn get_eff_short2_wordlist() -> &'static [&'static str] {
    EFF_SHORT2_CACHE.get_or_init(|| {
        EFF_SHORT_WORDLIST_2_0
            .lines()
            .filter_map(|line| parse_eff_line(line))
            .collect()
    })
}

fn parse_eff_line(line: &str) -> Option<&str> {
    // EFF format: "11111\tabacus"
    // Split by tab and take the second part (the word)
    line.split('\t').nth(1)
}

// For EFF_Short1 and EFF_Short2, we'll use subsets of the large list for now
// In a real implementation, you'd include the actual short wordlist files
impl Default for WordList {
    fn default() -> Self {
        WordList::EffLarge
    }
}

impl WordList {
    pub fn from_custom(custom: Vec<String>) -> Self {
        WordList::Custom(custom)
    }

    pub fn words(&self) -> Vec<&str> {
        match self {
            WordList::EffLarge => get_eff_large_wordlist().to_vec(),
            WordList::EffShort1 => get_eff_short1_wordlist().to_vec(),
            WordList::EffShort2 => get_eff_short2_wordlist().to_vec(),
            WordList::Custom(custom) => custom.iter().map(|s| s.as_str()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eff_line_parsing() {
        assert_eq!(parse_eff_line("11111\tabacus"), Some("abacus"));
        assert_eq!(parse_eff_line("11112\tabdomen"), Some("abdomen"));
        assert_eq!(parse_eff_line("invalid line"), None);
        assert_eq!(parse_eff_line(""), None);
    }

    #[test]
    fn test_eff_line_parsing_edge_cases() {
        // Test with multiple tabs
        assert_eq!(parse_eff_line("11111\tabacus\textra"), Some("abacus"));
        // Test with no tab
        assert_eq!(parse_eff_line("11111abacus"), None);
        // Test with only tab
        assert_eq!(parse_eff_line("\t"), Some(""));
        // Test with tab at beginning
        assert_eq!(parse_eff_line("\tabacus"), Some("abacus"));
        // Test with whitespace
        assert_eq!(parse_eff_line("11111\t abacus "), Some(" abacus "));
    }

    #[test]
    fn test_eff_large_wordlist() {
        let words = get_eff_large_wordlist();
        assert!(!words.is_empty());
        assert!(words.contains(&"abacus"));
        assert!(words.contains(&"abdomen"));
    }

    #[test]
    fn test_eff_short_lists_are_different() {
        let large = get_eff_large_wordlist();
        let short1 = get_eff_short1_wordlist();
        let short2 = get_eff_short2_wordlist();

        assert!(short1.len() <= large.len());
        assert!(short2.len() <= large.len());
        // Verify they are different wordlists
        assert_ne!(short1, short2);
    }

    #[test]
    fn test_eff_short1_wordlist() {
        let words = get_eff_short1_wordlist();
        assert!(!words.is_empty());
        // EFF short wordlist 1 should have 1296 words (6^4)
        assert_eq!(words.len(), 1296);
    }

    #[test]
    fn test_eff_short2_wordlist() {
        let words = get_eff_short2_wordlist();
        assert!(!words.is_empty());
        // EFF short wordlist 2.0 should have 1296 words (6^4)
        assert_eq!(words.len(), 1296);
    }

    #[test]
    fn test_wordlist_default() {
        let default_wordlist = WordList::default();
        assert!(matches!(default_wordlist, WordList::EffLarge));
    }

    #[test]
    fn test_custom_wordlist_creation() {
        let custom_words = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ];
        let wordlist = WordList::from_custom(custom_words.clone());

        if let WordList::Custom(words) = wordlist {
            assert_eq!(words, custom_words);
        } else {
            panic!("Expected Custom wordlist");
        }
    }

    #[test]
    fn test_custom_wordlist_empty() {
        let empty_words = vec![];
        let wordlist = WordList::from_custom(empty_words);
        let words = wordlist.words();
        assert!(words.is_empty());
    }

    #[test]
    fn test_custom_wordlist_single_word() {
        let single_word = vec!["hello".to_string()];
        let wordlist = WordList::from_custom(single_word);
        let words = wordlist.words();
        assert_eq!(words.len(), 1);
        assert_eq!(words[0], "hello");
    }

    #[test]
    fn test_custom_wordlist_special_characters() {
        let special_words = vec![
            "hello-world".to_string(),
            "test@email.com".to_string(),
            "unicode_測試".to_string(),
            "emoji🎉".to_string(),
        ];
        let wordlist = WordList::from_custom(special_words.clone());
        let words = wordlist.words();
        assert_eq!(words.len(), 4);
        assert_eq!(words[0], "hello-world");
        assert_eq!(words[1], "test@email.com");
        assert_eq!(words[2], "unicode_測試");
        assert_eq!(words[3], "emoji🎉");
    }

    #[test]
    fn test_wordlist_words_method_all_variants() {
        // Test EffLarge
        let eff_large = WordList::EffLarge;
        let large_words = eff_large.words();
        assert!(!large_words.is_empty());

        // Test EffShort1
        let eff_short1 = WordList::EffShort1;
        let short1_words = eff_short1.words();
        assert_eq!(short1_words.len(), 1296);

        // Test EffShort2
        let eff_short2 = WordList::EffShort2;
        let short2_words = eff_short2.words();
        assert_eq!(short2_words.len(), 1296);

        // Test Custom
        let custom = WordList::from_custom(vec!["test".to_string()]);
        let custom_words = custom.words();
        assert_eq!(custom_words.len(), 1);
        assert_eq!(custom_words[0], "test");
    }

    #[test]
    fn test_wordlist_clone() {
        let original = WordList::EffLarge;
        let cloned = original.clone();
        assert!(matches!(cloned, WordList::EffLarge));

        let custom_original = WordList::from_custom(vec!["test".to_string()]);
        let custom_cloned = custom_original.clone();
        if let (WordList::Custom(orig), WordList::Custom(cloned)) =
            (&custom_original, &custom_cloned)
        {
            assert_eq!(orig, cloned);
        } else {
            panic!("Expected both to be Custom variants");
        }
    }

    #[test]
    fn test_wordlist_debug() {
        let wordlist = WordList::EffLarge;
        let debug_str = format!("{:?}", wordlist);
        assert!(debug_str.contains("EffLarge"));

        let custom = WordList::from_custom(vec!["test".to_string()]);
        let custom_debug = format!("{:?}", custom);
        assert!(custom_debug.contains("Custom"));
    }

    #[test]
    fn test_eff_wordlists_consistency() {
        // Ensure all wordlists return consistent results on multiple calls
        let words1 = get_eff_large_wordlist();
        let words2 = get_eff_large_wordlist();
        assert_eq!(words1.len(), words2.len());
        assert_eq!(words1, words2);

        let short1_words1 = get_eff_short1_wordlist();
        let short1_words2 = get_eff_short1_wordlist();
        assert_eq!(short1_words1, short1_words2);

        let short2_words1 = get_eff_short2_wordlist();
        let short2_words2 = get_eff_short2_wordlist();
        assert_eq!(short2_words1, short2_words2);
    }

    #[test]
    fn test_eff_wordlists_no_empty_words() {
        // Ensure no wordlist contains empty strings
        let large_words = get_eff_large_wordlist();
        assert!(!large_words.iter().any(|word| word.is_empty()));

        let short1_words = get_eff_short1_wordlist();
        assert!(!short1_words.iter().any(|word| word.is_empty()));

        let short2_words = get_eff_short2_wordlist();
        assert!(!short2_words.iter().any(|word| word.is_empty()));
    }

    #[test]
    fn test_eff_wordlists_unique_words() {
        // Ensure all words in each wordlist are unique
        let large_words = get_eff_large_wordlist();
        let mut large_unique: Vec<&str> = large_words.to_vec();
        large_unique.sort();
        large_unique.dedup();
        assert_eq!(large_words.len(), large_unique.len());

        let short1_words = get_eff_short1_wordlist();
        let mut short1_unique: Vec<&str> = short1_words.to_vec();
        short1_unique.sort();
        short1_unique.dedup();
        assert_eq!(short1_words.len(), short1_unique.len());

        let short2_words = get_eff_short2_wordlist();
        let mut short2_unique: Vec<&str> = short2_words.to_vec();
        short2_unique.sort();
        short2_unique.dedup();
        assert_eq!(short2_words.len(), short2_unique.len());
    }
}
