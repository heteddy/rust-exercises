use std::fmt::{self, Display};

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}


type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, value: T) {
        let node = Node {
            element: value,
            // Option<Box>没有实现copy，因此只能是move，但是self是借用，因此不能move head
            next: self.head.take(), // self是可变引用（或者引用），引用不能move，因此需要从中偷出来值
        };
        self.head = Some(Box::new(node));
    }
    pub fn pop(&mut self) -> Option<T> {
        // map 内的闭包是 FnOnce(T) -> U
        self.head.take().map(|node| {
            //
            self.head = node.next;
            node.element
        })
    }
    // 获取第一个node的值
    pub fn peek(&self) -> Option<&T> {
        // option 的as_ref获取option中T的引用
        self.head.as_ref().map(|node| &node.element)
    }
    // 获取第一个node的mutable值
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // &mut value
        self.head.as_mut().map(|node| &mut node.element)
    }
    // 消耗这个结构
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { l: self }
    }
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            // 这里注意,等效于下面的方法
            // next: self.head.as_deref()
            next: self.head.as_ref().map(|node| {
                // let _node = *node;
                // let __node = *_node;
                // &__node
                // node &Box<Node<T>>
                // *node Box<Node<T>>
                // **node Node<T>
                // &Node<T> // 这样会报错，期望Option<&Node<T>> 实际 Option<&Box<Node<T>>>
                &(**node)
            }),
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        // 取出下一个node
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

/**
* IntoIter 类型迭代器的 next 方法会拿走被迭代值的所有权，IterMut 是可变借用，
Iter 是不可变借用。事实上，类似的命名规则在 Rust 中随处可见，当熟悉后，以后见到类似的命名大家就可以迅速的理解其对值的运用方式。
IntoIter - T
IterMut - &mut T
Iter - &T
*/
pub struct IntoIter<T> {
    l: List<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.l.pop()
    }
}

/// 持有一个当前节点的指针，当生成一个值后，该指针将指向下一个节点。
/// Iter 是不可变借用。
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // 更新next
        /*
        Critically to why this code was working, shared references are also Copy! Because & is copy,
        Option<&> is also Copy. So when we did self.next.map it was fine because the Option was
        just copied.
        */
        self.next.map(|node| {
            // Option<&'a Node<T>>
            self.next = node.next.as_deref(); // node.next
            &node.element
        })
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
// impl<'a, T> Display for IterMut<'a, T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({:?})", self.next)
//     }
// }

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        /*
         Now we can't do that, because &mut isn't Copy (if you copied an &mut, you'd
        have two &mut's to the same location in memory, which is forbidden). Instead, we should
        properly take the Option to get it.
        */
        // map会导致next被move，但是self是引用因此不能
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter_mut();

        assert_eq!(iter.next(), Some(&mut 3));

        if let Some(v) = iter.next() {
            *v = 20
        }

        let mut iter = list.iter();
        println!("{:?}", iter.next());
        println!("{:?}", iter.next());
        println!("{:?}", iter.next());
        // assert_eq!(iter.next(), Some(&mut 20));
    }
}
