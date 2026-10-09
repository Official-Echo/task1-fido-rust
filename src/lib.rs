#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Self>>,
}

impl<T> Node<T> {
    pub const fn new(data: T) -> Self {
        Self { data, next: None }
    }

    pub fn with_next(data: T, next: Self) -> Self {
        Self {
            data,
            next: Some(Box::new(next)),
        }
    }
}

pub struct TwoThirdsOfLinkedList;

impl TwoThirdsOfLinkedList {
    pub fn get_two_thirds_node<T>(head: &Node<T>) -> Option<&Node<T>> {
        let mut len = 1usize;

        let mut tail = head;
        while let Some(next) = tail.next.as_deref() {
            len += 1;
            tail = next;
        }

        if len <= 1 {
            return None;
        }

        let res_index = (2 * len) / 3 - 1;
        let mut current = head;

        for _ in 0..res_index {
            current = current.next.as_deref()?;
        }

        Some(current)
    }
}