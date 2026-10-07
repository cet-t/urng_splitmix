const STEP: u32 = 0x9E37_79B9;
const A: u32 = 0x85EB_CA6B;
const B: u32 = 0xC2B2_AE35;

/// A 32-bit SplitMix generator, primarily intended for seed generation.
#[derive(Debug, Clone, Copy)]
pub struct SplitMix32 {
    state: u32,
}

impl self::SplitMix32 {
    /// Creates a generator whose initial state is `seed`.
    pub const fn new(seed: u32) -> Self {
        Self { state: seed }
    }
}

impl ::urng_core::Rng for self::SplitMix32 {
    type Word = u32;

    fn nextu(&mut self) -> Self::Word {
        self.state = self.state.wrapping_add(STEP);

        let mut z = self.state;
        z = (z ^ (z >> 16)).wrapping_mul(A);
        z = (z ^ (z >> 13)).wrapping_mul(B);
        z ^ (z >> 16)
    }
}
