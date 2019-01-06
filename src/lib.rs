#![feature(allocator_api, alloc_layout_extra, ptr_internals)]

use std::{cmp, slice};
use std::mem::{self, size_of};
use std::ops::{Deref, DerefMut};
use std::ptr::{self, Unique};
// Unique does not implement move semantics, nor destroy underlying resource.
use std::alloc::{alloc, realloc, dealloc, Layout, handle_alloc_error};

/*
 * MyVec aims to provide functionality that matches std::vec::Vec.
 */
pub struct MyVec<T> {
    my_vec: Unique<T>,
    layout: Layout,
    len: usize,
    reserve: usize,
}

/*
 * IntoIter iterates while consuming the data. It begins from the first element.
 * Disintegrate MyVec to get a new data structure.
 * Ways to wrap MyVec instead:
 * Use pop to read backwards. To read from beginning, with the same scheme,
 * into_iter could take a one time cost of O(n) to reverse Myvec.
 * Its hard for MyVec to provide safe methods for IntoIter. A safe
 * pop_front on MyVec would only be possible with a circular buffer..
 */
pub struct IntoIter<T> {
    my_vec: Unique<T>,
    layout: Layout,
    len: usize,
    next: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.len {
            None
        } else {
            let ret = unsafe { ptr::read(self.my_vec.as_ptr().offset(self.next as isize)) };
            self.next = self.next + 1;
            Some(ret)
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        unsafe {
            for index in 0..self.len {
                ptr::read(self.my_vec.as_ptr().offset(index as isize));
            }
            if self.len != 0 {
                dealloc(mem::transmute(self.my_vec.as_ptr()), self.layout);
            }
        }
    }
}

impl<T> MyVec<T> {
    pub fn new(reserve: Option<usize>) -> Self {
        MyVec {
            my_vec: Unique::empty(),
            layout: Layout::new::<()>(),
            len: 0,
            reserve: cmp::max(reserve.unwrap_or(32), 4),
        }
    }

    fn capacity(&self) -> usize {
        self.layout.size() / size_of::<T>()
    }

    fn erase(&mut self) {
        self.my_vec = Unique::empty();
        self.layout = Layout::new::<()>();
        self.len = 0;
        self.reserve = 4
    }

