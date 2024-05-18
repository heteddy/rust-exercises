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
use std::{io, thread};

mod tests {
    use super::*;
    #[test]
    fn test_thread() {
        let count = thread::available_parallelism().unwrap().get();
        println!("count={:?}",count);
        assert!(count > 1_usize);
    }
}
