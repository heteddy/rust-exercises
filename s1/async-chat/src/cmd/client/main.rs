#![cfg_attr(
    debug_assertions,
    allow(
        dead_code,
        unused_imports,
        unused_variables,
        unused_assignments,
        non_snake_case
    )
)]

use async_std::task;

fn main() -> (){
    let address = std::env::args().nth(1).expect("Usage: client address:port");
    println!("address = {} ", &address);
    
    
    return;
}