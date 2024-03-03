#[derive(Debug, Clone, PartialEq)]
pub struct Stack {
    head: Link,
}

type Link = Option<Box<Node>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    element: String,
    next: Link,
}

impl Stack {
    pub fn new() -> Self {
        Stack { head: None }
    }
    pub fn is_empty(&self) ->bool {
        self.head == None
    }
    pub fn push(&mut self, element: String) {
        let node = Node {
            element,
            next: self.head.take(), // 取出head的值给next
        };
        self.head = Some(Box::new(node));
    }
    pub fn pop(&mut self) -> Option<String> {
        // 取出来一个node
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.element.clone())
            }
        }
    }
    // peek 返回链表头部元素的引用
    pub fn peek(&self) -> Option<&String> {
        match &self.head {
            None => None,
            Some(ref n) => {
                Some(&n.element)
            }
        }
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        println!("Dropping with data `{:?}`!", self.head);
        let mut next = self.head.take();
        while let Some(node) = next {
            println!("drop {:?}", &node);
            next = node.next
        }
    }
}