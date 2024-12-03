#[macro_export]
macro_rules! aoc_day {
    ($mod_:ident) => {
        AocDay {
            part1: &$mod_::part1,
            part2: &$mod_::part2,
        }
    };
}
