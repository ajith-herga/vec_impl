#![feature(allocator_api, ptr_internals, unique)]

use std::{cmp, slice};
use std::mem::{self, size_of};
use std::ops::{Deref, DerefMut};
use std::ptr::{self, Unique};
use std::heap::{Alloc, AllocErr, Heap, Layout};

pub struct MyVec<T> {
    my_vec: Unique<T>,
    layout: Layout,
    len: usize,
    reserve: usize,
}

/*
 * IntoIter reads data from the first element.  Disintegrate MyVec to get a new
 * data structure.
 * Ways to wrap MyVec instead: Use pop to read backwards.
 * into_iter could take a one time cost of O(n) to reverse Myvec so
 * IntoIter::next could use pop().  Its hard for MyVec to provide safe methods
 * for IntoIter.  There could be a pop_front on MyVec, but then MyVec needs to
 * change fundamentally to a circular buffer..
 */
pub struct IntoIter<T> {
    my_vec: Unique<T>,
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
        for index in 0..self.len {
            unsafe {
                ptr::read(self.my_vec.as_ptr().offset(index as isize));
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

    fn resize(&mut self) -> Result<(), AllocErr> {
        // Allocate one size if len is 0
        if self.capacity() == 0 {
            unsafe {
                let layout = Layout::array::<T>(self.reserve).unwrap();
                let ptr = Heap.alloc(layout.clone())?;
                self.layout = layout;
                self.my_vec = Unique::new_unchecked(mem::transmute(ptr));
            }
        } else {
            // Reallocate if size is not zero.
            unsafe {
                let layout = self.layout.extend(self.layout.clone()).unwrap().0;
                let ptr = Heap.realloc(
                    mem::transmute(self.my_vec.as_ptr()),
                    self.layout.clone(),
                    layout.clone(),
                )?;
                self.layout = layout;
                self.my_vec = Unique::new_unchecked(mem::transmute(ptr));
            }
        }
        Ok(())
    }

    pub fn push_back(&mut self, elem: T) {
        // if no space left, resize to increase capacity.
        if self.len == self.capacity() {
            self.resize().unwrap();
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

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            my_vec: self.my_vec,
            len: self.len,
            next: 0,
        }
    }

    fn trim(&mut self) -> Result<(), AllocErr> {
        // Let minimum size remain at reserve. TODO: constant 4.
        let target_size = self.capacity() / 2;
        if (self.capacity() >= self.reserve * 2) && (self.len <= target_size / 2) {
            unsafe {
                let layout = Layout::array::<T>(target_size).unwrap();
                let ptr = Heap.realloc(
                    mem::transmute(self.my_vec.as_ptr()),
                    self.layout.clone(),
                    layout.clone(),
                )?;
                self.layout = layout;
                self.my_vec = Unique::new_unchecked(mem::transmute(ptr));
            }
        }
        Ok(())
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
            self.trim().unwrap();
            Some(ret)
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        for index in 0..self.len {
            unsafe {
                ptr::read(self.my_vec.as_ptr().offset(index as isize));
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

    #[test]
    fn test_vec_str() {
        let strings = vec!["So", "far", "so", "good"];
        let mut my_vec: MyVec<&str> = MyVec::new(None);

        for elem in strings.iter() {
            my_vec.push_back(elem);
        }

        for elem in my_vec.iter().enumerate() {
            assert_eq!(*elem.1, *strings.get(elem.0).unwrap());
        }

        for elem in my_vec.iter().enumerate() {
            assert_eq!(elem.1, strings.get(elem.0).unwrap());
        }

        for elem in my_vec.into_iter().enumerate() {
            assert_eq!(elem.1, *strings.get(elem.0).unwrap());
        }
    }

    #[derive(Debug)]
    struct RT<'a> {
        val: i32,
        name: &'a str,
    }

    impl<'a> RT<'a> {
        fn new(val: i32) -> Self {
            RT { val, name: "Ajith" }
        }
    }

    impl<'a> Drop for RT<'a> {
        fn drop(&mut self) {
            //println!("Called drop for RT ({}, {})", self.val, self.name);
        }
    }

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

    #[test]
    fn test_vec_rt_ref() {
        let mut rts: MyVec<RT> = MyVec::new(Some(2));
        let mut my_vec: MyVec<&RT> = MyVec::new(Some(2));

        let ints = vec![15, 150, 200, 250, 0, -15, -150, -200, -250];
        for elem in ints.iter() {
            rts.push_back(RT::new(*elem));
            // Filling references to my_vec here will be blocked by the compiler
            // my_vec.push_back(rts.back().unwrap());
        }

        for i in 0..ints.len() {
            my_vec.push_back(rts.get(i).unwrap());
        }

        assert_eq!(my_vec.capacity(), 16);

        for i in (0..ints.len()).rev() {
            assert_eq!(my_vec.get(i).unwrap().val, my_vec.pop().unwrap().val);
        }
    }
}
