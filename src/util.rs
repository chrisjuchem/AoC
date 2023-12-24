macro_rules! aoc_test {
    ($input1:expr, $part1:expr, $input2:expr, $part2:expr $(,)?) => {
        #[cfg(test)]
        mod test {
            use super::{part1, part2};

            #[test]
            fn test_part1() {
                assert_eq!(part1($input1.to_string()), $part1);
            }

            #[test]
            fn test_part2() {
                assert_eq!(part2($input2.to_string()), $part2);
            }
        }
    };
    ($input:expr, $part1:expr, $part2:expr $(,)?) => {
        aoc_test!($input, $part1, $input, $part2);
    };
}
pub(crate) use aoc_test;

pub trait SplitInto<'a, T> {
    fn split_into(&'a self, delim: &str) -> T;
}
impl<'a> SplitInto<'a, (&'a str, &'a str)> for str {
    fn split_into(&'a self, delim: &str) -> (&'a str, &'a str) {
        let array: [_; 2] = self
            .split(delim)
            .filter(|s| *s != "")
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        array.into()
    }
}
impl<'a> SplitInto<'a, (&'a str, &'a str, &'a str)> for str {
    fn split_into(&'a self, delim: &str) -> (&'a str, &'a str, &'a str) {
        let array: [_; 3] = self.split(delim).collect::<Vec<_>>().try_into().unwrap();
        array.into()
    }
}

pub fn parse_grid<T>(input: String, f: impl Copy + Fn(char) -> T) -> Vec<Vec<T>> {
    input
        .lines()
        .map(|row| row.chars().map(f).collect())
        .collect()
}
