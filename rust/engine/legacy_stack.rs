//! Preserve residual bytes in the frozen C/Wasm retry frames.
//! Sizes/offsets come from Emscripten 6.0.6 -O2 (see ../probes/retry-frames.s).
//! This is per-process state, just like the original engine's other globals.
use core::ffi::c_char;
const CAPACITY: usize = 65_536;
static mut MEMORY: [c_char; CAPACITY] = [0; CAPACITY];
static mut SP: usize = CAPACITY;

pub struct Frame<const N: usize> {
    sp: usize,
    size: usize,
    slots: [(usize, *mut [c_char; 60]); N],
}
impl<const N: usize> Frame<N> {
    pub unsafe fn enter(size: usize, slots: [(usize, *mut [c_char; 60]); N]) -> Self {
        assert!(SP >= size, "Legacy retry stack exhausted");
        SP -= size;
        let sp = SP;
        for &(offset, pointer) in &slots {
            assert!(offset + 60 <= size);
            core::ptr::copy_nonoverlapping(
                (&raw const MEMORY).cast::<c_char>().add(sp + offset),
                pointer.cast::<c_char>(), 60,
            );
        }
        Self { sp, size, slots }
    }
}
impl<const N: usize> Drop for Frame<N> {
    fn drop(&mut self) {
        unsafe {
            assert!(SP == self.sp, "Legacy retry frames must leave in stack order");
            for &(offset, pointer) in &self.slots {
                core::ptr::copy_nonoverlapping(
                    pointer.cast::<c_char>(),
                    (&raw mut MEMORY).cast::<c_char>().add(self.sp + offset), 60,
                );
            }
            SP += self.size;
        }
    }
}
