use rand::prelude::*;

pub fn p0014() -> f64 {
    let mut rng = rand::thread_rng();

    let hits = std::iter::repeat_with(|| (rng.gen::<f64>(), rng.gen::<f64>()))
        .take(1000)
        .filter(|(x, y)| x * x + y * y <= 1.)
        .count();

    (hits * 4) as f64 / 1000.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi() {
        println!("π is {:.3}", p0014());
    }
}
