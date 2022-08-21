/*
Jacob Collins
Fifth Data Structure in Tutorial
An OK Unsafe Queue - DONT USE ANY OF THIS FOR ACTUAL STUFF
Reference: https://rust-unofficial.github.io/too-many-lists/fifth.html
*/
// Stack: pops off same end it gets pushed to: push/pop -> stack
// Queue: pops off opposite side it gets pushed to: push (tail) -> queue -> (head) pop

use std::ptr;

pub struct List<T> {
    head: Link<T>,
    // tail: Option<&'a mut Node<T>>, // BAD - Lifetime is tied to list so can't be replaced 
    tail: *mut Node<T>                // BAD - Raw pointers (*mut T == &unchecked mut T)
}                                  

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }

    pub fn push(mut self, elem: T) {
        let new_tail = Box::new(Node {
            elem: elem,
            // Tail is at the end of the list so the pointer for next is nulled out.
            next: None,
        });
        
        let new_tail = match self.tail.take() {
            Some(mut old_tail) => {
                // List was non-empty; Move new tail to end of list and point old tail to new
                old_tail.next = Some(new_tail);
                old_tail.next.as_deref_mut()
            }
            None => {
                // List was empty. New tail is both tail and head.
                self.head = Some(new_tail);
                self.head.as_deref_mut()
            }
        };

        self.tail = new_tail;
    }

    pub fn pop(mut self) -> Option<T> {
        // Grab lists current head
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            // If no next head, set tail none (one elem -> 0 elem)
            if self.head.is_none() {
                self.tail = None;
            }

            head.elem
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}