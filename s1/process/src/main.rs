#![cfg_attr(
    debug_assertions,
    allow(
        unused,
        dead_code,
        unused_imports,
        unused_variables,
        unused_assignments,
        non_snake_case
    )
)]
use std::io::prelude::*;
use std::process::{Command,Stdio};


static PANGRAM: &'static str ="the quick brown fox jumped over the lazy dog\n";

fn main() {
    let process = Command::new("wc").stdin(Stdio::piped())
    .stdout(Stdio::piped()).spawn().expect("error of spawn process");

    // 2行 3个词 27字节

    process.stdin.unwrap().write_all("hedetao \n贺德涛 \nteddyhe".as_bytes()).unwrap();

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Ok(s2) =>{println!("received:{:?}, length={:?}",s,s2)},
        Err(e) => {
            println!("error={:?}",e);
        }
    }
    println!("Hello, world!");
}
