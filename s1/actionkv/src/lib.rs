#![cfg_attr(
debug_assertions,
allow(
// unused,
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

pub mod search;
pub mod config;
pub mod transport;
pub mod endpoint;
pub mod server;
pub mod pb;
pub mod dao;
pub mod sync;

pub mod utils;