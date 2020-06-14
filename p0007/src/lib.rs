use std::collections::HashMap;

fn is_valid_letter(i: &str) -> bool {
    let n: u8 = i.parse().unwrap();
    n >= 10 && n <= 26
}

pub fn p0007(input: &str) -> u32 {
    let mut memo = HashMap::new();

    fn compute(input: &str, memo: &mut HashMap<usize, u32>) -> u32 {
        if input.len() < 2 {
            return 0;
        }

        if let Some(&result) = memo.get(&input.len()) {
            return result;
        }

        let result = if is_valid_letter(&input[..2]) {
            1 + compute(&input[2..], memo)
        } else {
            0
        } + compute(&input[1..], memo);

        memo.insert(input.len(), result);
        result
    }

    1 + compute(input, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(3, p0007("111"));
    }

    #[test]
    fn single_letter() {
        assert_eq!(1, p0007("1"));
    }

    #[test]
    fn two_letters() {
        assert_eq!(1, p0007("77"));
    }

    #[test]
    fn two_letters_with_choice() {
        assert_eq!(2, p0007("10"));
    }

    #[test]
    fn four_letters() {
        assert_eq!(1, p0007("3333"));
    }

    #[test]
    fn four_letters_with_choice_1() {
        assert_eq!(4, p0007("2611"));
    }

    #[test]
    fn four_letters_with_choice_2() {
        assert_eq!(5, p0007("1111"));
    }

    #[test]
    fn five_letters() {
        assert_eq!(8, p0007("11111"));
    }
}
