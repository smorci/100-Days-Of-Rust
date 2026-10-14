pub fn parse_miles(inp: &str) -> Vec<u32> {
    inp
        .split(",")
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

pub fn progress_days(inp: &[u32]) -> usize {
    inp.windows(2).filter(|w| w[0] < w[1]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_miles() {
        assert_eq!(vec![1, 2, 3], parse_miles("1, 2, 3"));
    }

    #[test]
    fn miles_with_newline() {
        assert_eq!(vec![1, 2, 3], parse_miles("1, 2, 3\n"));
    }

    #[test]
    fn miles_with_space() {
        assert_eq!(vec![1, 2, 3], parse_miles("1, 2, 3 "));
    }

    #[test]
    fn no_progress() {
        assert_eq!(0, progress_days(&parse_miles("1, 1, 1")));
    }

    #[test]
    fn one_progress() {
        assert_eq!(1, progress_days(&parse_miles("1, 2, 1")));
    }

    #[test]
    fn readme_examples() {
        assert_eq!(2, progress_days(&[3, 4, 1, 2]));
        assert_eq!(3, progress_days(&[10, 11, 12, 9, 10]));
        assert_eq!(1, progress_days(&[6, 5, 4, 3, 2, 9]));
        assert_eq!(0, progress_days(&[9, 9]));
    }

    #[test]
    fn multi_digit() {
        assert_eq!(vec![9, 10], parse_miles("9, 10"));
    }

    #[test]
    fn empty() {
        assert_eq!(0, progress_days(&[]));
        assert_eq!(0, progress_days(&[5]));
    }
}
