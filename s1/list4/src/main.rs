/*!
* 持久化单向列表,这里只是共享链表，但不会修改，单向列表插入很简单，可以直接move
*/
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

mod stack;
fn main() {
    println!("Hello, world!");
}
