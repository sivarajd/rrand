use crate::error::{Result, RrandError};
use rand::seq::SliceRandom;
use rand::Rng;

/// Picks `count` items from `items`. If `unique` is true, items are sampled without replacement.
pub fn pick<T: Clone>(items: &[T], count: usize, unique: bool) -> Result<Vec<T>> {
    if items.is_empty() {
        return Err(RrandError::InsufficientUniqueItems {
            requested: count,
            available: 0,
        });
    }
    if count == 0 {
        return Ok(Vec::new());
    }

    if unique && count > items.len() {
        return Err(RrandError::InsufficientUniqueItems {
            requested: count,
            available: items.len(),
        });
    }

    let mut rng = rand::thread_rng();

    if unique {
        let mut indices: Vec<usize> = (0..items.len()).collect();
        indices.shuffle(&mut rng);
        let chosen: Vec<T> = indices.iter().take(count).map(|&i| items[i].clone()).collect();
        Ok(chosen)
    } else {
        let mut chosen = Vec::with_capacity(count);
        for _ in 0..count {
            let idx = rng.gen_range(0..items.len());
            chosen.push(items[idx].clone());
        }
        Ok(chosen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_unique() {
        let items = vec!["a", "b", "c", "d", "e"];
        let chosen = pick(&items, 3, true).unwrap();
        assert_eq!(chosen.len(), 3);
        let mut sorted = chosen.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 3);
    }

    #[test]
    fn test_pick_unique_too_many() {
        let items = vec!["a", "b"];
        let res = pick(&items, 3, true);
        assert!(res.is_err());
    }

    #[test]
    fn test_pick_with_replacement() {
        let items = vec!["a", "b"];
        let chosen = pick(&items, 10, false).unwrap();
        assert_eq!(chosen.len(), 10);
    }
}
