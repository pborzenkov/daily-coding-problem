pub fn p0021(lectures: &[(u32, u32)]) -> u32 {
    let mut times: Vec<(u32, i32)> = lectures
        .iter()
        .map(|&(start, end)| vec![(start, 1), (end, -1)])
        .flatten()
        .collect();

    times.sort_unstable();

    let mut rooms = 0;
    let mut max_rooms = 0;
    times.iter().for_each(|&(_, start_end)| {
        rooms += start_end;
        max_rooms = std::cmp::max(max_rooms, rooms);
    });

    max_rooms as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(2, p0021(&[(30, 75), (0, 50), (60, 150)]));
    }
}
