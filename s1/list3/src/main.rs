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
    let mut list = stack::List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));
    list.peek_mut().map(|value| *value = 42);

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
    
}
