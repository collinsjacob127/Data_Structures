/*
Jacob Collins
Silly Double Singly-Linked List
First Silly Data Structure :Sunglasses:
Based on tutorial from: https://rust-unofficial.github.io/too-many-lists/infinity-double-single.html
August 3, 2022
*/
// This is a code library and all of it is useful but only used in testing
#![allow(dead_code)]

struct List<T> {
    left: Stack<T>,
    right: Stack<T>,
}

pub struct Stack<T> {
    head: Link<T>,
    size: usize,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { left: Stack::new(), right: Stack::new() }
    }

    pub fn push_left(&mut self, elem: T) { self.left.push(elem) }
    pub fn push_right(&mut self, elem: T) { self.right.push(elem) }
    pub fn pop_left(&mut self) -> Option<T> { self.left.pop() }
    pub fn pop_right(&mut self) -> Option<T> { self.right.pop() }
    pub fn peek_left(&self) -> Option<&T> { self.left.peek() }
    pub fn peek_right(&self) -> Option<&T> { self.right.peek() }
    pub fn peek_left_mut(&mut self) -> Option<&mut T> { self.left.peek_mut() }
    pub fn peek_right_mut(&mut self) -> Option<&mut T> { self.right.peek_mut() }
    pub fn size_left(&mut self) -> usize { self.left.size() }
    pub fn size_right(&mut self) -> usize { self.right.size() }
    pub fn size(&mut self) -> usize { self.left.size() + self.right.size() }

    pub fn go_left(&mut self) -> bool {
        self.left.pop_node().map(|node| {
            self.right.push_node(node);
        }).is_some()
    }

    pub fn go_right(&mut self) -> bool {
        self.right.pop_node().map(|node| {
            self.right.push_node(node);
        }).is_some()
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None, size: 0 }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: None,
        });

        self.push_node(new_node);
    }

    fn push_node(&mut self, mut node: Box<Node<T>>) {
        self.size += 1;
        node.next = self.head.take();
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().take().map(|node| node.elem)
    }

    fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        if self.head.is_some() { self.size -= 1 };
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn size(&self) -> usize {
        self.size.clone()
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
       self.0.pop_right()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_left()
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
    next_back: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.right.head.as_deref_mut(),
            next_back: self.left.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next_back.take().map(|node| {
            self.next_back = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn walk_aboot() {
        let mut list = List::new();             // [_]

        list.push_left(0);                      // [0,_]
        list.push_right(1);                     // [0, _, 1]
        assert_eq!(list.peek_left(), Some(&0));
        assert_eq!(list.peek_right(), Some(&1));

        list.push_left(2);                      // [0, 2, _, 1]
        list.push_left(3);                      // [0, 2, 3, _, 1]
        list.push_right(4);                     // [0, 2, 3, _, 4, 1]

        while list.go_left() {}                 // [_, 0, 2, 3, 4, 1]

        assert_eq!(list.pop_left(), None);
        assert_eq!(list.pop_right(), Some(0));  // [_, 2, 3, 4, 1]
        assert_eq!(list.pop_right(), Some(2));  // [_, 3, 4, 1]

        list.push_left(5);                      // [5, _, 3, 4, 1]
        assert_eq!(list.pop_right(), Some(3));  // [5, _, 4, 1]
        assert_eq!(list.pop_left(), Some(5));   // [_, 4, 1]
        assert_eq!(list.pop_right(), Some(4));  // [_, 1]
        assert_eq!(list.pop_right(), Some(1));  // [_]

        assert_eq!(list.pop_right(), None);
        assert_eq!(list.pop_left(), None);

    }

    #[test]
    fn size() {
        let mut list = List::new();             // [_]

        assert_eq!(list.size_left(), 0);
        assert_eq!(list.size_right(), 0);
        assert_eq!(list.size(), 0);

        list.push_left(0);                      // [0,_]
        list.push_right(1);                     // [0, _, 1]

        // Works after adding first elems
        assert_eq!(list.size_left(), 1);
        assert_eq!(list.size_right(), 1);
        assert_eq!(list.size(), 2);

        list.push_left(2);                      // [0, 2, _, 1]
        list.push_left(3);                      // [0, 2, 3, _, 1]
        list.push_right(4);                     // [0, 2, 3, _, 4, 1]

        // Works left and right after removal
        assert_eq!(list.size_left(), 3);
        assert_eq!(list.size_right(), 2);
        assert_eq!(list.size(), 5);

        while list.go_left() {}                 // [_, 0, 2, 3, 4, 1]
        list.pop_left();
        list.pop_right();                       // [_, 2, 3, 4, 1]
        list.pop_right();                       // [_, 3, 4, 1]
        list.push_left(5);                      // [5, _, 3, 4, 1]
        list.pop_right();                             // [5, _, 4, 1]
        list.pop_left();                              // [_, 4, 1]
        list.pop_right();                             // [_, 1]
        list.pop_right();                             // [_]
        
        // Works at exhaustion
        assert_eq!(list.size_left(), 0);
        assert_eq!(list.size_right(), 0);
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_left(1); list.push_left(2);   // [1, 2, _]
        list.push_right(4); list.push_right(3); // [1, 2, _, 3, 4]

        let mut iter = list.into_iter(); 
        assert_eq!(iter.next(),  Some(3));
        assert_eq!(iter.next(),  Some(4));
        assert_eq!(iter.next_back(),  Some(2));
        assert_eq!(iter.next_back(),  Some(1));
        assert_eq!(iter.next_back(),  None);
        assert_eq!(iter.next(),  None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push_left(1); list.push_left(2);   // [1, 2, _]
        list.push_right(4); list.push_right(3); // [1, 2, _, 3, 4]

        let mut iter = list.iter_mut(); 
        assert_eq!(iter.next(),  Some(&mut 3));
        assert_eq!(iter.next(),  Some(&mut 4));
        assert_eq!(iter.next_back(),  Some(&mut 2));
        assert_eq!(iter.next_back(),  Some(&mut 1));
        assert_eq!(iter.next_back(),  None);
        assert_eq!(iter.next(),  None);
    }
}
