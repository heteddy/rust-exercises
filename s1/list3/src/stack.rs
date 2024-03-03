pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

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
            next: self.head.take(),
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
        self.head.as_mut().map(|node| {
            &mut node.element
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut next = self.head.take();
        while let Some(node) = next {
            next = node.next;
        }
    }
}
