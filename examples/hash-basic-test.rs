/* Simple hash table */
extern crate vec_impl;

fn main() {
    unsafe {
    let hset = vec_impl::myHashSetCreate();
    for ind in 0..10 {
        let key = ind * 10000;
        println!("Has key {} ? {}", key, vec_impl::myHashSetContains(hset, key));
        vec_impl::myHashSetAdd(hset, key);
        println!("Has key {} ? {}", key, vec_impl::myHashSetContains(hset, key));
    }
    for ind in 0..10 {
        let key = ind * 10000;
        println!("Has key {} ? {}", key, vec_impl::myHashSetContains(hset, key));
        vec_impl::myHashSetRemove(hset, key);
        println!("Has key {} ? {}", key, vec_impl::myHashSetContains(hset, key));
    }
    vec_impl::myHashSetFree(hset);
    }
}
