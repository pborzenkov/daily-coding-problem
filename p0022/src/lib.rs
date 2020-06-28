pub fn p0022(phrase: &str, words: &[&str]) -> Option<Vec<String>> {
    if phrase.len() == 0 {
        return Some(Vec::new());
    }

    for &w in words {
        if phrase.starts_with(w) {
            if let Some(mut sentence) = p0022(&phrase[w.len()..], words) {
                let mut res = vec![w.to_owned()];
                res.append(&mut sentence);
                return Some(res);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            Some(vec![
                "the".to_owned(),
                "quick".to_owned(),
                "brown".to_owned(),
                "fox".to_owned()
            ]),
            p0022("thequickbrownfox", &vec!["quick", "brown", "the", "fox"])
        );
    }
}
