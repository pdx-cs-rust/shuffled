use shuffled::ExtSliceShuffled;

fn main() {
    let sample: Vec<u64> = (0..100).collect();
    let sampler = &mut sample.as_slice().shuffled();
    for _ in 0..20 {
        let sample: Vec<u64> = sampler.take(5).cloned().collect();
        println!("{:?}", sample);
    }
}
