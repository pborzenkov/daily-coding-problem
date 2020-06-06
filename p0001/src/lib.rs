pub fn p0001(sum: i32, nums: &[i32]) -> bool {
    let mut seen = std::collections::HashSet::new();

    nums.iter().any(|n| -> bool {
        if seen.contains(&(sum - n)) {
            true
        } else {
            seen.insert(n);
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_up() {
        assert!(p0001(17, &vec![10, 15, 3, 7]));
    }

    #[test]
    fn adds_up_with_negative() {
        assert!(p0001(0, &vec![10, -10, 15, 3, 7]));
    }

    #[test]
    fn adds_up_same_number() {
        assert!(p0001(10, &vec![5, 5]));
    }

    #[test]
    fn doesn_add_up() {
        assert!(!p0001(16, &vec![10, 15, 3, 7]));
    }

    #[test]
    fn doesn_add_up_single_number() {
        assert!(!p0001(5, &vec![5]));
    }
}
