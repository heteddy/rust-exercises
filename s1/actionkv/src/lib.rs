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

use std;
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub mod server;
pub mod data;
pub mod config;