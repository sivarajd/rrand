use clap::{Args, Parser, Subcommand};
use rrand::bytes::{format_bytes, generate_bytes, ByteFormat};
use rrand::choice::pick;
use rrand::encoding::{generate_base36, generate_base58, generate_base62};
use rrand::games::{flip_coin, parse_and_roll_dice, roll_dice};
use rrand::id::{generate_nanoid, generate_ulid, generate_uuid, UuidFormat, UuidVersion};
use rrand::lorem::{generate_paragraphs, generate_sentences, generate_words};
use rrand::num::{format_int, generate_floats, generate_gaussian, generate_ints};
use rrand::output::{print_output, print_raw_bytes};
use rrand::passphrase::generate_passphrase;
use rrand::sequence::generate_sequence;
use rrand::string::{generate_string, StringConfig};
use std::io::{self, BufRead};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "rrand",
    author = "Sivaraj D <sivarajd@gmail.com>",
    version = "0.1.0",
    about = "Versatile random number, string, identifier, and byte generator",
    long_about = "A versatile random generator CLI and library inspired by random.org and openssl rand."
)]
struct Cli {
    /// Omit trailing newline in output (always active when called inline/piped unless overridden)
    #[arg(short = 'n', long = "no-newline", global = true)]
    no_newline: bool,

    /// Force trailing newline in output even when piped or called inline
    #[arg(short = 'N', long = "newline", global = true)]
    force_newline: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate cryptographically secure random bytes (openssl rand style)
    Bytes(BytesArgs),

    /// Generate UUID (v4, v7, v1)
    Uuid(UuidArgs),

    /// Generate ULID (Universally Unique Lexicographically Sortable Identifier)
    Ulid(UlidArgs),

    /// Generate NanoID
    Nanoid(NanoidArgs),

    /// Generate Base36 string [0-9a-z] or [0-9A-Z]
    Base36(Base36Args),

    /// Generate Base62 string [0-9a-zA-Z]
    Base62(BaseLengthArgs),

    /// Generate Base58 string (Bitcoin/IPFS alphabet, no 0, O, I, l)
    Base58(BaseLengthArgs),

    /// Generate customizable random string / password
    String(StringArgs),

    /// Generate Diceware-style passphrase of dictionary words
    Passphrase(PassphraseArgs),

    /// Generate random integers (random.org style)
    Int(IntArgs),

    /// Generate random floating-point numbers (uniform or Gaussian/normal)
    Float(FloatArgs),

    /// Generate a randomized sequence of integers
    Sequence(SequenceArgs),

    /// Pick random item(s) from arguments or standard input
    Pick(PickArgs),

    /// Flip a coin (Heads or Tails)
    Coin(CoinArgs),

    /// Roll dice (e.g. d6, 2d6, d20)
    Dice(DiceArgs),

    /// Generate Lorem Ipsum placeholder text
    Lorem(LoremArgs),
}

#[derive(Args, Debug)]
struct BytesArgs {
    /// Number of bytes to generate
    #[arg(default_value_t = 32)]
    count: usize,

    /// Output as hexadecimal string (default)
    #[arg(long, conflicts_with_all = ["base64", "base64url", "raw"])]
    hex: bool,

    /// Output as standard Base64 string
    #[arg(long, conflicts_with_all = ["hex", "base64url", "raw"])]
    base64: bool,

    /// Output as URL-safe Base64 string
    #[arg(long, conflicts_with_all = ["hex", "base64", "raw"])]
    base64url: bool,

    /// Output raw binary bytes
    #[arg(short = 'r', long = "raw", conflicts_with_all = ["hex", "base64", "base64url"])]
    raw: bool,
}

#[derive(Args, Debug)]
struct UuidArgs {
    /// Generate UUID version 4 (random, default)
    #[arg(long, conflicts_with_all = ["v7", "v1"])]
    v4: bool,

    /// Generate UUID version 7 (Unix timestamp + random)
    #[arg(long, conflicts_with_all = ["v4", "v1"])]
    v7: bool,

    /// Generate UUID version 1 (timestamp + node)
    #[arg(long, conflicts_with_all = ["v4", "v7"])]
    v1: bool,

    /// Output without hyphens (32 hex characters)
    #[arg(long)]
    simple: bool,

