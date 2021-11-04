#![feature(allocator_api, alloc_layout_extra, ptr_internals)]

mod vec;
mod stack;
mod heap;
mod hash;

pub use vec::MyVec;
pub use stack::MyStack;
pub use heap::MyMaxHeap;
pub use heap::MyMinHeap;
pub use hash::MyHashSet1;

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
