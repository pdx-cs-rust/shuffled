//! Iterator for lazily producing references to elements of
//! a data structure in shuffled order. This consumes random
//! numbers only as elements are produced.

use rand::{rngs::ThreadRng, Rng};

/// Iterator for lazily producing references to
/// elements.
pub struct ShuffledIter<'a, T, R: Rng> {
    rng: R,
    state: Vec<&'a T>,
}

/// Extension trait for lazily producing elements in
/// shuffled order from a given collection.
pub trait Shuffled<'a, T: 'a> {
    /// Get an iterator that lazily produces elements in
    /// shuffled order from a given collection using the
    /// given `Rng`.
    fn shuffled_by<R: Rng>(&'a self, rng: R) -> ShuffledIter<'a, T, R>;

    /// Get an iterator that lazily produces elements in
    /// shuffled order from a given collection using
    /// `rand::ThreadRng`.
    fn shuffled(&'a self) -> ShuffledIter<'a, T, ThreadRng> {
        self.shuffled_by(rand::thread_rng())
    }
}

impl<'a, T: 'a> Shuffled<'a, T> for [T] {
    fn shuffled_by<R: Rng>(&'a self, rng: R) -> ShuffledIter<'a, T, R> {
        let state: Vec<&'a T> = self.iter().collect();
        ShuffledIter { rng, state }
    }
}

/// Iterator for lazily producing references to elements of
/// a data structure in shuffled order.
impl<'a, T: 'a, R: Rng> Iterator for ShuffledIter<'a, T, R> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let last = self.state.pop()?;
        let top = self.state.len();
        let j = self.rng.gen_range(0, top + 1);
        if j < top {
            let result = self.state[j];
            self.state[j] = last;
            Some(result)
        } else {
            Some(last)
        }
    }
}
