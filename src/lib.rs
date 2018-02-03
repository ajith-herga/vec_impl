#![feature(allocator_api, ptr_internals, unique)]

use std::mem::{size_of, self};
use std::ptr::{Unique};
use std::heap::{Alloc, Heap, Layout};

pub struct MyVec<T> {
vec: Unique<T>,
layout: Layout,
len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        MyVec { vec: Unique::empty(), layout: Layout::new::<()>(), len: 0 }
    }

    fn resize(&mut self)
    {
        // Allocate one size if len is 0
        if self.layout.size() == 0
        {
            unsafe {
                let layout = Layout::array::<T>(32).unwrap();
                let ptr = Heap.alloc(layout.clone()).unwrap();
                self.layout = layout;
                self.vec = Unique::new_unchecked(mem::transmute(ptr));
            }
        } else {
        // Reallocate if size is not zero.
            unsafe {
                let layout = Layout::array::<T>(self.layout.size()*2).unwrap();
                let ptr = Heap.realloc(
                    mem::transmute(self.vec.as_ptr()),
                    self.layout.clone(),
                    layout.clone()
                    ).unwrap();
                self.layout = layout;
                self.vec = Unique::new_unchecked(mem::transmute(ptr));
            }
        }
    }

    // TODO deallocate? drop will be called.
    pub fn insert(&mut self, elem: T) {
        // if full, alloc
        if self.len * size_of::<T>() == self.layout.size() {
            self.resize();
        }
        // write
        // find the offset, len?
    }
}

#[cfg(test)]
mod tests {
    use super::MyVec;
    use std::mem::size_of;
    #[test]
    fn it_works() {
        let mut vec: MyVec<i32> = MyVec::new();
        vec.insert(15);
        assert_eq!(vec.layout.size(), 32*size_of::<i32>());
    }
}
