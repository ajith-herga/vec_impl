/* All nearest (from the left) smaller values. */
extern crate vec_impl;
use std::slice;
/* For a given number, nearest smaller is either immediately to the left or
   in the stored previous results. When none are found, the given number itself
   is nearest smaller.
*/
fn computeansv(iterator: slice::Iter<i32>) {
    // Store the nearest smallers of interest.
    // They are unique and get pruned when a smaller value is encountered.
    let mut uresvec = vec_impl::MyStack::new();
    // Store answer in a vec.
    let mut resvec = vec_impl::MyVec::new(None);
    let mut prev_elem: i32 = 0;
    for (index, elem) in iterator.enumerate() {
        if index == 0 {
            prev_elem = *elem;
            continue;
        }
        if prev_elem < *elem {
            resvec.push_back(prev_elem);
            uresvec.push(prev_elem);
        } else if prev_elem == *elem {
            resvec.push_back(prev_elem);
        } else {
            loop {
                if let Some(result_elem) = uresvec.top() {
                    if *result_elem < *elem {
                        resvec.push_back(*result_elem);
                        break;
                    }
                } else {
                    resvec.push_back(*elem);
                    break;
                }
                // Did not find a good stored result, pop stack.
                uresvec.pop();
            }
        }
        // Debug
        // println!("At {}:{}\nAnswer: {}\nLows: {}", index, elem, resvec, uresvec);
        prev_elem = *elem;
    }
    print!("Answer: -,");
    print!("{} \n", resvec);
}

fn main() {
    let numbers = vec_impl::myvec![1, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 0, 11, 7, 15];
    println!("Input: {}\n", numbers);
    computeansv(numbers.iter());
}
