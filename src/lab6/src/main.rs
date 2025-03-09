use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
            prev: None,
        }
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    pub fn len(&self) -> usize {
        self.size
    }
    
    pub fn push_back(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node.clone());
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
        }
        self.size += 1;
    }
    
    pub fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node.clone());
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
        }
        self.size += 1;
    }
    
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            }
            self.size -= 1;
            Rc::try_unwrap(old_tail)
                .ok()
                .expect("Щось пішло не так з Rc")
                .into_inner()
                .data
        })
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            self.size -= 1;
            Rc::try_unwrap(old_head)
                .ok()
                .expect("error in Rc")
                .into_inner()
                .data
        })
    }
    
    pub fn iter(&self) -> ListIterator<'_, T> {
        ListIterator::new(self.head.as_ref().map(|head| Rc::clone(head)))
    }
}

pub struct ListIterator<'a, T> {
    current: Option<Rc<RefCell<Node<T>>>>,
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T> ListIterator<'a, T> {
    fn new(head: Option<Rc<RefCell<Node<T>>>>) -> Self {
        ListIterator {
            current: head,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node_rc| {
            let node = node_rc.borrow();
            self.current = node.next.as_ref().map(|next_rc| Rc::clone(next_rc));
            Rc::clone(&node_rc)
        })
    }
}


fn main() {
    let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_front(0);
    list.push_front(-1);

    println!("List size: {}", list.len());

    println!("List elements:");
    for node in list.iter() {
        let value = node.borrow().data;
        println!("{}", value);
    }

    println!("Popped from back: {:?}", list.pop_back());
    println!("Popped from front: {:?}", list.pop_front());

    println!("List after popping:");
    for node in list.iter() {
        let value = node.borrow().data;
        println!("{}", value);
    }

    while let Some(item) = list.pop_front() {
        println!("Popped: {}", item);
    }

    println!("List is empty: {}", list.is_empty());
}