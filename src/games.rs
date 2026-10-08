use crate::error::{Result, RrandError};
use rand::Rng;

pub const HEADS: &str = "Heads";
pub const TAILS: &str = "Tails";

/// Flips a fair coin `count` times, returning a vector of "Heads" or "Tails".
pub fn flip_coin(count: usize) -> Vec<&'static str> {
    let mut rng = rand::thread_rng();
    (0..count)
        .map(|_| if rng.gen_bool(0.5) { HEADS } else { TAILS })
        .collect()
}

/// Rolls `count` dice, each having `sides` faces (numbered 1 to `sides`).
pub fn roll_dice(sides: u32, count: usize) -> Result<Vec<u32>> {
    if sides == 0 {
        return Err(RrandError::InvalidRange {
            min: "1".to_string(),
            max: "0".to_string(),
        });
    }
    if count == 0 {
        return Ok(Vec::new());
    }

    let mut rng = rand::thread_rng();
    let rolls = (0..count)
        .map(|_| rng.gen_range(1..=sides))
        .collect();
    Ok(rolls)
}

/// Parses standard dice notation like "6", "d20", "2d6", "3d10" and rolls them.
pub fn parse_and_roll_dice(spec: &str) -> Result<Vec<u32>> {
    let spec = spec.trim().to_lowercase();
    if spec.is_empty() {
        return Err(RrandError::InvalidDiceSpec(spec));
    }

    if let Ok(sides) = spec.parse::<u32>() {
        return roll_dice(sides, 1);
    }

    if let Some(sides_str) = spec.strip_prefix('d') {
        let sides: u32 = sides_str.parse().map_err(|_| RrandError::InvalidDiceSpec(spec.clone()))?;
        return roll_dice(sides, 1);
    }

    if let Some((count_str, sides_str)) = spec.split_once('d') {
        let count: usize = count_str.parse().map_err(|_| RrandError::InvalidDiceSpec(spec.clone()))?;
        let sides: u32 = sides_str.parse().map_err(|_| RrandError::InvalidDiceSpec(spec.clone()))?;
        return roll_dice(sides, count);
    }

    Err(RrandError::InvalidDiceSpec(spec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_coin() {
        let flips = flip_coin(100);
        assert_eq!(flips.len(), 100);
        assert!(flips.iter().all(|&f| f == HEADS || f == TAILS));
        assert!(flips.contains(&HEADS));
        assert!(flips.contains(&TAILS));
    }

    #[test]
    fn test_roll_dice() {
        let rolls = roll_dice(6, 50).unwrap();
        assert_eq!(rolls.len(), 50);
        for r in rolls {
            assert!((1..=6).contains(&r));
        }
    }

    #[test]
    fn test_parse_dice() {
        let rolls_simple = parse_and_roll_dice("20").unwrap();
        assert_eq!(rolls_simple.len(), 1);
        assert!((1..=20).contains(&rolls_simple[0]));

        let rolls_d = parse_and_roll_dice("d6").unwrap();
        assert_eq!(rolls_d.len(), 1);
        assert!((1..=6).contains(&rolls_d[0]));

        let rolls_multi = parse_and_roll_dice("3d10").unwrap();
        assert_eq!(rolls_multi.len(), 3);
        for r in rolls_multi {
            assert!((1..=10).contains(&r));
        }
    }

    #[test]
    fn test_invalid_dice() {
        assert!(parse_and_roll_dice("invalid").is_err());
        assert!(parse_and_roll_dice("0d6").is_ok()); // 0 rolls
    }
}
