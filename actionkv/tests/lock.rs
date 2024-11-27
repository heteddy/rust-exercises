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
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lock() {
        let mut i = 100;
        let l1 = Mutex::new(i);
        let mut _l1 = l1.lock().unwrap();
        *_l1 = *_l1 + 10;
        println!("l1={:?}", *_l1);
        drop(_l1);
        for i in 1..=5 {
            let mut _l2 = l1.lock().unwrap();
            *_l2 = *_l2 + 1;
            println!("l={:?}", *_l2);
        }
    }
}
