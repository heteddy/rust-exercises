use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Debug;
#[derive(Debug)]
pub struct List<T:Debug> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T:Debug> {
    element: T,
    next: Link<T>,
    previous: Link<T>,
}

impl<T:Debug> Node<T> {
    pub fn new(element: T) -> Self {
        Node {
            element,
            next: None,
            previous: None,
        }
    }
}

impl<T: Debug> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None, // 尾部指针指向最后一个node 而不是一直指向None
        }
    }
    pub fn is_empty(&self) -> bool {
        match self.tail {
            Some(_) => false,
            None => true,
        }
    }

    /// 插入一个新的节点，
    pub fn push_front(&mut self, element: T) {
        // 构造一个空节点
        let new_head = Node::new(element);
        // 头结点指向新节点，
        // 尾节点不变
        let old_head = self.head.take();
        // 直接move了old head
        match old_head {
            Some(old_node) => {
                // old
                let new_rc_head = Rc::new(RefCell::new(new_head));
                // 旧节点的 previous指向新的node
                old_node.borrow_mut().previous = Some(new_rc_head.clone());
                // 新节点的next 指向 旧节点
                new_rc_head.borrow_mut().next = Some(old_node);
                // 头指针指向新节点
                self.head = Some(new_rc_head)
            }
            None => {
                let rn = Rc::new(RefCell::new(new_head));
                self.head = Some(rn.clone());
                self.tail = Some(rn);
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        // 1. 取出第一个node；n1
        // 2. head = n1.next
        // 3. n1.next.prev = none
        // 4. 如果tail 是n1 那么 tail 是就none

        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().previous = None; // 这个是不是可以是None，
                                                           // new_head.borrow_mut().previous.take(); // 这个是不是可以是None，
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take(); // 这里不能直接设置是None，因为在map中
                }
            }
            Rc::try_unwrap(old_head).unwrap().into_inner().element
        })
    }
}

impl<T: Debug> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc() {
        let node = Node::new("element");
        // 为什么直接调用不影响
        let n = Rc::new(RefCell::new(node)).clone().clone(); //即使在调用一次
                                                             // let n = n.clone();
        println!("count of rc_1 node {}", Rc::strong_count(&n));

        let rc_b = n.clone();
        println!("count of rc_b node {}", Rc::strong_count(&rc_b));

        let rc_c: Rc<RefCell<Node<&str>>> = rc_b.clone();
        println!("count of rc_c node {}", Rc::strong_count(&rc_c));
        {
            let rc_d = rc_c.clone();
            println!("count of rc_c node {}", Rc::strong_count(&rc_d));
        }
        println!("count of rc_1 node {}", Rc::strong_count(&n));
    }
}
