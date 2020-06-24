pub fn p0019(costs: &[Vec<u32>]) -> Option<u32> {
    if costs.len() < 1 {
        return None;
    }

    let mut cost = (0, 0);
    let mut idx = usize::MAX;

    for c in costs.iter() {
        let mut new_cost = (u32::MAX, u32::MAX);
        let mut new_idx = 0;

        for (k, p) in c.iter().enumerate() {
            let prev_cost = if idx != k { cost.0 } else { cost.1 };
            let cost = p + prev_cost;

            if cost < new_cost.0 {
                new_cost.0 = cost;
                new_idx = k
            } else if cost < new_cost.1 {
                new_cost.1 = cost
            }
        }

        cost = new_cost;
        idx = new_idx;
    }

    Some(cost.0)
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
