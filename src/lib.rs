#![feature(allocator_api, alloc_layout_extra, ptr_internals)]

mod vec;
mod stack;

pub use vec::MyVec;
pub use stack::MyStack;
