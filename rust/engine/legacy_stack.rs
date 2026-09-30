//! Preserve residual bytes in the frozen C/Wasm retry frames.
//! Sizes/offsets come from the frozen linked C/Wasm, not the Rust compiler.
//! See ../CAUSALITY.md and ../probes/retry-memory/ for byte-level evidence.
//! This is per-process state, just like the original engine's other globals.
//! All modeled bytes are initialized Rust storage; no uninitialized Rust read
//! or dependence on the host's stack allocator is required.
use core::ffi::c_char;
const CAPACITY: usize = 65_536;
static mut MEMORY: [c_char; CAPACITY] = [0; CAPACITY];
static mut SP: usize = CAPACITY;

pub struct Frame<const N: usize> {
    sp: usize,
    size: usize,
    slots: [(usize, *mut c_char, usize); N],
}
impl<const N: usize> Frame<N> {
    pub unsafe fn enter(size: usize, slots: [(usize, *mut c_char, usize); N]) -> Self {
        assert!(SP >= size, "Legacy retry stack exhausted");
        SP -= size;
        let sp = SP;
        for &(offset, pointer, length) in &slots {
            assert!(offset + length <= size);
            core::ptr::copy_nonoverlapping(
                (&raw const MEMORY).cast::<c_char>().add(sp + offset),
                pointer, length,
            );
        }
        Self { sp, size, slots }
    }
}
impl<const N: usize> Drop for Frame<N> {
    fn drop(&mut self) {
        unsafe {
            assert!(SP == self.sp, "Legacy retry frames must leave in stack order");
            for &(offset, pointer, length) in &self.slots {
                core::ptr::copy_nonoverlapping(
                    pointer,
                    (&raw mut MEMORY).cast::<c_char>().add(self.sp + offset), length,
                );
            }
            SP += self.size;
        }
    }
}

// In the frozen checkstring4/checkverb call path, analyzed_verb's temporary
// string occupies the 64 bytes immediately below the current frame. It does
// not call back into retry traversal while that temporary is live.
pub struct EndStringSlot { position:usize, pointer:*mut [c_char;60] }
impl EndStringSlot {
 pub unsafe fn enter(pointer:*mut [c_char;60])->Self {
  assert!(SP >= 64, "Legacy verb slot exhausted");
  let position=SP-64;
  core::ptr::copy_nonoverlapping((&raw const MEMORY).cast::<c_char>().add(position),pointer.cast::<c_char>(),60);
  Self {position,pointer}
 }
}
impl Drop for EndStringSlot {
 fn drop(&mut self){unsafe{core::ptr::copy_nonoverlapping(self.pointer.cast::<c_char>(),(&raw mut MEMORY).cast::<c_char>().add(self.position),60);}}
}

// checkendind is inlined into chcknend in frozen function 28. Other callers
// must not reuse this slot. Scope the location to that exact call path.
static mut NOMINAL_TAG_POSITION: Option<usize> = None;
pub struct NominalIndexContext { previous:Option<usize> }
impl NominalIndexContext {
 pub unsafe fn enter()->Self {
  let previous=NOMINAL_TAG_POSITION;NOMINAL_TAG_POSITION=Some(SP+64);Self{previous}
 }
}
impl Drop for NominalIndexContext {fn drop(&mut self){unsafe{NOMINAL_TAG_POSITION=self.previous;}}}
pub unsafe fn nominal_tag_slot(pointer:*mut [c_char;60])->Option<EndStringSlot>{
 NOMINAL_TAG_POSITION.map(|position|{
  core::ptr::copy_nonoverlapping((&raw const MEMORY).cast::<c_char>().add(position),pointer.cast::<c_char>(),60);
  EndStringSlot{position,pointer}
 })
}
