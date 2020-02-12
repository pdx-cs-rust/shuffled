# shuffled: Lazy iterator in shuffled order

An iterator over a data structure that returns references to
elements in shuffled order. An implentation for slices is
provided.

This is primarily a demo of iterators and traits. That said,
it might be generally useful.

See the Rustdoc for documentation.

## Example

```rust
use shuffled::Shuffled;

let range: Vec<u64> = (0..100).collect();
let sampler = &mut range.as_slice().shuffled();
// Can use `cloned()` to dereference here if desired.
let samples: Vec<&u64> = sampler.take(5).collect();
let nsamples = samples.len();
assert_eq!(5, nsamples);
for i in 0..nsamples {
    for j in i+1..nsamples {
        assert!(samples[i] != samples[j]);
    }
}
assert_eq!(95, sampler.count());
```