    fn grow(&mut self) {
        // Allocate one size if len is 0
        if self.capacity() == 0 {
            unsafe {
                let layout = Layout::array::<T>(self.reserve).unwrap();
                let ptr = alloc(layout);
                if ptr.is_null() {
                    handle_alloc_error(layout);
                }
                self.layout = layout;
                self.my_vec = Unique::new_unchecked(mem::transmute(ptr));
            }
        } else {
            // grow by reallocate if size is not zero.
            unsafe {
                // Double the layout by extending it by its own size.
                let layout = self.layout.extend(self.layout).unwrap().0;
                let ptr = realloc(
                    mem::transmute(self.my_vec.as_ptr()),
                    self.layout,
                    layout.size(),
                );
                if ptr.is_null() {
                    handle_alloc_error(layout);
                }
                self.layout = layout;
                // Own the allocated pointer.
                self.my_vec = Unique::new_unchecked(mem::transmute(ptr));
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        // if no space left, resize to increase capacity.
        if self.len == self.capacity() {
            self.grow();
        }
        // append
        // TODO: self.len is usize, offset expects isize. Overflow?
        unsafe {
            ptr::write(self.my_vec.as_ptr().offset(self.len as isize), elem);
            self.len = self.len + 1;
        }
    }

    /* Comes from Deref.
    pub fn get(&self, index: usize) -> Option<&T> {}
    pub fn iter(&self) -> Iter<T> {}
    */

    pub fn into_iter(mut self) -> IntoIter<T> {
        let iter = IntoIter {
            my_vec: self.my_vec,
            layout: self.layout,
            len: self.len,
            next: 0,
        };
        self.erase();
        iter
    }

    fn trim(&mut self) {
        // Let minimum size remain at reserve. TODO: constant 4.
        let target_size = self.capacity() / 2;
        if (self.capacity() >= self.reserve * 2) && (self.len <= target_size / 2) {
            unsafe {
                let layout = Layout::array::<T>(target_size).unwrap();
                let ptr = realloc(
                    mem::transmute(self.my_vec.as_ptr()),
                    self.layout,
                    layout.size(),
                );
                if ptr.is_null() {
                    handle_alloc_error(layout);
                }
                self.layout = layout;
                self.my_vec = Unique::new_unchecked(mem::transmute(ptr));
            }
        }
    }

    pub fn back(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            self.get(self.len - 1)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            let ret = unsafe {
                self.len = self.len - 1;
                // len is now index.
                ptr::read(self.my_vec.as_ptr().offset(self.len as isize))
            };
            self.trim();
            Some(ret)
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            for index in 0..self.len {
                ptr::read(self.my_vec.as_ptr().offset(index as isize));
            }
            if self.capacity() != 0 {
                dealloc(mem::transmute(self.my_vec.as_ptr()), self.layout);
            }
        }
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe {
            slice::from_raw_parts(self.my_vec.as_ptr(), self.len)
        }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            slice::from_raw_parts_mut(self.my_vec.as_ptr(), self.len)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MyVec;

    /*
     * Test the assumptions made in the implementation.
     * alloc, dealloc and interaction with Unique.
     */
    #[test]
    fn test_alloc_unique() {
        use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
        use std::ptr::{Unique};
        use std::mem;

        let layout = Layout::array::<i32>(10).unwrap();
        unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        let un: Unique<i32> = Unique::new_unchecked(mem::transmute(ptr));
        let _oth_un = un;
        // No move semantics as Unique, by definition, is never null.
        assert_eq!(un.as_ptr().is_null(), false);
        // un is not even made invalid, can still be dealloc-ated.
        dealloc(mem::transmute(un.as_ptr()), layout);
        // Run with address/leak sanitizer to look for use after free/leaks.
        }
    }

    #[test]
    fn test_vec_simple() {
        let _my_vec: MyVec<i32> = MyVec::new(None);
        // Move
        let _other_vec = _my_vec;
        for _elem in _other_vec.iter() {
            assert_eq!(_elem, &0);
        }
    }

    /* Test vector of simple integers, for Deref and DerefMut */
    #[test]
    fn test_vec_int() {
        let mut ints = vec![1, 2, 3, 4, 5];
        let mut my_vec: MyVec<i32> = MyVec::new(None);

        for elem in ints.iter() {
            my_vec.push_back(*elem);
        }

        /* Verify Deref into splice, access method of splice. */
        assert_eq!(my_vec.is_empty(), false);
        assert_eq!(my_vec.len(), ints.len());
        assert_eq!(my_vec.first(), Some(&1));
        assert_eq!(my_vec.last(), Some(&5));

        {
            /* In a block to scope immutable borrow. */
            let mut splits = my_vec.split(|n| n % 2 == 0);
            assert_eq!(splits.next().unwrap(), &[1]);
            assert_eq!(splits.next().unwrap(), &[3]);
            assert_eq!(splits.next().unwrap(), &[5]);
        }

        assert_eq!(my_vec[my_vec.len() - 1], 5);
        for i in 0..30 {
            assert_eq!(my_vec.get(i), ints.get(i));
        }
        /* One of those times when borrow checker is too conservative */
        let vec_len = my_vec.len()/2;
        my_vec[vec_len] = 35;
        let vec_len = ints.len()/2;
        ints[vec_len] = my_vec[my_vec.len()/2];
        for i in 0..30 {
            assert_eq!(my_vec.get(i), ints.get(i));
        }
    }

    /*
     * Test iterating a vector of references to strings.
     */
    #[test]
    fn test_vec_str() {
        let strings = vec!["So", "far", "so", "good"];
        let mut my_vec: MyVec<&str> = MyVec::new(None);

        // Setup.
        for elem in strings.iter() {
            my_vec.push_back(elem);
        }

        // Test iteration by borrowing.
        // Compare values.
        for elem in my_vec.iter().enumerate() {
            assert_eq!(*elem.1, *strings.get(elem.0).unwrap());
        }

        // Compare references.
        for elem in my_vec.iter().enumerate() {
            assert_eq!(elem.1, strings.get(elem.0).unwrap());
        }

        // Test iteration by consuming.
        for elem in my_vec.into_iter().enumerate() {
            assert_eq!(elem.1, *strings.get(elem.0).unwrap());
        }
    }

    // Type that holds an allocated value.
    use std::ptr::{Unique};
    use std::mem;
    use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
    #[derive(Debug)]
    struct AT<T> {
        alloc: Unique<T>,
        layout: Layout,
    }

    impl<T> AT<T> {
        fn new(count: usize) -> Self {
            let layout = Layout::array::<T>(std::cmp::max(count, 4)).unwrap();
            unsafe {
                let ptr = alloc(layout);
                if ptr.is_null() {
                    handle_alloc_error(layout);
                }
                AT {
                    alloc: Unique::new_unchecked(mem::transmute(ptr)),
                    layout: layout,
                    }
            }
        }
    }

    impl<T> Drop for AT<T> {
        // Run test with RUSTFLAGS="-Z sanitizer=address"
        fn drop(&mut self) {
            unsafe {
                dealloc(mem::transmute(self.alloc.as_ptr()), self.layout);
            }
        }
    }

    // Integration if allocated type with vector. The test is setup to allocate
    // memory. Use address sanitizer to verify that API releases memory as well.
    #[test]
    fn test_vec_at_val() {
        let mut my_vec: MyVec<AT<i32>> = MyVec::new(None);
        for i in 0..30 {
            my_vec.push_back(AT::new(10 + i));
        }
    }

    // Type that holds a reference to a string.
    #[derive(Debug)]
    struct RT<'a> {
        val: i32,
        name: &'a str,
    }

