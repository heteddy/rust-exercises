/*
list1 -> A ---+
              |
              v
list2 ------> B -> C -> D
              ^
              |
list3 -> X ---+
*/

use std::rc::Rc;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    /// 实现头部追加，返回一个新的list
    pub fn prepend(&self, element: T) -> Self {
        let n = Node {
            element,
            next: self.head.clone(),
        };
        // 新list的head指向旧的list
        List {
            head: Some(Rc::new(n)),
        }
    }
    pub fn tail(&self) -> Self {
        List {
            // 使用map会导致多一个option, 因为and_then使用的是 FnOnce(T) -> Option<U>,
            // 主要是看闭包的返回值
            /*
            pub fn map<U, F>(self, f: F) -> Option<U>
                where
                    F: FnOnce(T) -> U,
            {
            match self {
                Some(x) => Some(f(x)),   // map这里会套上option
                None => None,
            }

            pub fn and_then<U, F>(self, f: F) -> Option<U>
                where
                    F: FnOnce(T) -> Option<U>,
            {
                match self {
                    Some(x) => f(x), //
                    None => None,
                }
            }

            */
            head: self.head.as_ref().and_then(|node| {
                node.next.clone() // Option<T>
            }),
        }
    }

    pub fn head(&self) -> Option<&T> {
        // todo 这里使用as_deref  as_ref 都行 因为Rc实现了Deref
        self.head.as_ref().map(|node| &node.element)
    }
    pub fn iter(&self) -> Iter<'_, T> {
        //
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for List<T> {
    // 列表本身有递归删除的问题，我们先把
    fn drop(&mut self) {
        // 指向下一个指针
        let mut current = self.head.take();
        while let Some(n) = current {
            // 这里try是保证Rc只有一个引用
            if let Ok(mut node) = Rc::try_unwrap(n) {
                current = node.next.take();
            }else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>, // iter 的next 定义的是一个node 的option
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // 通过map返回next的值，然后next更新到下一个
        self.next.map(|node| {
            // 更新next
            self.next = node.next.as_deref();
            // 返回当前值
            &node.element
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_iter() {
        let l = List::new();
        let l = l.prepend(3).prepend(2).prepend(1);
        
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
    }
}