    /// Output in URN format (urn:uuid:...)
    #[arg(long, conflicts_with = "simple")]
    urn: bool,

    /// Output in uppercase
    #[arg(long)]
    upper: bool,
}

#[derive(Args, Debug)]
struct UlidArgs {
    /// Output in lowercase (default is uppercase)
    #[arg(long)]
    lower: bool,
}

#[derive(Args, Debug)]
struct NanoidArgs {
    /// Length of the NanoID
    #[arg(short = 'l', long = "len", default_value_t = 21)]
    len: usize,

    /// Custom alphabet to sample from
    #[arg(long)]
    alphabet: Option<String>,
}

#[derive(Args, Debug)]
struct Base36Args {
    /// Length of the string
    #[arg(short = 'l', long = "len", default_value_t = 32)]
    len: usize,

    /// Output in uppercase
    #[arg(long)]
    upper: bool,
}

#[derive(Args, Debug)]
struct BaseLengthArgs {
    /// Length of the string
    #[arg(short = 'l', long = "len", default_value_t = 32)]
    len: usize,
}

#[derive(Args, Debug)]
struct StringArgs {
    /// Length of the string
    #[arg(short = 'l', long = "len", default_value_t = 32)]
    len: usize,

    /// Number of strings to generate
    #[arg(short = 'c', long = "count", default_value_t = 1)]
    count: usize,

    /// Include lowercase characters
    #[arg(long)]
    lower: bool,

    /// Include uppercase characters
    #[arg(long)]
    upper: bool,

    /// Include digits (0-9)
    #[arg(short = 'd', long = "digits")]
    digits: bool,

    /// Include special symbols (!@#$%^&*...)
    #[arg(short = 's', long = "symbols")]
    symbols: bool,

    /// Use alphanumeric characters (a-z, A-Z, 0-9)
    #[arg(short = 'a', long = "alphanumeric")]
    alphanumeric: bool,

    /// Exclude ambiguous characters (0, O, o, 1, l, I)
    #[arg(long = "no-ambiguous")]
    no_ambiguous: bool,

    /// Custom character set
    #[arg(long = "custom")]
    custom: Option<String>,

    /// Delimiter between multiple strings
    #[arg(long = "delimiter", default_value = "\n")]
    delimiter: String,
}

#[derive(Args, Debug)]
struct PassphraseArgs {
    /// Number of dictionary words
    #[arg(short = 'w', long = "words", default_value_t = 4)]
    words: usize,

    /// Word separator
    #[arg(short = 's', long = "separator", default_value = "-")]
    separator: String,

    /// Capitalize the first letter of each word
    #[arg(long = "capitalize")]
    capitalize: bool,
}

#[derive(Args, Debug)]
struct IntArgs {
    /// Minimum value (inclusive)
    #[arg(long = "min", default_value_t = 1)]
    min: i64,

    /// Maximum value (inclusive)
    #[arg(long = "max", default_value_t = 100)]
    max: i64,

    /// Number of integers to generate
    #[arg(short = 'c', long = "count", default_value_t = 1)]
    count: usize,

    /// Sample unique integers (without replacement)
    #[arg(short = 'u', long = "unique")]
    unique: bool,

    /// Base / radix for formatting (2, 8, 10, 16)
    #[arg(long = "radix", default_value_t = 10)]
    radix: u32,

    /// Delimiter between multiple integers
    #[arg(short = 'd', long = "delimiter", default_value = " ")]
    delimiter: String,
}

#[derive(Args, Debug)]
struct FloatArgs {
    /// Minimum value
    #[arg(long = "min", default_value_t = 0.0)]
    min: f64,

    /// Maximum value
    #[arg(long = "max", default_value_t = 1.0)]
    max: f64,

    /// Number of floats to generate
    #[arg(short = 'c', long = "count", default_value_t = 1)]
    count: usize,

    /// Decimal precision
    #[arg(short = 'p', long = "precision")]
    precision: Option<usize>,

    /// Sample from Gaussian / normal distribution
    #[arg(long = "gaussian", visible_alias = "normal")]
    gaussian: bool,

    /// Mean for Gaussian distribution
    #[arg(long = "mean", default_value_t = 0.0)]
    mean: f64,

