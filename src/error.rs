use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RrandError {
    #[error("Invalid range: min ({min}) must be less than or equal to max ({max})")]
    InvalidRange { min: String, max: String },

    #[error("Cannot sample {requested} unique items from a set of size {available}")]
    InsufficientUniqueItems { requested: usize, available: usize },

    #[error("Invalid character set: charset cannot be empty")]
    EmptyCharset,

    #[error("Invalid dice specification: '{0}'. Expected format like '2d6', 'd20', or '6'")]
    InvalidDiceSpec(String),

    #[error("Invalid radix: {0}. Supported radices are 2, 8, 10, 16")]
    InvalidRadix(u32),

    #[error("Invalid count: count must be greater than 0")]
    ZeroCount,

    #[error("Invalid standard deviation: std_dev must be positive (got {0})")]
    InvalidStdDev(f64),
}

pub type Result<T> = std::result::Result<T, RrandError>;
