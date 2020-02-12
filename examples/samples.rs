use shuffled::Shuffled;

fn main() {
    let range: Vec<u64> = (0..100).collect();
    let sampler = &mut range.as_slice().shuffled();
    for _ in 0..20 {
        let sample: Vec<String> =
            sampler.take(5).map(|&n| n.to_string()).collect();
        println!("{}", sample.join(" "));
    }
}
