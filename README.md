# rrand

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A versatile, high-performance random generator CLI tool and Rust library inspired by [random.org](https://www.random.org/) and `openssl rand`.

---

## Features

- **Identifiers**:
  - **UUID**: Versions 4 (pure random), 7 (Unix timestamp + random), and 1, with standard, simple (no hyphens), or URN formats.
  - **ULID**: 128-bit lexicographically sortable identifiers (Crockford's Base32).
  - **NanoID**: Compact, URL-friendly unique IDs with customizable alphabets.
- **Encodings**:
  - **Base36**: Compact alphanumeric strings (`[0-9a-z]` or `[0-9A-Z]`).
  - **Base62**: Alphanumeric `[0-9a-zA-Z]`.
  - **Base58**: Bitcoin/IPFS alphabet (avoids `0, O, I, l`).
- **Cryptographic Bytes (`openssl rand` style)**:
  - Secure random bytes formatted as Hex, Base64, Base64URL, or raw binary.
- **Strings & Passwords**:
  - Alphanumeric, custom charsets, symbols, exclusion of ambiguous characters.
  - **Passphrases**: Diceware-style multi-word passphrases from a curated English wordlist.
- **Numbers & Permutations (`random.org` style)**:
  - **Integers**: Custom ranges `[min, max]`, sampling with or without replacement (`--unique`), and radices (binary, octal, decimal, hex).
  - **Floating-point**: Uniform `[min, max)` and Gaussian / Normal distributions (`--mean`, `--std-dev`).
  - **Sequences**: Full range permutations / shuffles.
- **Games & Sampling**:
  - **Coin flips**: Heads or Tails.
  - **Dice rolling**: Standard notation (`2d6`, `d20`, `100`, etc.).
  - **Pick**: Sample items randomly from command-line arguments or standard input lines (`stdin`).
- **Text Generation**:
  - **Lorem Ipsum**: Latin placeholder text by words, sentences, or paragraphs.
- **Clean Inline Output**:
  - Automatically appends a newline when running interactively in a terminal.
  - Automatically omits trailing newlines when piped or used in command substitution (`VAR=$(rrand uuid)`).
  - Explicit override flags (`-n` / `--no-newline`, `-N` / `--newline`).

---

## Installation

### From Source
```bash
cargo install --path .
```

### Build Binary
```bash
cargo build --release
# Binary available at target/release/rrand
```

---

## CLI Usage

### Quick Default
Without any arguments, `rrand` produces a 32-character random alphanumeric string:
```bash
rrand
# => 8kQ3Lm9vW2ZtY1Xp7R6b...
```

### 1. Identifiers

```bash
# UUID (default v4)
rrand uuid
# => 4627d351-40e9-4e78-bc57-08e8cf193563

# UUID v7 (time-ordered, modern database friendly)
rrand uuid --v7

# UUID simple (no hyphens) & uppercase
rrand uuid --simple --upper
# => 4627D35140E94E78BC5708E8CF193563

# ULID
rrand ulid
# => 01M4CZ5P6EKH73QRF68KYWVGH8
rrand ulid --lower

# NanoID
rrand nanoid
rrand nanoid -l 12 --alphabet "0123456789ABCDEF"
```

### 2. Encodings & Tokens

```bash
# Base36 (uppercase or lowercase)
rrand base36 -l 16 --upper
# => 7Z3K9X1M8QP0W4RT

# Base62 & Base58
rrand base62 -l 24
rrand base58 -l 32
```

### 3. Cryptographic Bytes (`openssl rand` style)

```bash
# 32 random bytes as hex (64 hex characters)
rrand bytes 32 --hex

# 32 random bytes as Base64 / Base64URL
rrand bytes 32 --base64
rrand bytes 32 --base64url

# Raw binary bytes (write to file or pipe)
rrand bytes 16 --raw > secret.bin
```

### 4. Strings & Passwords

```bash
# Password with symbols
rrand string -l 16 --symbols

# Avoid ambiguous characters (0, O, o, 1, l, I)
rrand string -l 20 --no-ambiguous

# Custom character set
rrand string -l 8 --custom "ACGT"

# Diceware passphrase
rrand passphrase -w 4
# => dense-ivory-voice-clerk

rrand passphrase -w 5 -s "." --capitalize
# => Apple.Bridge.Falcon.Silver.Winter
```

### 5. Numbers & Sequences

```bash
# Random integer between 1 and 100
rrand int --min 1 --max 100

# 6 unique integers for lottery (1-49)
rrand int --min 1 --max 49 -c 6 --unique

# Hexadecimal random integers
rrand int --min 0 --max 255 --radix 16 -c 4

# Floating point numbers
rrand float --min 0.0 --max 1.0 -c 3

# Gaussian / Normal distribution
rrand float --gaussian --mean 0.0 --std-dev 1.0 -p 4

# Random sequence / permutation
rrand sequence --min 1 --max 10
```

### 6. Sampling, Games & Utilities

```bash
# Flip a coin
rrand coin -c 3
# => Heads Tails Heads

# Roll dice
rrand dice 2d6
rrand dice d20

# Pick from arguments
rrand pick apple banana cherry date

# Pick from standard input (stdin)
cat users.txt | rrand pick -c 2 --unique
```

### 7. Lorem Ipsum Placeholder Text

```bash
# Generate 20 Latin words
rrand lorem -w 20

# Generate 3 sentences
rrand lorem -s 3

# Generate 2 paragraphs
rrand lorem -p 2
```

---

## Inline vs. Terminal Output

- **When printing to an interactive terminal:** `rrand` includes a trailing newline `\n` for readability.
- **When called inline, piped, or in subshells:** `rrand` automatically omits the trailing newline:
  ```bash
  # No trailing newline in variable substitution
  TOKEN=$(rrand base36 -l 16)
  echo "Length is exactly ${#TOKEN}" # 16

  # Piping directly to clipboard or tools
  rrand bytes 16 --hex | xclip -selection clipboard
  ```
- **Explicit flags:**
  - `-n`, `--no-newline`: Force no trailing newline.
  - `-N`, `--newline`: Force trailing newline even when piped.

---

## Library Usage (`librrand`)

Add `rrand` to your `Cargo.toml`:

```toml
[dependencies]
rrand = "0.1"
```

### Example

```rust
use rrand::{
    generate_uuid, UuidVersion, UuidFormat,
    generate_ulid,
    generate_base36,
    generate_string, StringConfig,
    generate_passphrase,
    generate_ints,
    generate_floats,
    generate_bytes, ByteFormat, format_bytes,
    pick,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // UUID & ULID
    let u = generate_uuid(UuidVersion::V7, UuidFormat::Standard, false);
    let ulid = generate_ulid(true);

    // Cryptographic bytes
    let raw = generate_bytes(16);
    let hex_str = format_bytes(&raw, ByteFormat::Hex);

    // Custom strings
    let config = StringConfig {
        length: 16,
        symbols: true,
        no_ambiguous: true,
        ..Default::default()
    };
    let password = generate_string(&config)?;

    // Passphrase
    let phrase = generate_passphrase(4, "-", true);

    // Unique random numbers
    let lottery = generate_ints(1, 49, 6, true)?;

    // Pick from slice
    let items = vec!["alpha", "beta", "gamma"];
    let chosen = pick(&items, 1, false)?;

    println!("UUID: {}", u);
    println!("ULID: {}", ulid);
    println!("Hex: {}", hex_str);
    println!("Password: {}", password);
    println!("Phrase: {}", phrase);
    println!("Lottery: {:?}", lottery);
    println!("Picked: {:?}", chosen);

    Ok(())
}
```

---

## License

This project is licensed under the [MIT License](LICENSE).
