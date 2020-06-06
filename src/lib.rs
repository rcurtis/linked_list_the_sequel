pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            len: 0
        }
    }

    pub fn push(&mut self, i: T) {
        let current = self.head.take();
        self.head = Some(Box::new(Node {
            next: current,
            val: i
        }));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.len -= 1;
            self.head = node.next;
            return Some(node.val);
        }
        None
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            return &node.val;
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            return &mut node.val;
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(i) = self.pop() {
        }
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl<T> Into<Vec<T>> for LinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = vec![];
        while let Some(i) = self.pop() {
            vec.push(i);
        }
        vec
    }
}

#[test]
fn create() {
    let l : LinkedList<i32> = LinkedList::new();
}

#[test]
fn push_increments() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    assert_eq!(2, list.len)
}

#[test]
fn pop_returns_item() {
    let mut list = LinkedList::new();
    list.push(1);
    assert_eq!(Some(1), list.pop())
}

#[test]
fn pop_returns_LIFO() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    assert_eq!(Some(2), list.pop())
}

#[test]
pub fn iter() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let vec: Vec<i32> = list.into_iter().collect();
    assert_eq!(vec![3,2,1], vec)
}

#[test]
fn peek() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    assert_eq!(2, *list.peek().unwrap())
}

#[test]
fn peek_mut() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    assert_eq!(Some(&mut 2), list.peek_mut())
}

#[test]
fn peek_mut_mutate() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.peek_mut().map(|val| {
        *val = 3
    });
    assert_eq!(Some(&mut 3), list.peek_mut())
}

#[test]
fn to_vec() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    let vec: Vec<i32> = list.into();
    assert_eq!(vec![2, 1], vec);
}