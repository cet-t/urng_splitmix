# urng_splitmix

SplitMix32 / SplitMix64 for [`urng_core`](https://crates.io/crates/urng_core).
`no_std`, no allocation.

These generators are primarily intended for **seed generation**: turning a
single seed (`u32` / `u64`) into well-mixed values used to initialize the
larger state of other generators. Every seed is valid, including `0`.

> Not cryptographically secure.
> Primarily intended for seed generation rather
> than general-purpose random number generation.

## Usage

```toml
[dependencies]
urng_splitmix = "0.1"
urng_core = "0.1"
```

```rust
use urng_core::Rng;
use urng_splitmix::{SplitMix32, SplitMix64};

let mut rng = SplitMix64::new(0);
let _: f64 = rng.randf(-1.0, 1.0);

let mut rng = SplitMix32::new(1);
let _: u32 = rng.nextu();
```
