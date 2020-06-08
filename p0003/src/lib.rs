type Link = Option<Box<Node>>;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub value: String,
    pub left: Link,
    pub right: Link,
}

pub fn serialize(n: &Link) -> String {
    match n {
        None => "?".to_owned(),
        Some(n) => format!(
            "{} {} {}",
            base64::encode(&n.value),
            serialize(&n.left),
            serialize(&n.right)
        ),
    }
}

// Should really write a tokenizer + parser and verify expected tokens. But probably not for this
// example...
pub fn deserialize(s: &str) -> Link {
    fn deser<'a, I>(iter: &mut I) -> Link
    where
        I: std::iter::Iterator<Item = &'a str>,
    {
        match iter.next() {
            Some("?") | None => None,
            Some(x) => Some(Box::new(Node {
                value: String::from_utf8(base64::decode(x.as_bytes()).unwrap()).unwrap(),
                left: deser(iter),
                right: deser(iter),
            })),
        }
    };

    deser(&mut s.split(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_tree() -> Link {
        Some(Box::new(Node {
            value: "root".to_owned(),
            left: Some(Box::new(Node {
                value: "left".to_owned(),
                left: Some(Box::new(Node {
                    value: "left_left".to_owned(),
                    left: Some(Box::new(Node {
                        value: "left_left_left".to_owned(),
                        left: None,
                        right: None,
                    })),
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: "left_right".to_owned(),
                    left: None,
                    right: Some(Box::new(Node {
                        value: "left_right_left".to_owned(),
                        left: None,
                        right: None,
                    })),
                })),
            })),
            right: Some(Box::new(Node {
                value: "right".to_owned(),
                left: Some(Box::new(Node {
                    value: "right_left".to_owned(),
                    left: None,
                    right: None,
                })),
                right: None,
            })),
        }))
    }

    #[test]
    fn serialize_tree() {
        assert_eq!(
            "cm9vdA== bGVmdA== bGVmdF9sZWZ0 bGVmdF9sZWZ0X2xlZnQ= ? ? ? bGVmdF9yaWdodA== ? bGVmdF9yaWdodF9sZWZ0 ? ? cmlnaHQ= cmlnaHRfbGVmdA== ? ? ?",
            serialize(&get_tree()),
        );
    }

    #[test]
    fn deserialize_tree() {
        assert_eq!(
            get_tree(),
            deserialize(
                "cm9vdA== bGVmdA== bGVmdF9sZWZ0 bGVmdF9sZWZ0X2xlZnQ= ? ? ? bGVmdF9yaWdodA== ? \
                bGVmdF9yaWdodF9sZWZ0 ? ? cmlnaHQ= cmlnaHRfbGVmdA== ? ? ?"
            )
        )
    }

    #[test]
    fn full() {
        assert_eq!(deserialize(&serialize(&get_tree())), get_tree());
    }
}