    /// Standard deviation for Gaussian distribution
    #[arg(long = "std-dev", default_value_t = 1.0)]
    std_dev: f64,

    /// Delimiter between multiple numbers
    #[arg(short = 'd', long = "delimiter", default_value = " ")]
    delimiter: String,
}

#[derive(Args, Debug)]
struct SequenceArgs {
    /// Minimum value (inclusive)
    #[arg(long = "min", default_value_t = 1)]
    min: i64,

    /// Maximum value (inclusive)
    #[arg(long = "max", default_value_t = 10)]
    max: i64,

    /// Delimiter between numbers
    #[arg(short = 'd', long = "delimiter", default_value = " ")]
    delimiter: String,
}

#[derive(Args, Debug)]
struct PickArgs {
    /// Candidate items to pick from (reads from stdin if omitted)
    items: Vec<String>,

    /// Number of items to pick
    #[arg(short = 'c', long = "count", default_value_t = 1)]
    count: usize,

    /// Pick unique items (without replacement)
    #[arg(short = 'u', long = "unique")]
    unique: bool,

    /// Delimiter between picked items
    #[arg(short = 'd', long = "delimiter", default_value = " ")]
    delimiter: String,
}

#[derive(Args, Debug)]
struct CoinArgs {
    /// Number of coin flips
    #[arg(short = 'c', long = "count", default_value_t = 1)]
    count: usize,

    /// Delimiter between coin flips
    #[arg(short = 'd', long = "delimiter", default_value = " ")]
    delimiter: String,
}

#[derive(Args, Debug)]
struct DiceArgs {
    /// Dice specification (e.g., 'd6', '2d6', '20')
    spec: Option<String>,

    /// Explicit number of sides on each die
    #[arg(long = "sides")]
    sides: Option<u32>,

    /// Number of dice to roll (when using --sides)
    #[arg(short = 'c', long = "count", default_value_t = 1)]
    count: usize,

    /// Delimiter between dice rolls
    #[arg(short = 'd', long = "delimiter", default_value = " ")]
    delimiter: String,
}

#[derive(Args, Debug)]
struct LoremArgs {
    /// Number of words to generate
    #[arg(short = 'w', long = "words")]
    words: Option<usize>,

    /// Number of sentences to generate
    #[arg(short = 's', long = "sentences")]
    sentences: Option<usize>,

    /// Number of paragraphs to generate (default: 1)
    #[arg(short = 'p', long = "paragraphs")]
    paragraphs: Option<usize>,

