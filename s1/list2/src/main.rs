/*! 
* 基于option实现链表，链表的next是Option<Box<Node>>
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
    let mut l = stack::Stack::new();
    l.push("e1".to_owned());
    l.push("e2".to_owned());
    l.push("e3".to_owned());

    println!("list = {:?}", l);

    assert!(!l.is_empty(), "list is empty {:?}", l);
    let ret1 = l.pop();
    println!("ret = {:?}", ret1);
    let ret1 = l.pop();
    println!("ret = {:?}", ret1);
    let ret1 = l.pop();
    println!("ret = {:?}", ret1);
    let ret1 = l.pop();
    assert!(ret1.is_none(), "list should be empty");

    l.push("A".to_owned());
    l.push("B".to_owned());
    l.push("C".to_owned());
    println!(" l ={:?}", l.peek());
    println!(" l ={:?}", l.peek());
    drop(l);
}
