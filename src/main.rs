mod passgen;

use crate::passgen::alphabet::Alphabet;
use clap::{Parser, Subcommand};
use log::debug;
use passgen::Password;

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
    Generate {
        /// Length of the generated password
        #[arg(short, long, default_value_t = 12)]
        length: usize,

        /// Alphabet to use for password generation
        #[arg(short, long)]
        alphabet: Option<Alphabet>,

        /// Custom alphabet to use for password generation
        #[arg(short = 'C', long = "custom")]
        custom: Option<String>,

        /// Print strength of the generated password (default: false)
        #[arg(short, long, default_value_t = false)]
        strength: bool,

        /// Number of passwords to generate (default: 1)
        #[arg(short, long, default_value_t = 1)]
        count: usize,
    },

    /// Check password strength
    Check {
        /// Password to check for strength
        #[arg(short, long)]
        password: String,

        /// Custom alphabet to use for password strength calculation
        #[arg(short = 'C', long = "custom")]
        custom: Option<String>,

        // Alphabet to use for password strength calculation
        #[arg(short, long)]
        alphabet: Option<Alphabet>,
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

fn main() {
    debug!("starting run_bcl");
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Generate {
            ref alphabet,
            ref custom,
            length,
            strength,
            count,
        }) => {
            if alphabet.is_some() && custom.is_some() {
                eprintln!("Error: Cannot specify both alphabet and custom alphabet.");
                return;
            }

            let alphabet = get_alphabet_from_args(alphabet, custom);

            debug!(
                "Generating {} passwords with length: {}, alphabet: {:?}",
                count, length, alphabet
            );

            for _ in 0..count {
                generate_password(length, &alphabet, strength);
            }
        }
        Some(Commands::Check {
            password,
            ref alphabet,
            ref custom,
        }) => {
            debug!("Checking password");

            let alphabet = get_alphabet_from_args(alphabet, custom);

            let password_obj = Password { value: password };
            match password_obj.classify(&alphabet) {
                Ok(classification) => {
                    println!("Password: {}", password_obj.value);
                    println!("Strength: {:?}", classification);
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

fn get_alphabet_from_args(alphabet: &Option<Alphabet>, custom: &Option<String>) -> Alphabet {
    let alphabet: Alphabet = if custom.is_none() {
        match alphabet {
            Some(alphabet) => alphabet.clone(),
            None => Alphabet::Full,
        }
    } else {
        Alphabet::Custom(custom.clone().unwrap())
    };
    alphabet
}
