#![allow(dead_code, unused_imports)]

mod gcd; // main里面虽然没有用，但是要声明mod gcd 否则不会加入到工程
mod arg;
mod web;

fn main() {
    println!("Hello, world!");
    // println!("{:?}", gcd::calc::gcd_calc_function(100, 2));
    let d:u64 = arg::gcd_arg();
    println!("the max gcd is {}", d)
}
