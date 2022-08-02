/*
Jacob Collins
Fifth Data Structure in Tutorial
An OK Unsafe Queue
Reference: https://rust-unofficial.github.io/too-many-lists/fifth.html
*/
// Stack: pops off same end it gets pushed to: push/pop -> stack
// Queue: pops off opposite side it gets pushed to: push (tail) -> queue -> (head) pop

use std::mem;

pub struct List<'a, T> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>, //NEW! 
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<'a, T> List<'a, T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push(&'a mut self, elem: T) {
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
}