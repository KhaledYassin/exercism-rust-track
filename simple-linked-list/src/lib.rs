use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        let new_node = Some(Box::new(Node {
            data: element,
            next: std::mem::replace(&mut self.head, None),
        }));

        self.head = new_node;

        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {

        let optional_node = self.head.take()?;
        let node = optional_node;

        if self.length > 0 {
            self.length -= 1;
        }

        self.head = node.next;

        Some(node.data)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut reversed_list = SimpleLinkedList::new();
        while let Some(x) = self.pop() {
            reversed_list.push(x);
        }
        reversed_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iterator = iter.into_iter();
        let mut list = SimpleLinkedList {
            head: None,
            length: 0,
        };

        while let Some(data) = iterator.next() {
            list.push(data)
        }

        list
    }
}

pub struct SimpleLinkedListIterator<T> {
    list: SimpleLinkedList<T>,
}

impl<T> Iterator for SimpleLinkedListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;

    type IntoIter = SimpleLinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleLinkedListIterator { list: self.rev() }
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        linked_list.into_iter().collect()
    }
}
