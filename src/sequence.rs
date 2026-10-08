use crate::error::{Result, RrandError};
use rand::seq::SliceRandom;

/// Generates a randomized sequence (permutation) of all integers from `min` to `max` inclusive.
pub fn generate_sequence(min: i64, max: i64) -> Result<Vec<i64>> {
    if min > max {
        return Err(RrandError::InvalidRange {
            min: min.to_string(),
            max: max.to_string(),
        });
    }

    let mut values: Vec<i64> = (min..=max).collect();
    let mut rng = rand::thread_rng();
    values.shuffle(&mut rng);
    Ok(values)
}

/// Shuffles a slice in-place using Fisher-Yates.
pub fn shuffle<T>(items: &mut [T]) {
    let mut rng = rand::thread_rng();
    items.shuffle(&mut rng);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sequence() {
        let seq = generate_sequence(1, 10).unwrap();
        assert_eq!(seq.len(), 10);
        let mut sorted = seq.clone();
        sorted.sort();
        assert_eq!(sorted, (1..=10).collect::<Vec<_>>());
    }

    #[test]
    fn test_shuffle() {
        let mut original = vec!["apple", "banana", "cherry", "date", "elderberry"];
        shuffle(&mut original);
        assert_eq!(original.len(), 5);
    }
}
