use base64::{engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD}, Engine as _};
use rand::RngCore;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ByteFormat {
    #[default]
    Hex,
    Base64,
    Base64Url,
    Raw,
}

/// Generates `len` cryptographically secure random bytes using thread_rng.
pub fn generate_bytes(len: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; len];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes
}

/// Formats a byte slice into the specified encoding.
pub fn format_bytes(bytes: &[u8], format: ByteFormat) -> String {
    match format {
        ByteFormat::Hex => hex::encode(bytes),
        ByteFormat::Base64 => STANDARD.encode(bytes),
        ByteFormat::Base64Url => URL_SAFE_NO_PAD.encode(bytes),
        ByteFormat::Raw => {
            // Note: For raw binary, caller should typically write directly to stdout buffer.
            // When string formatting is requested, lossy UTF-8 or raw representation is provided.
            String::from_utf8_lossy(bytes).into_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_bytes_length() {
        let b = generate_bytes(16);
        assert_eq!(b.len(), 16);
        let b0 = generate_bytes(0);
        assert_eq!(b0.len(), 0);
    }

    #[test]
    fn test_format_hex() {
        let sample = [0xde, 0xad, 0xbe, 0xef];
        assert_eq!(format_bytes(&sample, ByteFormat::Hex), "deadbeef");
    }

    #[test]
    fn test_format_base64() {
        let sample = b"hello world";
        assert_eq!(format_bytes(sample, ByteFormat::Base64), "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_format_base64url() {
        let sample = [251, 255, 254];
        let b64 = format_bytes(&sample, ByteFormat::Base64Url);
        assert!(!b64.contains('+'));
        assert!(!b64.contains('/'));
        assert!(!b64.contains('='));
    }
}
