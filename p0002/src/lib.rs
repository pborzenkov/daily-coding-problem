pub fn p0002(nums: &[i32]) -> Vec<i32> {
    let len = nums.len();
    if len < 2 {
        return vec![0; len];
    }

    let (mut forward, mut backward) = (vec![0; len], vec![0; len]);
    for i in 0..len {
        forward[i] = forward.get(i.wrapping_sub(1)).unwrap_or(&1) * nums[i];
        backward[len - i - 1] = backward.get(len - i).unwrap_or(&1) * nums[len - i - 1];
    }

    (0..len)
        .map(|idx| {
            forward.get(idx.wrapping_sub(1)).unwrap_or(&1) * backward.get(idx + 1).unwrap_or(&1)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(p0002(&vec![1, 2, 3, 4, 5]), vec![120, 60, 40, 30, 24]);
    }

    #[test]
    fn example_2() {
        assert_eq!(p0002(&vec![3, 2, 1]), vec![2, 3, 6]);
    }

    #[test]
    fn empty() {
        assert_eq!(p0002(&vec![]), vec![]);
    }

    #[test]
    fn one_element() {
        assert_eq!(p0002(&vec![10]), vec![0]);
    }

    #[test]
    fn two_elements() {
        assert_eq!(p0002(&vec![1, 2]), vec![2, 1]);
    }
}
