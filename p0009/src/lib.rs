use std::cmp::max;

pub fn p0009(nums: &[i32]) -> i32 {
    if nums.len() <= 2 {
        return *nums.iter().chain(std::iter::once(&0)).max().unwrap();
    }

    let mut max_wo_last = max(0, nums[0]);
    let mut max_w_last = max(max_wo_last, nums[1]);

    for n in &nums[2..] {
        let p = max_w_last;

        max_w_last = max(max_wo_last + n, max_w_last);
        max_wo_last = p;
    }

    max(max_w_last, max_wo_last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(13, p0009(&[2, 4, 6, 2, 5]));
    }

    #[test]
    fn example_2() {
        assert_eq!(10, p0009(&[5, 1, 1, 5]));
    }

    #[test]
    fn zero() {
        assert_eq!(0, p0009(&[]));
    }

    #[test]
    fn one() {
        assert_eq!(0, p0009(&[-2]));
        assert_eq!(1, p0009(&[1]));
    }

    #[test]
    fn two() {
        assert_eq!(0, p0009(&[-1, -2]));
        assert_eq!(2, p0009(&[2, 1]));
        assert_eq!(2, p0009(&[1, 2]));
    }
}
