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
// Mutex<T> instead of RefCell<T> if you want shared mutability in multithreaded situation
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

/*
Maxims for List struct:
Every node will always have two references going towards it.
Each node in the middle of the list will be pointed to by its predecessor and successor.
Any nodes at the ends of the list will be pointed to by the list itself.
*/
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }
    pub fn push_front(&mut self, elem: T) {
        // new node needs two new links, everything else should have same number of links after.
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        // Take the old head, ensuring it's -2 (loses both pointers)
        self.head.take().map(|old_head| {           // -1 old
            match old_head.borrow_mut().next.take() { 
                Some(new_head) => {                 // -1 new
                    // There IS another element after the old head
                    new_head.borrow_mut().prev.take();                    // -1 old
                    self.head = Some(new_head);                           // +1 new
                    // Total: -2 old, +0 new
                }
                None => {
                    // emptying list
                    self.tail.take();                                     // -1 old
                    //total: -2 old, (no new)
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        }) 
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}