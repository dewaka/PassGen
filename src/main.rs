mod passgen;

use crate::passgen::alphabet::Alphabet;
use crate::passgen::commonwords::CommonWords;
use crate::passgen::password::Password;
use crate::passgen::wordlist::WordList;
use crate::passgen::{commonwords, passphrase};
use clap::{Parser, Subcommand};
use log::debug;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// debug message verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a random password
    Password {
        /// Length of the generated password
        #[arg(short, long, default_value_t = 12)]
        length: usize,

        /// Alphabet to use for password generation
        #[arg(short, long)]
        alphabet: Option<Alphabet>,

        /// Custom alphabet to use for password generation
        #[arg(short = 'C', long = "custom")]
        custom: Option<String>,

        /// Print strength of the generated password
        #[arg(short, long, default_value_t = false)]
        strength: bool,

        /// Number of passwords to generate
        #[arg(short, long, default_value_t = 1)]
        count: usize,
    },

    /// Generate a passphrase from a word list
    Passphrase {
        /// Length of the generated password
        #[arg(short, long, default_value_t = 3)]
        length: usize,

        /// Word list to use for password generation
        #[arg(short, long)]
        wordlist: Option<WordList>,

        /// Custom words to use for passphrase generation (can be specified multiple times)
        #[arg(short = 'C', long = "custom", num_args = 1..)]
        custom: Option<Vec<String>>,

        /// Custom separator for the passphrase
        #[arg(short, long, default_value = "-")]
        separator: String,

        /// Number of passwords to generate
        #[arg(short, long, default_value_t = 1)]
        count: usize,
    },

    /// Check password strength
    Check {
        /// Password to check for strength
        password: String,

        /// Custom alphabet to use for password strength calculation
        #[arg(short = 'C', long = "custom")]
        custom: Option<String>,

        // Alphabet to use for password strength calculation
        #[arg(short, long)]
        alphabet: Option<Alphabet>,

        // Check safety against common words
        #[arg(short, long, default_value_t = true)]
        common: bool,

        /// Word list to check for common word combinations
        #[arg(short, long, num_args = 1..)]
        wordlist: Option<Vec<String>>,
    },
}

fn generate_password(length: usize, alphabet: &Alphabet, strength: bool) {
    let password = Password::generate(length, alphabet);
    if strength {
        let classification = password.classify(alphabet);
        println!("{} [{:?}]", password.value, classification.unwrap());
    } else {
        println!("{}", password.value);
    }
}

fn check_password_safety(password: &Password) -> Option<String> {
    const SAFETY_CHECKS: &[(CommonWords, &str)] = &[
        (CommonWords::Passwords, "common password"),
        (CommonWords::English, "common English word"),
        (CommonWords::MaleNames, "common male name"),
        (CommonWords::FemaleNames, "common female name"),
        (CommonWords::LastNames, "common last name"),
        (CommonWords::All, "combination of common words"),
    ];

    for (word_type, description) in SAFETY_CHECKS {
        if !password.is_safe(word_type) {
            return Some(format!(
                "{} is not safe because it is a {}",
                password.value, description
            ));
        }
    }
    None
}

fn get_alphabet_from_args(alphabet: Option<Alphabet>, custom: Option<String>) -> Alphabet {
    if let Some(custom_alphabet) = custom {
        Alphabet::Custom(custom_alphabet)
    } else {
        alphabet.unwrap_or_default()
    }
}

fn validate_alphabet_args(
    alphabet: &Option<Alphabet>,
    custom: &Option<String>,
) -> Result<(), &'static str> {
    if alphabet.is_some() && custom.is_some() {
        Err("Cannot specify both alphabet and custom alphabet.")
    } else {
        Ok(())
    }
}

fn main() {
    debug!("starting run_bcl");
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Password {
            alphabet,
            custom,
            length,
            strength,
            count,
        }) => {
            if let Err(e) = validate_alphabet_args(&alphabet, &custom) {
                eprintln!("Error: {}", e);
                return;
            }

            let alphabet = get_alphabet_from_args(alphabet, custom);

            debug!(
                "Generating {} passwords with length: {}, alphabet: {:?}",
                count, length, &alphabet
            );

            for _ in 0..count {
                generate_password(length, &alphabet, strength);
            }
        }

        Some(Commands::Passphrase {
            length,
            wordlist,
            custom,
            separator,
            count,
        }) => {
            debug!(
                "Generating {} passphrases with length: {}, separator: {}",
                count, length, separator
            );

            let wordlist = if let Some(wl) = wordlist {
                wl
            } else if let Some(custom_words) = custom {
                WordList::from_custom(custom_words)
            } else {
                WordList::default()
            };

            for _ in 0..count {
                let passphrase = passphrase::generate_passphrase(length, &separator, &wordlist);
                println!("{}", passphrase.value);
            }
        }

        Some(Commands::Check {
            password,
            alphabet,
            custom,
            common,
            wordlist,
        }) => {
            debug!("Checking password");

            let alphabet = get_alphabet_from_args(alphabet, custom);
            let password_obj = Password::new(&password);

            if common {
                if let Some(wl) = wordlist {
                    let common_words = commonwords::CommonWords::Custom(wl);
                    if !password_obj.is_safe(&common_words) {
                        println!(
                            "{} is not safe because it contains common words from the provided list",
                            password_obj.value
                        );
                        return;
                    }
                } else if let Some(safety_message) = check_password_safety(&password_obj) {
                    println!("{}", safety_message);
                    return;
                }
            }

            match password_obj.classify(&alphabet) {
                Ok(classification) => {
                    println!("{} -> {:?}", password_obj.value, classification);
                }
                Err(e) => {
                    eprintln!("Error classifying password: {}", e);
                }
            }
        }
        None => {
            eprintln!("No command provided. Use --help for more information.");
        }
    }
}
