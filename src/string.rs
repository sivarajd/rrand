use crate::error::{Result, RrandError};
use rand::Rng;

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}|;:,.<>?/~`";
const AMBIGUOUS_CHARS: &[char] = &['0', 'O', 'o', '1', 'l', 'I'];

#[derive(Debug, Clone)]
pub struct StringConfig {
    pub length: usize,
    pub lowercase: bool,
    pub uppercase: bool,
    pub digits: bool,
    pub symbols: bool,
    pub no_ambiguous: bool,
    pub custom_charset: Option<String>,
}

impl Default for StringConfig {
    fn default() -> Self {
        Self {
            length: 32,
            lowercase: true,
            uppercase: true,
            digits: true,
            symbols: false,
            no_ambiguous: false,
            custom_charset: None,
        }
    }
}

/// Generates a random string based on the given configuration.
pub fn generate_string(config: &StringConfig) -> Result<String> {
    if config.length == 0 {
        return Ok(String::new());
    }

    let charset: Vec<char> = if let Some(ref custom) = config.custom_charset {
        let chars: Vec<char> = custom.chars().collect();
        if chars.is_empty() {
            return Err(RrandError::EmptyCharset);
        }
        chars
    } else {
        let mut pool = String::new();
        if config.lowercase {
            pool.push_str(LOWERCASE);
        }
        if config.uppercase {
            pool.push_str(UPPERCASE);
        }
        if config.digits {
            pool.push_str(DIGITS);
        }
        if config.symbols {
            pool.push_str(SYMBOLS);
        }

        let mut chars: Vec<char> = pool.chars().collect();

        if config.no_ambiguous {
            chars.retain(|c| !AMBIGUOUS_CHARS.contains(c));
        }

        if chars.is_empty() {
            return Err(RrandError::EmptyCharset);
        }
        chars
    };

    let mut rng = rand::thread_rng();
    let result: String = (0..config.length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx]
        })
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_string() {
        let config = StringConfig::default();
        let s = generate_string(&config).unwrap();
        assert_eq!(s.len(), 32);
        assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_symbols_included() {
        let config = StringConfig {
            length: 100,
            symbols: true,
            ..Default::default()
        };
        let s = generate_string(&config).unwrap();
        assert_eq!(s.len(), 100);
        assert!(s.chars().any(|c| SYMBOLS.contains(c)));
    }

    #[test]
    fn test_no_ambiguous() {
        let config = StringConfig {
            length: 1000,
            no_ambiguous: true,
            ..Default::default()
        };
        let s = generate_string(&config).unwrap();
        for bad in AMBIGUOUS_CHARS {
            assert!(!s.contains(*bad));
        }
    }

    #[test]
    fn test_custom_charset() {
        let config = StringConfig {
            length: 10,
            custom_charset: Some("xyz".to_string()),
            ..Default::default()
        };
        let s = generate_string(&config).unwrap();
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| c == 'x' || c == 'y' || c == 'z'));
    }

    #[test]
    fn test_empty_charset_error() {
        let config = StringConfig {
            length: 10,
            lowercase: false,
            uppercase: false,
            digits: false,
            symbols: false,
            custom_charset: None,
            ..Default::default()
        };
        assert!(generate_string(&config).is_err());
    }
}
