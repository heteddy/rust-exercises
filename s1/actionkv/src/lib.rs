#![cfg_attr(
    debug_assertions,
    allow(
        unused,
        // unused_imports,
        // dead_code,
        async_fn_in_trait, // 关闭async 函数的警告
        unreachable_patterns,
        unused_variables,
        unused_assignments,
        non_snake_case
    )
)]

use std;

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub mod cache;
pub mod config;
pub mod dao;
pub mod endpoint;
pub mod pb;
pub mod search;
pub mod server;
pub mod transport;
pub mod utils;
