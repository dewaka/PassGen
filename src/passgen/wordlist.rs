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
const EFF_LARGE_WORDLIST: &str = include_str!("../../resources/eff_large_wordlist.txt");
const EFF_SHORT_WORDLIST_1: &str = include_str!("../../resources/eff_short_wordlist_1.txt");
const EFF_SHORT_WORDLIST_2_0: &str = include_str!("../../resources/eff_short_wordlist_2_0.txt");

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
}
