pub fn p0019(costs: &[Vec<u32>]) -> Option<u32> {
    if costs.len() < 1 {
        return None;
    }

    let mut cache = [costs[0].clone(), vec![0; costs[0].len()]];
    for i in 1..costs.len() {
        for (k, p) in costs[i].iter().enumerate() {
            cache[i % 2][k] = cache[(i - 1) % 2]
                .iter()
                .enumerate()
                .filter(|&(ck, _)| ck != k)
                .map(|(_, cp)| p + cp)
                .min()
                .unwrap()
        }
    }

    cache[(costs.len() - 1) % 2].iter().min().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(None, p0019(&vec![]));
    }

    #[test]
    fn simple() {
        assert_eq!(
            Some(6),
            p0019(&vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]])
        );
    }

    #[test]
    fn complex() {
        assert_eq!(
            Some(6),
            p0019(&vec![
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 1, 1, 2],
                vec![3, 6, 8, 9, 2],
                vec![1, 1, 1, 4, 4],
                vec![2, 2, 1, 3, 4],
            ])
        );
    }
}
