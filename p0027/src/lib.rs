pub fn p0027(input: &str) -> bool {
    let mut stack = Vec::new();

    for c in input.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' => match stack.pop() {
                Some(b) if b == c => (),
                _ => return false,
            },
            _ => (),
        }
    }

    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert!(true == p0027("([])[]({})"));
        assert!(false == p0027("([)]"));
        assert!(false == p0027("((()"));
    }
}
