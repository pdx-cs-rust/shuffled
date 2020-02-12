//! Iterator for lazily producing references to elements of
//! a data structure in shuffled order. This consumes random
//! numbers only as elements are produced.
//!
//! # Examples
//!
//! Sample random integers from a collection.
//! 
//! ```
//! use shuffled::Shuffled;
//! 
//! let range: Vec<u64> = (0..100).collect();
//! let sampler = &mut range.as_slice().shuffled();
//! // Can use `cloned()` to dereference here if desired.
//! let samples: Vec<&u64> = sampler.take(5).collect();
//! let nsamples = samples.len();
//! assert_eq!(5, nsamples);
//! for i in 0..nsamples {
//!     for j in i+1..nsamples {
//!         assert!(samples[i] != samples[j]);
//!     }
//! }
//! assert_eq!(95, sampler.count());
//! ```


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

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.state.len(), Some(self.state.len()))
    }

    fn count(mut self) -> usize {
        let result = self.state.len();
        self.state.clear();
        result
    }

    // The first shall be last, and the last shall also be
    // last.
    fn last(mut self) -> Option<Self::Item> {
        let result = self.next()?;
        self.state.clear();
        Some(result)
    }
}
