# PassGen

A command-line password and passphrase generator written in Rust with customizable alphabets, multiple wordlists, and password strength analysis.

## Features

- **Password Generation**: Generate secure random passwords with customizable length and character sets
- **Passphrase Generation**: Create memorable passphrases using various wordlists
- **Password Strength Analysis**: Check the strength and entropy of existing passwords
- **Multiple Alphabets**: Support for different character sets including custom alphabets
- **EFF Wordlists**: Built-in support for Electronic Frontier Foundation wordlists
- **Batch Generation**: Generate multiple passwords/passphrases at once

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd passgen

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

### Generate Random Passwords

Generate a single password with default settings (12 characters):
```bash
passgen password
```

Generate passwords with custom length:
```bash
passgen password --length 16
```

Generate multiple passwords:
```bash
passgen password --count 5 --length 20
```

Generate passwords with specific alphabet:
```bash
passgen password --alphabet lowercase
passgen password --alphabet uppercase
passgen password --alphabet alphanumeric
passgen password --alphabet special
```

Generate passwords with custom character set:
```bash
passgen password --custom "abcdef123456!@#"
```

Show password strength:
```bash
passgen password --strength
```

### Generate Passphrases

Generate a passphrase with default settings (3 words, hyphen separator):
```bash
passgen passphrase
```

Generate passphrases with custom length and separator:
```bash
passgen passphrase --length 5 --separator " "
```

Use specific wordlists:
```bash
passgen passphrase --wordlist embedded
passgen passphrase --wordlist eff-large
passgen passphrase --wordlist eff-short1
passgen passphrase --wordlist eff-short2
```

Use custom words:
```bash
passgen passphrase --custom apple banana cherry --length 4
```

Generate multiple passphrases:
```bash
passgen passphrase --count 3 --length 4
```

### Check Password Strength

Check the strength of an existing password:
```bash
passgen check "mypassword123"
```

Check strength against specific alphabet:
```bash
passgen check "MyP@ssw0rd!" --alphabet alphanumeric
```

Check strength with custom alphabet:
```bash
passgen check "abc123" --custom "abcdefghijklmnopqrstuvwxyz0123456789"
```

## Alphabets

The tool supports several predefined alphabets:

- **lowercase**: `abcdefghijklmnopqrstuvwxyz`
- **uppercase**: `ABCDEFGHIJKLMNOPQRSTUVWXYZ`
- **alphanumeric**: Letters and numbers
- **special**: Letters, numbers, and special characters
- **custom**: User-defined character set

## Wordlists

Available wordlists for passphrase generation:

- **embedded**: Built-in wordlist
- **eff-large**: EFF Large Wordlist (7776 words, 5 dice)
- **eff-short1**: EFF Short Wordlist #1 (1296 words, 4 dice)
- **eff-short2**: EFF Short Wordlist #2 (1296 words, 4 dice)
- **custom**: User-provided words

## Examples

### Generate secure passwords for different use cases

```bash
# Simple password for basic accounts
passgen password --length 12 --alphabet alphanumeric

# High-security password with special characters
passgen password --length 16 --alphabet special --strength

# Multiple passwords for bulk account creation
passgen password --count 10 --length 14 --alphabet special
```

### Create memorable passphrases

```bash
# Standard passphrase
passgen passphrase --length 4 --separator "-"

# Passphrase with spaces for better readability
passgen passphrase --length 5 --separator " " --wordlist eff-large

# Custom word passphrase
passgen passphrase --custom red blue green yellow --length 3 --separator "."
```

### Analyze password security

```bash
# Check a weak password
passgen check "password123"

# Check a strong password
passgen check "Tr0ub4dor&3" --alphabet special

# Check against custom requirements
passgen check "abc123XYZ" --custom "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
```

## Security Notes

- All passwords are generated using cryptographically secure random number generation
- Entropy calculations help you understand password strength
- EFF wordlists are designed for diceware-style secure passphrase generation
- Custom alphabets allow you to meet specific password policy requirements

## Debug Mode

Enable debug output to see detailed information about the generation process:

```bash
passgen -d password --length 16
passgen -dd passphrase --length 4  # More verbose
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

### Apache License 2.0

```
Copyright 2024 PassGen Contributors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

## Dependencies

- `clap`: Command-line argument parsing
- `log`: Logging functionality
- `rand`: Cryptographically secure random number generation

## Build Requirements

- Rust 1.70+ (or compatible version)
