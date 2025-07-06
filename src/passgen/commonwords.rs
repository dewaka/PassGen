use std::collections::HashSet;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub enum CommonWords {
    Passwords,
    English,
    MaleNames,
    FemaleNames,
    LastNames,
    All,
    Custom(Vec<String>),
}

const COMMON_ENGLISH: &str = include_str!("../../resources/common/english.txt");
const COMMON_PASSWORDS: &str = include_str!("../../resources/common/passwords.txt");
const COMMON_MALE_NAMES: &str = include_str!("../../resources/common/male_names.txt");

const COMMON_FEMALE_NAMES: &str = include_str!("../../resources/common/female_names.txt");
const COMMON_LAST_NAMES: &str = include_str!("../../resources/common/last_names.txt");

// Static caches for lazy loading
static COMMON_ENGLISH_CACHE: OnceLock<Vec<&'static str>> = OnceLock::new();
static COMMON_PASSWORDS_CACHE: OnceLock<Vec<&'static str>> = OnceLock::new();
static COMMON_MALE_NAMES_CACHE: OnceLock<Vec<&'static str>> = OnceLock::new();
static COMMON_FEMALE_NAMES_CACHE: OnceLock<Vec<&'static str>> = OnceLock::new();
static COMMON_LAST_NAMES_CACHE: OnceLock<Vec<&'static str>> = OnceLock::new();
static COMMON_ALL_CACHE: OnceLock<Vec<&'static str>> = OnceLock::new();

fn get_common_english() -> &'static [&'static str] {
    COMMON_ENGLISH_CACHE.get_or_init(|| COMMON_ENGLISH.lines().collect())
}

fn get_common_passwords() -> &'static [&'static str] {
    COMMON_PASSWORDS_CACHE.get_or_init(|| COMMON_PASSWORDS.lines().collect())
}

fn get_common_male_names() -> &'static [&'static str] {
    COMMON_MALE_NAMES_CACHE.get_or_init(|| COMMON_MALE_NAMES.lines().collect())
}

fn get_common_female_names() -> &'static [&'static str] {
    COMMON_FEMALE_NAMES_CACHE.get_or_init(|| COMMON_FEMALE_NAMES.lines().collect())
}

fn get_common_last_names() -> &'static [&'static str] {
    COMMON_LAST_NAMES_CACHE.get_or_init(|| COMMON_LAST_NAMES.lines().collect())
}

fn get_common_all() -> &'static [&'static str] {
    COMMON_ALL_CACHE.get_or_init(|| {
        let mut all_words = HashSet::new();
        all_words.extend(get_common_passwords().iter());
        all_words.extend(get_common_english().iter());
        all_words.extend(get_common_male_names().iter());
        all_words.extend(get_common_female_names().iter());
        all_words.extend(get_common_last_names().iter());
        all_words.into_iter().collect()
    })
}

impl Default for CommonWords {
    fn default() -> Self {
        CommonWords::All
    }
}

impl CommonWords {
    pub fn words(&self) -> Vec<&str> {
        match self {
            CommonWords::Passwords => get_common_passwords().to_vec(),
            CommonWords::English => get_common_english().to_vec(),
            CommonWords::MaleNames => get_common_male_names().to_vec(),
            CommonWords::FemaleNames => get_common_female_names().to_vec(),
            CommonWords::LastNames => get_common_last_names().to_vec(),
            CommonWords::All => get_common_all().to_vec(),
            CommonWords::Custom(custom) => custom.iter().map(|s| s.as_str()).collect(),
        }
    }
}
