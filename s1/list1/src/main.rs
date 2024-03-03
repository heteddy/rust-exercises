/*! 
 * 这种方式的注释只能在lib和main中，其他地方不适用，只能是//!;

 * list1 是采用栈的方式实现链表，后插入的在前面
*/
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
    assert!(ret1.is_none(),"list should be empty");

    
    l.push("A".to_owned());
    l.push("B".to_owned());
    l.push("C".to_owned());
    drop(l);
}
