use std::iter::{Filter, Iterator, Map};
use std::slice::Iter;

pub struct CrossSampler<'a, R, L>
where
    L: Iterator + Sized,
{
    /// Used to cache the current left item
    left_item: Option<L::Item>,
    left_iter: L,
    samples: &'a [R],
    sample_iter: Iter<'a, R>,
    sample_read: bool,
}

impl<'a, R, L> CrossSampler<'a, R, L>
where
    L: Iterator + Sized,
{
    pub fn next_left(&mut self) {
        self.left_item = self.left_iter.next();
    }

    pub fn new(left_iter: L, samples: &'a [R]) -> CrossSampler<'a, R, L> {
        let mut left_iter = left_iter;
        let left_item = left_iter.next();
        CrossSampler {
            left_item,
            left_iter,
            samples,
            sample_iter: samples.iter(),
            sample_read: false,
        }
    }

    pub fn get_sample(&mut self) -> Option<&'a R> {
        self.sample_read = true;
        self.sample_iter.next()
    }
}

// #[derive(Debug, PartialEq)]
pub struct CrossSamplerItem<'a, L, R, F>
where
    F: Fn() -> Option<&'a R> + Sized,
{
    left: L,
    right: F,
}

impl<'a, R, L, F> Iterator for CrossSampler<'a, R, L>
where
    F: Fn() -> Option<&'a R> + Sized,
    L: Iterator + Sized,
    L::Item: Clone,
{
    type Item = CrossSamplerItem<'a, L::Item, &'a R, F>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(CrossSamplerItem {
            left: self.left_item.unwrap(),
            right: || self.get_sample(),
        })
        // match self.sample_iter.next() {
        //     Some(right) => Some(CrossSamplerItem {
        //         right,
        //         left: self.left_item.clone().unwrap(),
        //     }),
        //     None => {
        //         self.next_left();
        //         match &self.left_item {
        //             None => return None,
        //             Some(left) => {
        //                 self.sample_iter = self.samples.iter();
        //                 Some(CrossSamplerItem {
        //                     right: self
        //                         .sample_iter
        //                         .next()
        //                         .expect("Right has at least one item"),
        //                     left: left.clone(),
        //                 })
        //             }
        //         }
        //     }
        // }
    }
}

pub trait CrossSamplerTrait: Iterator {
    fn sample<'a, 'b, S>(&'b mut self, samples: &'a [S], mut f:F) -> CrossSampler<'a, S, Self>
    where
        Self: Sized + Clone,
        F: FnMut(&mut dyn FnMut()->S) ->()
    {
        CrossSampler::new(self.clone(), samples)
    }
}

impl<'a, T> CrossSamplerTrait for Iter<'a, T> {}
impl<B, I: Iterator, F> CrossSamplerTrait for Map<I, F> where F: FnMut(I::Item) -> B {}
impl<I: Iterator, P> CrossSamplerTrait for Filter<I, P> where P: FnMut(&I::Item) -> bool {}

#[cfg(test)]
mod specs {
    use super::*;

    #[test]
    fn cross_sampler() {
        let input = [1, 2];
        let samples = [4, 5];
        let actual = input
            .iter()
            .sample(&samples)
            .collect::<Vec<CrossSamplerItem<&i32, &i32>>>();
        let expected = vec![
            CrossSamplerItem {
                left: &1,
                right: &4,
            },
            CrossSamplerItem {
                left: &1,
                right: &5,
            },
            CrossSamplerItem {
                left: &2,
                right: &4,
            },
            CrossSamplerItem {
                left: &2,
                right: &5,
            },
        ];
        assert_eq!(actual, expected);
    }
}
