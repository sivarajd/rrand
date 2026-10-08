use rand::Rng;

pub const BASE36_LOWER: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
pub const BASE36_UPPER: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const BASE62: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const BASE58: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Generates a random Base36 string of the given length.
pub fn generate_base36(len: usize, uppercase: bool) -> String {
    let alphabet = if uppercase { BASE36_UPPER } else { BASE36_LOWER };
    sample_alphabet(alphabet, len)
}

/// Generates a random Base62 string of the given length.
pub fn generate_base62(len: usize) -> String {
    sample_alphabet(BASE62, len)
}

/// Generates a random Base58 string of the given length (Bitcoin alphabet, avoiding 0, O, I, l).
pub fn generate_base58(len: usize) -> String {
    sample_alphabet(BASE58, len)
}

fn sample_alphabet(alphabet: &[u8], len: usize) -> String {
    if len == 0 {
        return String::new();
    }
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..alphabet.len());
            alphabet[idx] as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base36_lower() {
        let s = generate_base36(32, false);
        assert_eq!(s.len(), 32);
        assert!(s.chars().all(|c| c.is_ascii_digit() || (c.is_ascii_lowercase() && c.is_alphabetic())));
    }

    #[test]
    fn test_base36_upper() {
        let s = generate_base36(32, true);
        assert_eq!(s.len(), 32);
        assert!(s.chars().all(|c| c.is_ascii_digit() || (c.is_ascii_uppercase() && c.is_alphabetic())));
    }

    #[test]
    fn test_base62() {
        let s = generate_base62(64);
        assert_eq!(s.len(), 64);
        assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_base58() {
        let s = generate_base58(50);
        assert_eq!(s.len(), 50);
        assert!(!s.contains('0'));
        assert!(!s.contains('O'));
        assert!(!s.contains('I'));
        assert!(!s.contains('l'));
    }
}
