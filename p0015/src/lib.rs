use rand::prelude::*;

pub fn p0015<I, T>(stream: I) -> Option<T>
where
    I: std::iter::IntoIterator<Item = T>,
{
    let mut rng = rand::thread_rng();
    let mut ret = None;

    for (i, v) in stream.into_iter().enumerate() {
        if rng.gen_range(0, i + 1) == 0 {
            ret = Some(v)
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", p0015(&[] as &[i32]));
        println!("{:?}", p0015(&[1]));
        println!("{:?}", p0015(&[1, 2, 3, 4, 5, 6, 7, 8, 9]));
        println!("{:?}", p0015(&["dog"]));
        println!("{:?}", p0015(&["dog", "cat", "parrot", "rabbit"]));
    }
}
