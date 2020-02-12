use rand::{rngs::ThreadRng, Rng};

pub struct ShuffledIter<'a, T, R: Rng> {
    rng: R,
    state: Vec<&'a T>,
}

pub trait Shuffled<'a, T: 'a> {
    fn shuffled_by<R: Rng>(&'a self, rng: R) -> ShuffledIter<'a, T, R>;

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

impl<'a, T: 'a, R: Rng> Iterator for ShuffledIter<'a, T, R> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let nstate = self.state.len();
        if nstate == 0 {
            return None;
        }
        let j = self.rng.gen_range(0, nstate);
        self.state.swap(nstate - 1, j);
        self.state.pop()
    }
}
