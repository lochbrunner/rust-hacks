use std::iter::{Filter, Iterator, Map};
use std::slice::Iter;

// Learn trait

pub trait LearnTrait: Iterator {
    fn learn(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<'a, T> LearnTrait for Iter<'a, T> {}
impl<B, I: Iterator, F> LearnTrait for Map<I, F> where F: FnMut(I::Item) -> B {}
impl<I: Iterator, P> LearnTrait for Filter<I, P> where P: FnMut(&I::Item) -> bool {}

#[cfg(test)]
mod specs_learn {
    use super::*;

    #[test]
    fn sam_trait() {
        let input = [1, 2, 3];
        let actual = input.iter().learn().unwrap();
        let expected = 1;
        assert_eq!(actual, &expected);
    }

    #[test]
    fn sam_trait_map() {
        let input = [1, 2, 3];
        let actual = input.iter().map(|a| 2 * a).learn().unwrap();
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn sam_trait_filter() {
        let input = [1, 2, 3];
        let actual = input.iter().filter(|a| **a > 1).learn().unwrap();
        let expected = 2;
        assert_eq!(actual, &expected);
    }
}
