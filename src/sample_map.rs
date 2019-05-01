use std::iter::{Filter, Iterator, Map};
use std::slice::Iter;

pub struct SampleMap<'a, I, F, S> {
    iter: I,
    f: F,
    s: &'a [S],
}

impl<'a, I, F, S> SampleMap<'a, I, F, S> {
    pub(super) fn new(iter: I, f: F, s: &'a [S]) -> SampleMap<'a, I, F, S> {
        SampleMap { iter, f, s }
    }
}

type P<S> = FnMut() -> S;

impl<'a, B, I: Iterator, F, S> Iterator for SampleMap<'a, I, F, S>
where
    F: FnMut(I::Item, fn() -> i32) -> B,
    // P: FnMut() -> S + Sized,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        let s = self.s.iter().nth(0).unwrap();
        match self.iter.next() {
            Some(value) => Some((self.f)(value, || 1)),
            None => None,
        }
    }
}

pub trait SampleMapTrait: Iterator {
    fn sample_map<'a, B, F, S, P>(self, f: F, s: &'a [S]) -> SampleMap<'a, Self, F, S>
    where
        Self: Sized,
        P: FnMut() -> S + Sized,
        F: FnMut(Self::Item, P) -> B,
    {
        SampleMap::new(self, f, s)
    }
}

impl<'a, T> SampleMapTrait for Iter<'a, T> {}

#[cfg(test)]
mod specs {
    use super::*;

    #[test]
    fn sample_map_simple() {
        let input = [1, 2];
        let actual = input
            .iter()
            .sample_map(|s, f: fn() -> i32| 2 * s * f(), &[2, 3])
            .collect::<Vec<i32>>();
        let expected = [2, 4];

        assert_eq!(actual, expected);
    }
}
