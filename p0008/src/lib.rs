pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub fn p0008(tree: Option<Box<Node>>) -> u32 {
    fn compute(tree: Option<Box<Node>>, need: i32) -> (Option<i32>, u32) {
        if tree.is_none() {
            return (Some(need), 0);
        }
        let tree = tree.unwrap();
        let left = compute(tree.left, tree.value);
        let right = compute(tree.right, tree.value);

        if Some(tree.value) == left.0 && left.0 == right.0 {
            (Some(tree.value), left.1 + right.1 + 1)
        } else {
            (None, left.1 + right.1)
        }
    }

    let (_, unival) = compute(tree, 0);
    unival
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(value: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Option<Box<Node>> {
        Some(Box::new(Node { value, left, right }))
    }

    #[test]
    fn example() {
        assert_eq!(
            5,
            p0008(node(
                0,
                node(1, None, None),
                node(
                    0,
                    node(1, node(1, None, None), node(1, None, None)),
                    node(0, None, None)
                )
            ))
        );
    }

    #[test]
    fn empty() {
        assert_eq!(0, p0008(None));
    }

    #[test]
    fn only_root() {
        assert_eq!(1, p0008(node(1, None, None)));
    }

    #[test]
    fn one_side_unival() {
        assert_eq!(2, p0008(node(1, node(1, None, None), None)));
    }

    #[test]
    fn one_side_nonunival() {
        assert_eq!(1, p0008(node(1, node(2, None, None), None)));
    }
}
