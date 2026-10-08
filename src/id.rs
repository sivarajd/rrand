use rand::Rng;
use uuid::{Timestamp, Uuid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UuidVersion {
    #[default]
    V4,
    V7,
    V1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UuidFormat {
    #[default]
    Standard,
    Simple,
    Urn,
}

/// Generates a UUID with the given version and format.
pub fn generate_uuid(version: UuidVersion, format: UuidFormat, uppercase: bool) -> String {
    let id = match version {
        UuidVersion::V4 => Uuid::new_v4(),
        UuidVersion::V7 => Uuid::now_v7(),
        UuidVersion::V1 => {
            let ts = Timestamp::from_unix(uuid::ContextV1::new(0), 0, 0);
            let mut node = [0u8; 6];
            rand::thread_rng().fill(&mut node);
            Uuid::new_v1(ts, &node)
        }
    };

    let formatted = match format {
        UuidFormat::Standard => id.hyphenated().to_string(),
        UuidFormat::Simple => id.simple().to_string(),
        UuidFormat::Urn => id.urn().to_string(),
    };

    if uppercase {
        formatted.to_uppercase()
    } else {
        formatted.to_lowercase()
    }
}

/// Generates a ULID (Universally Unique Lexicographically Sortable Identifier).
pub fn generate_ulid(uppercase: bool) -> String {
    let id = ulid::Ulid::new().to_string();
    if uppercase {
        id.to_uppercase()
    } else {
        id.to_lowercase()
    }
}

pub const NANOID_DEFAULT_ALPHABET: &str =
    "_-0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Generates a NanoID of the specified length using the provided or default alphabet.
pub fn generate_nanoid(len: usize, custom_alphabet: Option<&str>) -> String {
    if len == 0 {
        return String::new();
    }
    let alphabet_chars: Vec<char> = custom_alphabet
        .unwrap_or(NANOID_DEFAULT_ALPHABET)
        .chars()
        .collect();

    if alphabet_chars.is_empty() {
        return String::new();
    }

    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..alphabet_chars.len());
            alphabet_chars[idx]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_v4() {
        let u = generate_uuid(UuidVersion::V4, UuidFormat::Standard, false);
        assert_eq!(u.len(), 36);
        assert_eq!(u.chars().filter(|&c| c == '-').count(), 4);
    }

    #[test]
    fn test_uuid_v7() {
        let u = generate_uuid(UuidVersion::V7, UuidFormat::Standard, false);
        assert_eq!(u.len(), 36);
    }

    #[test]
    fn test_uuid_simple_and_uppercase() {
        let u = generate_uuid(UuidVersion::V4, UuidFormat::Simple, true);
        assert_eq!(u.len(), 32);
        assert!(!u.contains('-'));
        assert_eq!(u, u.to_uppercase());
    }

    #[test]
    fn test_uuid_urn() {
        let u = generate_uuid(UuidVersion::V4, UuidFormat::Urn, false);
        assert!(u.starts_with("urn:uuid:"));
    }

    #[test]
    fn test_ulid() {
        let u_upper = generate_ulid(true);
        assert_eq!(u_upper.len(), 26);
        assert_eq!(u_upper, u_upper.to_uppercase());

        let u_lower = generate_ulid(false);
        assert_eq!(u_lower.len(), 26);
        assert_eq!(u_lower, u_lower.to_lowercase());
    }

    #[test]
    fn test_nanoid() {
        let n21 = generate_nanoid(21, None);
        assert_eq!(n21.len(), 21);

        let custom = generate_nanoid(10, Some("01"));
        assert_eq!(custom.len(), 10);
        assert!(custom.chars().all(|c| c == '0' || c == '1'));
    }
}
