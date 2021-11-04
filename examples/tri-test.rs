/* Simple hash table */
/* TODO: Write to buf writer instead of string. Check input again.. pase errors
 */
extern crate vec_impl;
extern crate anyhow;
use std::ffi::CString;

use std::fs::{File};
use std::io::{prelude::*, BufReader};
use anyhow::{Context, Result, anyhow};

unsafe fn trie_insert(trie: *mut vec_impl::MyTrieBurst, val: &str, exp: &str)  -> Result<()> {
    let cword = CString::new(val.as_bytes()).expect("Cstring Failed");
    vec_impl::myTrieBurstInsert(trie, cword.as_ptr());
    print!("null, ");
    if exp.as_bytes() == b"null" { Ok(()) } else { Err(anyhow!("null not {}", exp)) }
}
unsafe fn search(trie: *mut vec_impl::MyTrieBurst, val: &str, exp: &str)  -> Result<()> {
    let cword = CString::new(val.as_bytes()).expect("Cstring Failed");
    let ret = vec_impl::myTrieBurstSearch(trie, cword.as_ptr());
    let prt_str = |ret| if ret == 0 { "false" } else { "true"};
    let prt_val = prt_str(ret);
    print!("{},", prt_val);
    if exp.as_bytes() == prt_val.as_bytes() { Ok(()) } else { Err(anyhow!("{} not {}", prt_val, exp)) }
}
unsafe fn starts_with(trie: *mut vec_impl::MyTrieBurst, val: &str, exp: &str)  -> Result<()> {
    let cword = CString::new(val.as_bytes()).expect("Cstring Failed");
    let ret = vec_impl::myTrieBurstStartsWith(trie, cword.as_ptr());
    let prt_str = |ret| if ret == 0 { "false" } else { "true"};
    let prt_val = prt_str(ret);
    print!("{},", prt_val);
    if exp.as_bytes() == prt_val.as_bytes() { Ok(()) } else { Err(anyhow!("{} not {}", prt_val, exp)) }
}

fn main() -> Result<()> {
    let ifile = File::open("/home/ajith/repos/vec_impl/examples/input2.txt").context("Failed to open input")?;
    let irdr = BufReader::new(ifile);
    let efile = File::open("/home/ajith/repos/vec_impl/examples/expected2.txt").context("Failed to open exp output")?;
    let erdr = BufReader::new(efile);

    let mut ilines = irdr.lines();
    let mut elines = erdr.lines();
    let ops = ilines.next().unwrap().context("Failed to read first input line")?;
    let vals = ilines.next().unwrap().context("Failed to read second input line")?;
    let exps = elines.next().unwrap().context("Failed to read expect line")?;
    let op = ops.split("\",\"");
    let mut val = vals.split("\"],[\"");
    let mut exp = exps.split(",");
    let mut first = 1;
    unsafe {
    let trie = vec_impl::myTrieBurstCreate();
    for oper in op {
        match oper {
            "insert" => trie_insert(trie, val.next().unwrap(), exp.next().unwrap()).with_context(|| format!("Line {}: Failed insert", first))?,
            "search" => search(trie, val.next().unwrap(), exp.next().unwrap()).with_context(|| format!("Line {}: Failed search", first))?,
            "startsWith" => starts_with(trie, val.next().unwrap(), exp.next().unwrap()).with_context(|| format!("Line {}: Failed startswith", first))?,
            _ => { println!("Unknown operation {}", oper); return Ok(());}
        }
        first = first + 1;
    }
    vec_impl::myTrieBurstFree(trie);
    }
    Ok(())
}
