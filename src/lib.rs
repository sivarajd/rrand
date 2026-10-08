pub mod bytes;
pub mod choice;
pub mod encoding;
pub mod error;
pub mod games;
pub mod id;
pub mod lorem;
pub mod num;
pub mod output;
pub mod passphrase;
pub mod sequence;
pub mod string;

// High-level re-exports for library users
pub use bytes::{format_bytes, generate_bytes, ByteFormat};
pub use choice::pick;
pub use encoding::{generate_base36, generate_base58, generate_base62};
pub use error::{Result, RrandError};
pub use games::{flip_coin, parse_and_roll_dice, roll_dice};
pub use id::{generate_nanoid, generate_ulid, generate_uuid, UuidFormat, UuidVersion};
pub use lorem::{generate_paragraphs, generate_sentences, generate_words};
pub use num::{format_int, generate_floats, generate_gaussian, generate_ints};
pub use passphrase::generate_passphrase;
pub use sequence::{generate_sequence, shuffle};
pub use string::{generate_string, StringConfig};
