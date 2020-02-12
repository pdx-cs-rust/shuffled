use rand::{rngs::ThreadRng, Rng};

pub struct Shuffled<'a, T, R: Rng> {
    rng: R,
    state: Vec<&'a T>,
}

pub trait ExtSliceShuffled<'a, T: 'a, R: Rng> {
    fn shuffled(&'a self) -> Shuffled<'a, T, R>;
}

impl<'a, T: 'a> ExtSliceShuffled<'a, T, ThreadRng> for [T] {
    fn shuffled(&'a self) -> Shuffled<'a, T, ThreadRng> {
        let rng = rand::thread_rng();
        let state: Vec<&'a T> = self.iter().collect();
        Shuffled { rng, state }
    }
}

impl<'a, T: 'a, R: Rng> Iterator for Shuffled<'a, T, R> {
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
