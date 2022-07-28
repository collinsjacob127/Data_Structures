/*
Jacob Collins
A Bad but Safe Doubly Linked Deque
Fourth data structure in tutorial
Based on tutorial from: https://rust-unofficial.github.io/too-many-lists/index.html
Used to learn about interior mutability, and why doubly linked deques are bad.
July 27, 2022
*/
// use std::sync::Arc; <- Rc but with Atomics implemented, making it safe for multithreading
use std::rc::Rc;
use std::cell::RefCell;
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let list = List::new();

        // Check empty list behaves right
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));        
        assert_eq!(iter.next(), Some(&2));        
        assert_eq!(iter.next(), Some(&1));        
    }
}