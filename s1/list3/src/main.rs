/*!
* 模板化的链表，并且实现了intoIter和Iter  IterMut等 通过next方法遍历和修改链表的值
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
    let mut list = stack::List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));

    // 这里map的参数怎么写  |value| 还是 |&mut value|; 由于拿到的是一个 &mut value; map中的闭包是一个pattern匹配，如果写成|&mut value|；得到的是一个值
    list.peek_mut().map(|value| *value = 42);

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
    let mut list2 = stack::List::new();
    list2.push(1);
    list2.push(2);
    list2.push(3);

    let mut iter = list2.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));

    let mut list3 = stack::List::new();
    list3.push(1);
    list3.push(2);
    list3.push(3);
    let mut iter = list3.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}
