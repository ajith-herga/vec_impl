#![feature(allocator_api, ptr_internals, unique)]

use std::mem;
use std::ptr::{Unique};
use std::heap::{Alloc, Heap, Layout};

pub struct Vec<T> {
vec: Unique<T>,
capacity: u32,
len: u32,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec { vec: Unique::empty(), capacity: 0, len: 0 }
    }

    // TODO deallocate? drop will be called.
    fn insert(&mut self, elem: T) {
        // if full, alloc
        if self.len == self.capacity {
            // Allocate one size if len is 0
            if self.len == 0 {
                unsafe {
                    let ptr = Heap.alloc(Layout::array::<T>(32).unwrap()).unwrap();
                    self.vec = Unique::new_unchecked(mem::transmute(ptr));
                }
            }
            // Reallocate if size is not zero.
        }
        // write
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let vec = Vec::new();

        assert_eq!(2 + 2, 4);
    }
}
