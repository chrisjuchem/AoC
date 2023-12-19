macro_rules! aoc_test {
    ($input:expr, $part1:expr, $part2:expr $(,)?) => {
        #[cfg(test)]
        mod test {
            use super::{part1, part2};

            #[test]
            fn test_part1() {
                assert_eq!(part1($input.to_string()), $part1);
            }

            #[test]
            fn test_part2() {
                assert_eq!(part2($input.to_string()), $part2);
            }
        }
    };
}
pub(crate) use aoc_test;
