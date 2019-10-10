/* All nearest (from the left) smaller values. */
extern crate vec_impl;
use std::slice;
/*
 * For a given number, nearest smaller is either immediately to the left or in the stored previous
 * results. When none are found print none.
 */

/* A wrap struct to impl result */
struct SmallerClose(std::option::Option<i32>);

impl std::fmt::Display for SmallerClose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(value) = self.0 {
            write!(f, "{}", value)
        } else {
            write!(f, "-")
        }
    }
}
fn computeansv(iterator: slice::Iter<i32>) {
    /*
     * Store the nearest smallers, how does this change?  They get pruned when a smaller value is
     * encountered.
     */
    let mut uresvec = vec_impl::MyStack::new();
    /*
     * A toy array to store result: can be int or none, None when there is nothing smaller in the
     * left.
     */
    let mut resvec = vec_impl::MyVec::new(None);
    for elem in iterator {
        loop {
            if let Some(result_elem) = uresvec.top() {
                if *result_elem < *elem {
                    resvec.push_back(SmallerClose(Some(*result_elem)));
                    break;
                }
            } else {
                resvec.push_back(SmallerClose(None));
                break;
            }
            uresvec.pop();
        }
        uresvec.push(*elem);
    }
    /*
     * All this to just print result.. may be we need to have a custom struct
     */
    println!("\nAnswer: {}", resvec);
}

fn main() {
    let numbers = vec_impl::myvec![1, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 0, 11, 7, 15];
    println!("Input: {}\n", numbers);
    computeansv(numbers.iter());
}
