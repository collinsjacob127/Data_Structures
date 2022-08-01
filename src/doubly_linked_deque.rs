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
use std::cell::{Ref, RefCell, RefMut};

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
        // New node needs +2 links, everything else should have same # refs after
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
    pub fn push_back(&mut self, elem: T) {
        // New node needs +2 links, everything else should have same # refs after
        let new_tail = Node::new(elem);
        match self.tail.take() {                                          // -1 old
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());      // +1 new
                new_tail.borrow_mut().prev = Some(old_tail);              // +1 old
                self.tail = Some(new_tail);                               // +1 new
            }                                                             // +0 old, +2 new
            None => {
                self.head = Some(new_tail.clone());                       // +1 new
                self.tail = Some(new_tail);                               // +1 new
            }                                                             // No old, +2 new
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
    pub fn pop_back(&mut self) -> Option<T> {
        // Lose two references to tail, all else keeps # refs
        self.tail.take().map(|old_tail| {     // -1 old
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {                 // -1 new
                    new_tail.borrow_mut().next.take();                    // -1 old
                    self.tail = Some(new_tail);                           // +1 new
                }
                None => {
                    // emptying list
                    self.head.take();                                     // -1 old
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    } 
    pub fn peek_front(&self) -> Option<Ref<T>> {
        // Must reference, dereference, and unwrap (&*foo_list.peek_front().unwrap() -> Option<&T>)
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        }) 
    }
    pub fn peek_back(&self) -> Option<Ref<T>> {
    // Must reference, dereference, and unwrap (&*foo_list.peek_front().unwrap() -> Option<&T>)
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(),|node| &node.elem)
        })
    }
    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        // Must reference, dereference, and unwrap (&*foo_list.peek_front().unwrap() -> Option<&T>)
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(),|node| &mut node.elem)
        })
    }
    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        // Must reference, dereference, and unwrap (&*foo_list.peek_front().unwrap() -> Option<&T>)
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(),|node| &mut node.elem)
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        // Works while empty
        assert_eq!(list.pop_front(), None);
        // Populate
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        // Depopulate
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        // Emptied 
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        // --- Back ---
        // Works while empty
        assert_eq!(list.pop_back(), None);
        // Populate
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        // Depopulate
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        // Checking for corruption
        list.push_back(4);
        list.push_back(5);
        // Checking normal removal again
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));
        // Emptied 
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }
    #[test]
    fn peek() {
        let mut list = List::new();
        // Behaves correctly while empty
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());
        // Adding to the list
        list.push_front(1); list.push_front(2); list.push_front(3);
        // Behaves correctly with elements
        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }
}
