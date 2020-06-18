use std::cmp::max;
use std::collections::HashMap;

pub fn p0013(s: &str, k: u32) -> u32 {
    let mut letters = HashMap::new();
    let mut max_len = 0;
    let mut lidx = 0;

    let s = s.as_bytes();
    for (ridx, c) in s.iter().enumerate() {
        if let Some(l) = letters.get_mut(c) {
            *l += 1;
        } else {
            letters.insert(s[ridx], 1);
            while letters.len() > k as usize {
                let l = letters.get_mut(&s[lidx]).unwrap();
                *l -= 1;
                if *l == 0 {
                    letters.remove(&s[lidx]);
                }
                lidx += 1;
            }
        }
        max_len = max(max_len, ridx - lidx + 1);
    }

    max_len as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(3, p0013("abcba", 2));
    }

    #[test]
    fn one() {
        assert_eq!(1, p0013("abcdef", 1));
        assert_eq!(2, p0013("abccdef", 1));
        assert_eq!(3, p0013("abcccdef", 1));
        assert_eq!(4, p0013("abcdeffff", 1));
    }

    #[test]
    fn two() {
        assert_eq!(5, p0013("abcbccdef", 2));
    }
}
