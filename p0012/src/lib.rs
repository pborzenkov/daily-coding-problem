use std::collections::HashMap;

pub fn p0012(steps: u32, choices: &[u32]) -> u32 {
    let mut memo = HashMap::new();

    fn compute(steps: u32, choices: &[u32], memo: &mut HashMap<u32, u32>) -> u32 {
        if let Some(&val) = memo.get(&steps) {
            return val;
        }

        let mut res = 0;
        for c in choices {
            res += match c {
                &c if c == steps => 1,
                &c if c > steps => 0,
                _ => compute(steps - c, choices, memo),
            }
        }

        memo.insert(steps, res);
        res
    }

    compute(steps, choices, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(5, p0012(4, &[1, 2]));
    }
}