    /// Do not start with classic 'Lorem ipsum dolor sit amet...'
    #[arg(long = "random")]
    random: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("rrand error: {}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let no_nl = cli.no_newline;
    let force_nl = cli.force_newline;

    match cli.command {
        None => {
            // Default behavior: 32-character random alphanumeric string
            let config = StringConfig::default();
            let s = generate_string(&config)?;
            print_output(&s, no_nl, force_nl)?;
        }
        Some(Commands::Bytes(args)) => {
            let bytes = generate_bytes(args.count);
            if args.raw {
                print_raw_bytes(&bytes)?;
            } else {
                let fmt = if args.base64 {
                    ByteFormat::Base64
                } else if args.base64url {
                    ByteFormat::Base64Url
                } else {
                    ByteFormat::Hex
                };
                let formatted = format_bytes(&bytes, fmt);
                print_output(&formatted, no_nl, force_nl)?;
            }
        }
        Some(Commands::Uuid(args)) => {
            let version = if args.v7 {
                UuidVersion::V7
            } else if args.v1 {
                UuidVersion::V1
            } else {
                UuidVersion::V4
            };

            let format = if args.simple {
                UuidFormat::Simple
            } else if args.urn {
                UuidFormat::Urn
            } else {
                UuidFormat::Standard
            };

            let uuid_str = generate_uuid(version, format, args.upper);
            print_output(&uuid_str, no_nl, force_nl)?;
        }
        Some(Commands::Ulid(args)) => {
            let ulid_str = generate_ulid(!args.lower);
            print_output(&ulid_str, no_nl, force_nl)?;
        }
        Some(Commands::Nanoid(args)) => {
            let id = generate_nanoid(args.len, args.alphabet.as_deref());
            print_output(&id, no_nl, force_nl)?;
        }
        Some(Commands::Base36(args)) => {
            let s = generate_base36(args.len, args.upper);
            print_output(&s, no_nl, force_nl)?;
        }
        Some(Commands::Base62(args)) => {
            let s = generate_base62(args.len);
            print_output(&s, no_nl, force_nl)?;
        }
        Some(Commands::Base58(args)) => {
            let s = generate_base58(args.len);
            print_output(&s, no_nl, force_nl)?;
        }
        Some(Commands::String(args)) => {
            let has_specific_class = args.lower || args.upper || args.digits || args.symbols;
            let config = StringConfig {
                length: args.len,
                lowercase: if has_specific_class { args.lower } else { true },
                uppercase: if has_specific_class { args.upper } else { true },
                digits: if has_specific_class { args.digits } else { true },
                symbols: args.symbols,
                no_ambiguous: args.no_ambiguous,
                custom_charset: args.custom,
            };

            let mut strings = Vec::with_capacity(args.count);
            for _ in 0..args.count {
                strings.push(generate_string(&config)?);
            }
            let output_str = strings.join(&args.delimiter);
            print_output(&output_str, no_nl, force_nl)?;
        }
        Some(Commands::Passphrase(args)) => {
            let pass = generate_passphrase(args.words, &args.separator, args.capitalize);
            print_output(&pass, no_nl, force_nl)?;
        }
        Some(Commands::Int(args)) => {
            let ints = generate_ints(args.min, args.max, args.count, args.unique)?;
            let formatted_ints: Result<Vec<String>, _> = ints
                .into_iter()
                .map(|val| format_int(val, args.radix))
                .collect();
            let output_str = formatted_ints?.join(&args.delimiter);
            print_output(&output_str, no_nl, force_nl)?;
        }
        Some(Commands::Float(args)) => {
            let floats = if args.gaussian {
                generate_gaussian(args.mean, args.std_dev, args.count)?
            } else {
                generate_floats(args.min, args.max, args.count)?
            };

            let formatted: Vec<String> = floats
                .into_iter()
                .map(|val| {
                    if let Some(p) = args.precision {
                        format!("{:.*}", p, val)
                    } else {
                        val.to_string()
                    }
                })
                .collect();
            let output_str = formatted.join(&args.delimiter);
            print_output(&output_str, no_nl, force_nl)?;
        }
        Some(Commands::Sequence(args)) => {
            let seq = generate_sequence(args.min, args.max)?;
            let seq_str: Vec<String> = seq.into_iter().map(|n| n.to_string()).collect();
            let output_str = seq_str.join(&args.delimiter);
            print_output(&output_str, no_nl, force_nl)?;
        }
        Some(Commands::Pick(args)) => {
            let items = if !args.items.is_empty() {
                args.items
            } else {
                let stdin = io::stdin();
                let lines: Result<Vec<String>, _> = stdin.lock().lines().collect();
                lines?
            };
            let chosen = pick(&items, args.count, args.unique)?;
            let output_str = chosen.join(&args.delimiter);
            print_output(&output_str, no_nl, force_nl)?;
        }
        Some(Commands::Coin(args)) => {
            let flips = flip_coin(args.count);
            let output_str = flips.join(&args.delimiter);
            print_output(&output_str, no_nl, force_nl)?;
        }
        Some(Commands::Dice(args)) => {
            let rolls = if let Some(sides) = args.sides {
                roll_dice(sides, args.count)?
            } else if let Some(spec) = args.spec {
                parse_and_roll_dice(&spec)?
            } else {
                roll_dice(6, args.count)?
            };
            let rolls_str: Vec<String> = rolls.into_iter().map(|r| r.to_string()).collect();
            let output_str = rolls_str.join(&args.delimiter);
            print_output(&output_str, no_nl, force_nl)?;
        }
        Some(Commands::Lorem(args)) => {
            let start_lorem = !args.random;
            let output_str = if let Some(words) = args.words {
                generate_words(words)
            } else if let Some(sentences) = args.sentences {
                generate_sentences(sentences, start_lorem)
            } else {
                let paragraphs = args.paragraphs.unwrap_or(1);
                generate_paragraphs(paragraphs, start_lorem)
            };
            print_output(&output_str, no_nl, force_nl)?;
        }
    }

    Ok(())
}