    impl<'a> RT<'a> {
        fn new(val: i32) -> Self {
            RT { val, name: "Star" }
        }
    }

    impl<'a> Drop for RT<'a> {
        fn drop(&mut self) {
            //println!("Called drop for RT ({}, {})", self.val, self.name);
        }
    }

    /*
     * Test iterating a vector of RTs.
     */
    #[test]
    fn test_vec_rt_val() {
        let mut my_vec: MyVec<RT> = MyVec::new(Some(2));

        let ints = vec![15, 150, 200, 250, 0, -15, -150, -200, -250];
        for elem in ints.iter() {
            my_vec.push_back(RT::new(*elem));
        }

        assert_eq!(my_vec.capacity(), 16);
        assert_eq!(my_vec.len, 9);

        // Drain the elements
        for elem in ints.iter().rev() {
            assert_eq!(*elem, my_vec.pop().unwrap().val);
        }
        assert_eq!(my_vec.len, 0);
        assert_eq!(my_vec.pop().is_none(), true);
        assert_eq!(my_vec.back().is_none(), true);
        assert_eq!(my_vec.capacity(), 4);
    }

    /*
     * Test iterating a vector of references to RTs.
     */
    #[test]
    fn test_vec_rt_ref() {
        let mut rts: MyVec<RT> = MyVec::new(Some(2));
        let mut rt_refs: MyVec<&RT> = MyVec::new(Some(2));

        let ints = vec![15, 150, 200, 250, 0, -15, -150, -200, -250];
        for elem in ints.iter() {
            rts.push_back(RT::new(*elem));
            // Filling references to rt_refs here will be blocked by the compiler
            // rt_refs.push_back(rts.back().unwrap());
        }

        for i in 0..ints.len() {
            rt_refs.push_back(rts.get(i).unwrap());
        }

        assert_eq!(rt_refs.capacity(), 16);

        for i in (0..ints.len()).rev() {
            assert_eq!(rt_refs.get(i).unwrap().val, rt_refs.pop().unwrap().val);
        }
    }
}
