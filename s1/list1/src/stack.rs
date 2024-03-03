/*！
如果要判断head是否==Link::Empty;就需要定义PartialEq
*/
//! 如何定义Drop呢, 默认是按照递归的方式Drop，但是递归的方式由于层层压栈的原因可能导致栈溢出
/// 定义一个
#[derive(Clone, Debug)]
pub struct Stack {
    pub head: Link,
}
/// 如果要判断head是否==Link::Empty;就需要定义PartialEq
#[derive(Clone, Debug, PartialEq)]
pub enum Link {
    Empty,
    More(Box<Node>),
}
impl Link {}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    element: String,
    next: Link,
}
impl Node {}

impl Stack {
    pub fn new() -> Self {
        Stack { head: Link::Empty }
    }

    pub fn is_empty(&self) -> bool {
        match &self.head {
            Link::Empty => true,
            _ => false,
        }
    }

    pub fn push(&mut self, element: String) {
        // 按照栈的方式实现，创建一个新的node，并把head指向新node，直接替换成
        let n = Node {
            element,
            next: std::mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(n));
    }
    pub fn pop(&mut self) -> Option<String> {
        // 这里是不可变借用
        // match &self.head {
        //     Link::Empty => {
        //         None
        //     }
        //     // 因为用的是head的引用，因此这里的n是引用
        //     Link::More(node) =>{
        //         self.head = node.next; // 这里不能赋值，需要一次性处理
        //         Some(node.element.clone())
        //     }
        // }
        // 这里返回的Link 会move
        match std::mem::replace(&mut self.head, Link::Empty) {
            // 去除第一个元素
            Link::Empty => None,
            Link::More(n) => {
                // 先把head值拿出来，把head替换成空，最后再把head指向n.next
                self.head = n.next; // n.next也被move了
                Some(n.element.clone())
            }
        }
    }
    
}

impl Drop for Stack {
    /// 默认的方式是递归的实现，可能导致溢出栈
    fn drop(&mut self) {
        // 从头指针开始，逐个被消耗掉并把next指针置空
        println!("Dropping with data `{:?}`!", self.head);
        
        let mut current_link = std::mem::replace(&mut self.head, Link::Empty);
        // list->A->B->C
        // current link 是head ；
        // A是current_link 取出值，current_link=a.next,a.next=Link::Empty，(两次调用a.next)
        while let Link::More(mut node) = current_link {
            // 实际的操作是
            // current_link = node.next;
            // node.next = Link::Empty;
            println!("drop {:?}", &node);
            current_link = std::mem::replace(&mut node.next,Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_list1() {
        let mut l = Stack::new();
        l.push("e1".to_owned());
        l.push("e2".to_owned());
        l.push("e3".to_owned());
        assert!(l.head != Link::Empty, "");
        assert_eq!(l.pop(), Some("e3".to_owned()));
        assert_eq!(l.pop(), Some("e2".to_owned()));
        assert_eq!(l.pop(), Some("e1".to_owned()));
    }
}
