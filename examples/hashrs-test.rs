/* Simple hash table */
/* TODO: Write to buf writer instead of string. Check input again.. pase errors
 */
extern crate vec_impl;
extern crate anyhow;
use std::fs::{File};
use std::io::{prelude::*, BufReader};
use anyhow::{Context, Result, anyhow};

fn set_add(hset: &mut vec_impl::MyHashSet1::<i32>, val: &str, exp: &str)  -> Result<()> {
    hset.add(val.parse::<i32>().unwrap());
    if exp.as_bytes() == b"null" { Ok(()) } else { Err(anyhow!("null not {}", exp)) }
}
fn set_contains(hset: &mut vec_impl::MyHashSet1::<i32>, val: &str, exp: &str)  -> Result<()> {
    let ret = hset.contains(&val.parse::<i32>().unwrap());
    let prt_str = |ret| if ret { "true" } else { "false"};
    let prt_val = prt_str(ret);
    if exp.as_bytes() == prt_val.as_bytes() { Ok(()) } else { Err(anyhow!("{} not {}", prt_val, exp)) }
}

fn set_remove(hset: &mut vec_impl::MyHashSet1::<i32>, val: &str, exp: &str) -> Result<()> {
    hset.remove(&val.parse::<i32>().unwrap());
    if exp.as_bytes() == b"null" { Ok(()) } else { Err(anyhow!("null not {}", exp)) }
}

fn main() -> Result<()> {
    let ifile = File::open("/home/ajith/repos/vec_impl/examples/input1.txt").context("Failed to open input")?;
    let irdr = BufReader::new(ifile);
    let efile = File::open("/home/ajith/repos/vec_impl/examples/expected1.txt").context("Failed to open exp output")?;
    let erdr = BufReader::new(efile);

    let mut ilines = irdr.lines();
    let mut elines = erdr.lines();
    let ops = ilines.next().unwrap().context("Failed to read first input line")?;
    let vals = ilines.next().unwrap().context("Failed to read second input line")?;
    let exps = elines.next().unwrap().context("Failed to read expect line")?;
    let op = ops.split("\",\"");
    let mut val = vals.split("],[");
    let mut exp = exps.split(",");
    let mut first = 1;
    let mut hset = vec_impl::MyHashSet1::<i32>::new();
    for oper in op {
        match oper {
            "add" => set_add(&mut hset, val.next().unwrap(), exp.next().unwrap()).with_context(|| format!("Line {}: Failed add", first))?,
            "contains" => set_contains(&mut hset, val.next().unwrap(), exp.next().unwrap()).with_context(|| format!("Line {}: Failed contains", first))?,
            "remove" => set_remove(&mut hset, val.next().unwrap(), exp.next().unwrap()).with_context(|| format!("Line {}: Failed remove", first))?,
            _ => { println!("Unknown operation {}", oper); return Ok(());}
        }
        first = first + 1;
    }
    Ok(())
}
