use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

/// Linked List with generic type
///
/// ```
/// use algorithms::data_structures::LinkedList;
/// let mut list = LinkedList::<String>::new();
/// list.insert_last("hello".to_string());
/// if let Some(value) = list.get_first() {
///     assert_eq!(value, "hello")
/// }
/// ```
#[derive(Clone)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    pub length: u64,
}

impl<T> Node<T> {
    // create a new node
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

impl<T: Clone + Debug> LinkedList<T> {
    // create an empty linked list
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }
    // append a new node to the start of a list
    pub fn insert_first(&mut self, value: T) {
        let node = Node::new(value);

        match self.head.take() {
            None => {
                self.tail = Some(node.clone());
            }
            Some(old_head) => {
                node.borrow_mut().next = Some(old_head.clone());
            }
        }
        self.length += 1;
        self.head = Some(node);
    }

    // append a new node to the end of a list
    pub fn insert_last(&mut self, value: T) {
        let node = Node::new(value);

        match self.tail.take() {
            None => {
                self.head = Some(node.clone());
            }
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(node.clone());
            }
        }
        self.length += 1;
        self.tail = Some(node);
    }

    pub fn get_first(&self) -> Option<T> {
        self.get(0)
    }

    pub fn get(&self, index: u64) -> Option<T> {
        Self::get_ith(&self.head, index)
    }

    fn get_ith(node: &Link<T>, index: u64) -> Option<T> {
        match node {
            None => None,
            Some(n) => {
                let node_ref = n.borrow();
                match index {
                    0 => Some(node_ref.value.clone()),
                    _ => Self::get_ith(&node_ref.next, index - 1),
                }
            }
        }
    }

    pub fn remove_first(&mut self) -> Option<T> {
        if let Some(old_head) = self.head.take() {
            let old_head = old_head.borrow();
            match &old_head.next {
                None => self.tail = None,
                Some(next_ptr) => self.head = Some(next_ptr.clone()),
            }
            self.length -= 1;
            return Some(old_head.value.clone());
        }
        None
    }

    pub fn remove_ith(&mut self, index: u64) -> Option<T> {
        if index == 0 {
            return self.remove_first();
        }

        let mut current: Link<T> = self.head.clone();
        for _ in 0..index - 1 {
            current = match current {
                None => None,
                Some(x) => x.borrow().next.clone(),
            };
        }

        match &current {
            None => None,
            Some(prev) => {
                let mut prev_borrow = prev.borrow_mut();
                match prev_borrow.next.clone() {
                    None => None,
                    Some(next) => {
                        prev_borrow.next = next.borrow().next.clone();
                        self.length -= 1;
                        Some(next.borrow().value.clone())
                    }
                }
            }
        }
    }
}

pub struct ListIterator<T> {
    current: Link<T>,
}

impl<T: Clone> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head.clone())
    }
}

impl<T> ListIterator<T> {
    fn new(start_at: Link<T>) -> ListIterator<T> {
        ListIterator { current: start_at }
    }
}

impl<T: Clone> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            None => None,
            Some(current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            }
        };
        result
    }
}

#[cfg(test)]
mod tests {

    use super::{LinkedList, ListIterator};

    #[test]
    fn insert_first_ints() {
        let mut list = LinkedList::<i32>::new();
        list.insert_first(1);
        list.insert_last(2);

        match list.get_first() {
            None => panic!("Expect to have value at head"),
            Some(value) => assert_eq!(value, 1),
        }
    }

    #[test]
    fn insert_last_ints() {
        let mut list = LinkedList::<i32>::new();
        list.insert_last(1);

        match list.get_first() {
            None => panic!("Expect to have value at head"),
            Some(value) => assert_eq!(value, 1),
        }
    }

    #[test]
    fn insert_multi_operations() {
        let mut list = LinkedList::<i32>::new();
        list.insert_first(1); // 1
        list.insert_last(2); // 1 2
        list.insert_first(3); // 3 1 2
        list.insert_last(4); // 3 1 2 4
        list.insert_first(5); // 5 3 1 2 4

        assert_eq!(list.length, 5);

        match list.get_first() {
            None => panic!("Expect to have value"),
            Some(value) => assert_eq!(value, 5),
        }

        match list.get(1) {
            None => panic!("Expect to have value"),
            Some(value) => assert_eq!(value, 3),
        }

        match list.get(2) {
            None => panic!("Expect to have value"),
            Some(value) => assert_eq!(value, 1),
        }

        match list.get(3) {
            None => panic!("Expect to have value"),
            Some(value) => assert_eq!(value, 2),
        }

        match list.get(4) {
            None => panic!("Expect to have value"),
            Some(value) => assert_eq!(value, 4),
        }

        list.remove_first();
        match list.get(0) {
            None => panic!("Expect to have value"),
            Some(value) => assert_eq!(value, 3),
        }
        list.remove_first();
        match list.get(0) {
            None => panic!("Expect to have value"),
            Some(value) => assert_eq!(value, 1),
        }
        list.remove_first();
        match list.get(0) {
            None => panic!("Expect to have value"),
            Some(value) => assert_eq!(value, 2),
        }
        list.remove_first();
        match list.get(0) {
            None => panic!("Expect to have value"),
            Some(value) => assert_eq!(value, 4),
        }
        list.remove_first();
        if let Some(value) = list.get(0) {
            panic!("Do not expect to have value {value}")
        }
        assert_eq!(list.length, 0);
    }

    #[test]
    fn insert_last_strings() {
        let mut list = LinkedList::<String>::new();
        list.insert_last("hello".to_string());

        match list.get_first() {
            None => panic!("Expect to have value at head"),
            Some(value) => assert_eq!(value, "hello"),
        }
    }

    #[test]
    fn remove_ith() {
        let mut list = LinkedList::<i32>::new();
        list.insert_last(1);
        list.insert_last(2);
        list.insert_last(3);
        list.insert_last(4); // 1 2 3 4

        let r = list.remove_ith(2); // 1 2 4
        assert_eq!(r, Some(3));

        match list.get(2) {
            None => panic!("Expect to have value"),
            Some(value) => assert_eq!(value, 4),
        }

        let r = list.remove_ith(1); // 1 4
        assert_eq!(r, Some(2));

        match list.get(1) {
            None => panic!("Expect to have value"),
            Some(value) => assert_eq!(value, 4),
        }

        let r = list.remove_ith(1); // 1
        assert_eq!(r, Some(4));
        if let Some(value) = list.get(1) {
            panic!("Do not expect to have value {value}")
        }

        let r = list.remove_ith(0); // empty
        assert_eq!(r, Some(1));
        assert_eq!(list.length, 0);

        if let Some(value) = list.get(0) {
            panic!("Do not expect to have value {value}")
        }
    }

    #[test]
    fn into_iterator() {
        let mut list = LinkedList::<String>::new();
        list.insert_last("hello".to_string());
        list.insert_last("world".to_string());

        let values: Vec<String> = ListIterator::new(list.head.clone()).collect();
        assert_eq!(values, vec!["hello", "world"]);

        for v in list {
            assert!(v == "hello" || v == "world");
        }
    }
}
