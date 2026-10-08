use crate::error::{Result, RrandError};
use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::collections::HashSet;

/// Generates `count` random 64-bit integers in the range `[min, max]`.
/// If `unique` is true, values are sampled without replacement.
pub fn generate_ints(min: i64, max: i64, count: usize, unique: bool) -> Result<Vec<i64>> {
    if min > max {
        return Err(RrandError::InvalidRange {
            min: min.to_string(),
            max: max.to_string(),
        });
    }
    if count == 0 {
        return Ok(Vec::new());
    }

    let range_size = (max as i128 - min as i128 + 1) as usize;

    if unique && count > range_size {
        return Err(RrandError::InsufficientUniqueItems {
            requested: count,
            available: range_size,
        });
    }

    let mut rng = rand::thread_rng();

    if unique {
        // If range is relatively small, shuffle the full range
        if range_size <= 100_000 {
            let mut all_vals: Vec<i64> = (min..=max).collect();
            all_vals.shuffle(&mut rng);
            all_vals.truncate(count);
            Ok(all_vals)
        } else {
            // Otherwise use a hashset to collect unique samples
            let mut set = HashSet::with_capacity(count);
            while set.len() < count {
                let val = rng.gen_range(min..=max);
                set.insert(val);
            }
            Ok(set.into_iter().collect())
        }
    } else {
        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            result.push(rng.gen_range(min..=max));
        }
        Ok(result)
    }
}

/// Formats an integer in the specified radix (2, 8, 10, or 16).
pub fn format_int(val: i64, radix: u32) -> Result<String> {
    match radix {
        2 => {
            if val < 0 {
                Ok(format!("-{:b}", -val))
            } else {
                Ok(format!("{:b}", val))
            }
        }
        8 => {
            if val < 0 {
                Ok(format!("-{:o}", -val))
            } else {
                Ok(format!("{:o}", val))
            }
        }
        10 => Ok(val.to_string()),
        16 => {
            if val < 0 {
                Ok(format!("-{:x}", -val))
            } else {
                Ok(format!("{:x}", val))
            }
        }
        other => Err(RrandError::InvalidRadix(other)),
    }
}

/// Generates `count` uniformly distributed 64-bit floating point numbers in `[min, max)`.
pub fn generate_floats(min: f64, max: f64, count: usize) -> Result<Vec<f64>> {
    if min > max {
        return Err(RrandError::InvalidRange {
            min: min.to_string(),
            max: max.to_string(),
        });
    }
    if count == 0 {
        return Ok(Vec::new());
    }

    let mut rng = rand::thread_rng();
    let mut results = Vec::with_capacity(count);
    for _ in 0..count {
        if (min - max).abs() < f64::EPSILON {
            results.push(min);
        } else {
            results.push(rng.gen_range(min..max));
        }
    }
    Ok(results)
}

/// Generates `count` numbers sampled from a Gaussian/Normal distribution.
pub fn generate_gaussian(mean: f64, std_dev: f64, count: usize) -> Result<Vec<f64>> {
    if std_dev <= 0.0 {
        return Err(RrandError::InvalidStdDev(std_dev));
    }
    if count == 0 {
        return Ok(Vec::new());
    }

    let normal = Normal::new(mean, std_dev).map_err(|_| RrandError::InvalidStdDev(std_dev))?;
    let mut rng = rand::thread_rng();
    let mut results = Vec::with_capacity(count);
    for _ in 0..count {
        results.push(normal.sample(&mut rng));
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ints_range() {
        let ints = generate_ints(10, 20, 50, false).unwrap();
        assert_eq!(ints.len(), 50);
        for v in ints {
            assert!((10..=20).contains(&v));
        }
    }

    #[test]
    fn test_generate_ints_unique() {
        let ints = generate_ints(1, 10, 10, true).unwrap();
        assert_eq!(ints.len(), 10);
        let mut sorted = ints.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 10);
    }

    #[test]
    fn test_unique_overflow_error() {
        let res = generate_ints(1, 5, 10, true);
        assert!(res.is_err());
    }

    #[test]
    fn test_format_int_radices() {
        assert_eq!(format_int(15, 2).unwrap(), "1111");
        assert_eq!(format_int(15, 8).unwrap(), "17");
        assert_eq!(format_int(15, 10).unwrap(), "15");
        assert_eq!(format_int(15, 16).unwrap(), "f");
        assert!(format_int(15, 3).is_err());
    }

    #[test]
    fn test_generate_floats() {
        let floats = generate_floats(0.0, 1.0, 20).unwrap();
        assert_eq!(floats.len(), 20);
        for f in floats {
            assert!((0.0..1.0).contains(&f));
        }
    }

    #[test]
    fn test_generate_gaussian() {
        let g = generate_gaussian(0.0, 1.0, 100).unwrap();
        assert_eq!(g.len(), 100);
        assert!(generate_gaussian(0.0, -1.0, 5).is_err());
    }
}
