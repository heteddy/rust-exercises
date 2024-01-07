use std::str::FromStr;
use std::{env, process};

// mod gcd;
use crate::gcd::calc;

pub fn gcd_arg() -> u64 {
    let mut numbers = Vec::new();

    for a in env::args().skip(1) {
        if let Ok(v) = u64::from_str(&a) {
            numbers.push(v);
        } else {

        }
    }
    if numbers.len() == 0 {
        eprintln!("error of get number from args");
        process::exit(1);
    }

    let mut a: u64 = numbers[0];
    for b in numbers.iter().skip(1) {
        a = calc::gcd_calc_function(a, *b);
    }
    a
}
