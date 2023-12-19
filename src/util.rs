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
        aoc_test!($input, $part1, $input, $Part2)
    };
}
pub(crate) use aoc_test;
