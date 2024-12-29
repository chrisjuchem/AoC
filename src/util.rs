use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use variadics_please::all_tuples_with_size;

macro_rules! aoc_test {
    ($input1:expr, $part1:expr, $input2:expr, $part2:expr $(,)?) => {
        #[cfg(test)]
        mod aoc_test {
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

fn split_impl<'a, const N: usize>(input: &'a str, delim: &str) -> [&'a str; N] {
    input
        .split(delim)
        .filter(|s| *s != "")
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
pub trait SplitInto<'a, T> {
    fn split_into(&'a self, delim: &str) -> T;
}
macro_rules! impl_split_into {
    ($N:expr, $($T:ident),*) => {
        impl<'a> SplitInto<'a, ($(&'a str ${ignore($T)}),*)> for str {
            fn split_into(&'a self, delim: &str) -> ($(&'a str ${ignore($T)}),*) {
                split_impl::<$N>(self, delim).into()
            }
        }
    };
}
all_tuples_with_size!(impl_split_into, 2, 5, T);

pub trait CollectVec: Iterator {
    fn collect_vec(self) -> Vec<<Self as Iterator>::Item>;
}
impl<I> CollectVec for I
where
    I: Iterator,
{
    fn collect_vec(self) -> Vec<<Self as Iterator>::Item> {
        self.collect()
    }
}

pub trait MultiMap<K, T> {
    fn insert_multi(&mut self, key: K, t: T);
}
impl<K: Eq + Hash, T: Eq + Hash> MultiMap<K, T> for HashMap<K, HashSet<T>> {
    fn insert_multi(&mut self, key: K, t: T) {
        self.entry(key).or_default().insert(t);
    }
}

pub trait CountMap<T> {
    fn insert_one(&mut self, t: T);
    fn insert_n(&mut self, t: T, n: u64);
    fn count(&self, t: impl Borrow<T>) -> u64;
}
impl<T: Eq + Hash> CountMap<T> for HashMap<T, u64> {
    fn insert_one(&mut self, t: T) {
        self.insert_n(t, 1);
    }
    fn insert_n(&mut self, t: T, n: u64) {
        *self.entry(t).or_default() += n;
    }

    fn count(&self, t: impl Borrow<T>) -> u64 {
        self.get(t.borrow()).copied().unwrap_or(0)
    }
}

pub trait ToCountMap<T> {
    fn to_count_map(self) -> HashMap<T, u64>;
}
impl<I: Iterator<Item = T>, T: Eq + Hash> ToCountMap<T> for I {
    fn to_count_map(mut self) -> HashMap<T, u64> {
        let mut map = HashMap::new();
        while let Some(t) = self.next() {
            map.insert_one(t)
        }
        map
    }
}
