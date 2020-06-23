use std::cmp::max;

pub fn p0017(dirs: &str) -> usize {
    let mut entries = Vec::new();
    let mut max_len: usize = 0;

    for entry in dirs.split('\n') {
        let depth = entry.rfind('\t').map_or(0, |x| x + 1);

        entries.truncate(depth);
        if entry.contains('.') {
            max_len = max(
                max_len,
                entries.len() + entries.iter().sum::<usize>() + entry.len() - depth,
            );
        } else {
            entries.push(entry.len() - depth)
        }
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            32,
            p0017(
                "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
            )
        );
    }

    #[test]
    fn no_file() {
        assert_eq!(
            0,
            p0017("dir\n\tsubdir1\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2")
        );
    }
}
