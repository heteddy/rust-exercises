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
use bincode::de;
use bincode::deserialize as from_bincode;

mod data;
mod dump;

fn main() {
    dump::dump();
}
