pub fn p0004(slice: &mut [i32]) -> i32 {
    let mut idx = 0;
    let len = slice.len() as i32;
    while idx < len {
        match slice[idx as usize] {
            x if x <= 0 || x - 1 >= len || slice[x as usize - 1] == x as i32 => idx += 1,
            x if x - 1 <= idx => {
                slice[x as usize - 1] = x as i32;
                idx += 1;
            }
            x => slice.swap(idx as usize, x as usize - 1),
        }
    }

    let mut idx = 0;
    while idx < len && slice[idx as usize] == idx + 1 {
        idx += 1
    }
    idx + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, p0004(&mut vec![3, 4, 1, -1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, p0004(&mut vec![1, 2, 0]));
    }

    #[test]
    fn all_non_positive() {
        assert_eq!(1, p0004(&mut vec![-1, -2, -10, 0]));
    }

    #[test]
    fn large() {
        assert_eq!(1, p0004(&mut vec![10, 15, 7, 8, 32]));
    }

    #[test]
    fn swap_forward() {
        assert_eq!(
            11,
            p0004(&mut vec![6, 7, 8, 9, 10, 3, 4, 5, 1, -10, 6, 12, 6, 2])
        );
    }

    #[test]
    fn duplicates() {
        assert_eq!(2, p0004(&mut vec![1, 1, 1, 1, 1, 1, 1]));
    }

    #[test]
    fn increasing() {
        assert_eq!(10, p0004(&mut vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }

    #[test]
    fn decreasing() {
        assert_eq!(11, p0004(&mut vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]));
    }
}
