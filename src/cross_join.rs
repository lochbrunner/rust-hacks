use std::iter::{Filter, Iterator, Map};
use std::slice::Iter;

pub struct CrossJoin<'a, R, L>
where
    L: Iterator + Sized,
{
    /// Used to cache the current left item
    left_item: Option<L::Item>,
    left_iter: L,
    samples: &'a [R],
    sample_iter: Iter<'a, R>,
}

impl<'a, R, L> CrossJoin<'a, R, L>
where
    L: Iterator + Sized,
{
    pub fn next_left(&mut self) {
        self.left_item = self.left_iter.next();
    }

    pub fn new(left_iter: L, samples: &'a [R]) -> CrossJoin<'a, R, L> {
        let mut left_iter = left_iter;
        let left_item = left_iter.next();
        CrossJoin {
            left_item,
            left_iter,
            samples,
            sample_iter: samples.iter(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CrossJoinItem<L, R> {
    left: L,
    right: R,
}

impl<'a, R, L> Iterator for CrossJoin<'a, R, L>
where
    L: Iterator + Sized,
    L::Item: Clone,
{
    type Item = CrossJoinItem<L::Item, &'a R>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.sample_iter.next() {
            Some(right) => Some(CrossJoinItem {
                right,
                left: self.left_item.clone().unwrap(),
            }),
            None => {
                self.next_left();
                match &self.left_item {
                    None => return None,
                    Some(left) => {
                        self.sample_iter = self.samples.iter();
                        Some(CrossJoinItem {
                            right: self
                                .sample_iter
                                .next()
                                .expect("Right has at least one item"),
                            left: left.clone(),
                        })
                    }
                }
            }
        }
    }
}

pub trait CrossJoinTrait: Iterator {
    fn cross<'a, 'b, S>(&'b mut self, samples: &'a [S]) -> CrossJoin<'a, S, Self>
    where
        Self: Sized + Clone,
    {
        CrossJoin::new(self.clone(), samples)
    }
}

impl<'a, T> CrossJoinTrait for Iter<'a, T> {}
impl<B, I: Iterator, F> CrossJoinTrait for Map<I, F> where F: FnMut(I::Item) -> B {}
impl<I: Iterator, P> CrossJoinTrait for Filter<I, P> where P: FnMut(&I::Item) -> bool {}

#[cfg(test)]
mod specs {
    use super::*;

    #[test]
    fn cross_iter() {
        let input = [1, 2];
        let samples = [4, 5];
        let actual = input
            .iter()
            .cross(&samples)
            .collect::<Vec<CrossJoinItem<&i32, &i32>>>();
        let expected = vec![
            CrossJoinItem {
                left: &1,
                right: &4,
            },
            CrossJoinItem {
                left: &1,
                right: &5,
            },
            CrossJoinItem {
                left: &2,
                right: &4,
            },
            CrossJoinItem {
                left: &2,
                right: &5,
            },
        ];
        assert_eq!(actual, expected);

        input.iter().map(|s| s);
    }
}
