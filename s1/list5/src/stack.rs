use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::rc::Rc;
#[derive(Debug)]
pub struct List<T: Debug> {
    head: Link<T>,
    tail: Link<T>,
}

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T: Debug> {
    element: T,
    next: Link<T>,
    previous: Link<T>,
}

impl<T: Debug> Node<T> {
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
        // let new_head = Node::new(element);
        // // 头结点指向新节点，
        // // 尾节点不变
        // let old_head = self.head.take();
        // // 直接move了old head
        // match old_head {
        //     Some(old_node) => {
        //         // old
        //         let new_rc_head = Rc::new(RefCell::new(new_head));
        //         // 旧节点的 previous指向新的node
        //         old_node.borrow_mut().previous = Some(new_rc_head.clone());
        //         // 新节点的next 指向 旧节点
        //         new_rc_head.borrow_mut().next = Some(old_node);
        //         // 头指针指向新节点
        //         self.head = Some(new_rc_head)
        //     }
        //     None => {
        //         let rn = Rc::new(RefCell::new(new_head));
        //         self.head = Some(rn.clone());
        //         self.tail = Some(rn);
        //     }
        // }
    }
    /// 插入一个元素到tail
    // pub fn push_back(&mut self, element: T) {
    //     let mut new_node = Node::new(element);
    //     let old_tail = self.tail.take();
    //
    //     match old_tail {
    //         Some(old_tail_node) => {
    //             new_node.previous = Some(old_tail_node.clone());
    //             let new_rc_node = Rc::new(RefCell::new(new_node));
    //             self.tail = Some(new_rc_node.clone());
    //             // 本身是空
    //             old_tail_node.borrow_mut().next = Some(new_rc_node);
    //         }
    //         None => {
    //             // 如果是空，构造一个
    //             let new_rc_node = Rc::new(RefCell::new(new_node));
    //             self.head = Some(new_rc_node.clone());
    //             self.tail = Some(new_rc_node);
    //         }
    //     }
    // }
    pub fn pop_front(&mut self) -> Option<T> {
        // 1. 取出第一个node first_node
        // 2. 取出第二个node second_node
        // 3. 第二个node的previous指向None
        // 4. self.head = Some(second_node)
        // 取出第一个link，使用map获取option里面内容(第1个node)，
        self.head.take().map(|first_node| {
            match first_node.borrow_mut().next.take() {
                Some(second_node) => {
                    //第二个node作为头节点
                    // new_head.borrow_mut().previous.take(); // 这两种方式效果一样，因为不需要当前previous的值
                    second_node.borrow_mut().previous = None; // 这个是不是可以是None，

                    self.head = Some(second_node);
                }
                None => {
                    self.tail.take(); // 这里不能直接设置是None，因为在map中
                }
            }
            Rc::try_unwrap(first_node).unwrap().into_inner().element
        })
    }
    /// 获取链表最后一个元素的值，
    // pub fn pop_back(&mut self) -> Option<T> {
    //     //1. 取出tail
    //     let old_tail = self.tail.take();

    //     match old_tail {
    //         Some(old_tail_node) => {
    //             let old_tail_node_refcell = old_tail_node.into_inner().previous.take();
    //             let new_tail_node_link = old_tail_node_refcell;
    //             // new_tail_node_link.borrow_mut().next = None;
    //             self.tail = new_tail_node_link; // 销毁一个rc
    //             Some(Rc::try_unwrap(old_tail_node).unwrap().into_inner().element)
    //         },
    //         None=>{
    //             self.tail = None;
    //             None
    //         },
    //     }
   
    // }

    // 返回第一个节点的引用
    pub fn peek_front(&self) -> Option<Ref<T>> {
        // as_ref 是因为不能move
        self.head.as_ref().map(|node| {
            let n = node.borrow(); //借出一个不可变引用
                                   // 参考下面注释熟悉从RefCell中获取
            Ref::map(n, |_node:Node<T>| &_node.element)
        })
        /*
        fn borrow<'a>(&'a self) -> Ref<'a, T>
        fn borrow_mut<'a>(&'a self) -> RefMut<'a, T>
        /// # Examples
        ///
        /// ```
        /// use std::cell::{RefCell, Ref};
        ///
        /// let c = RefCell::new((5, 'b'));
        /// let b1: Ref<'_, (u32, char)> = c.borrow();
        /// let b2: Ref<'_, u32> = Ref::map(b1, |t| &t.0);
        /// assert_eq!(*b2, 5)
        /// ```
        */
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
    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.pop_front().is_none());
        list.push_front("this is 3");
        list.push_front("this is 2");
        list.push_front("this is 1");
        let n1 = list.peek_front().unwrap();
        let n2 = *n1;
        println!("peek list1 = {:?}", n2);

        let mut list2 = List::new();
        list2.push_front(3);
        list2.push_front(2);
        list2.push_front(1);
        let n1 = list2.peek_front().unwrap();
        let n2 = *n1; // 这里实现了copy
        println!("peek list2 = {:?}", n2);
    }
}
