//From ![https://github.com/RustAudio/vst-rs/blob/master/src/util/atomic_float.rs]

use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Default)]
pub struct AtomicF32 {
    atomic: AtomicU32,
}

impl AtomicF32 {
    #[inline]
    /// New atomic float with initial value `value`.
    pub const fn new(value: f32) -> AtomicF32 {
        AtomicF32 {
            atomic: AtomicU32::new(value.to_bits()),
        }
    }

    #[inline]
    /// Get the current value of the atomic float.
    pub fn get(&self) -> f32 {
        f32::from_bits(self.atomic.load(Ordering::Relaxed))
    }

    #[inline]
    /// Set the value of the atomic float to `value`.
    pub fn set(&self, value: f32) {
        self.atomic.store(value.to_bits(), Ordering::Relaxed)
    }
}
