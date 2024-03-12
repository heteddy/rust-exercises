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
mod fn1;

fn main() {
    /*
    {
        FnOnce {
            FnMut {
                Fn
            }
        }
    }
    */

    let fn_plain = fn1::create_fn();
    let mut fn_mut = fn1::create_fnmut();
    let fn_once = fn1::create_fnonce();

    fn_plain();
    fn_plain();
    fn_mut();
    fn_mut();
    fn_once();
    // fn_once();
}
