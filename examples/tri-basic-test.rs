/* Simple hash table */
extern crate vec_impl;
use std::ffi::CString;
fn main() {
    unsafe {
    let trie = vec_impl::myTrieBurstCreate();
    let words = ["this", "is", "good", "india", "great", "thinkers", "this", "thiswas"];
    for word in words.iter() {
        let cword = CString::new(word.as_bytes()).expect("Cstring Failed");
        println!("Has word {} ? {}", word, vec_impl::myTrieBurstSearch(trie, cword.as_ptr()));
        vec_impl::myTrieBurstInsert(trie, cword.as_ptr());
        println!("Has word {} ? {}", word, vec_impl::myTrieBurstSearch(trie, cword.as_ptr()));
    }
    let words = ["tis", "i", "goo", "indi", "grea", "think", "thisw"];
    for word in words.iter() {
        let cword = CString::new(word.as_bytes()).expect("Cstring Failed");
        println!("Has word {} ? {}", word, vec_impl::myTrieBurstStartsWith(trie, cword.as_ptr()));
    }
    vec_impl::myTrieBurstFree(trie);
    }
}
