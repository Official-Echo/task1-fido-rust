#[derive(Debug, PartialEq, PartialOrd)]
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

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct TwoThirdsOfLinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T> TwoThirdsOfLinkedList<T> {
    pub const fn new() -> Self {
        Self { head: None }
    }

    pub fn prepend(&mut self, data: T) -> &mut Self {
        self.head = Some(Box::new(Node {
            data,
            next: self.head.take(),
        }));
        self
    }

    pub fn prepend_iter(&mut self, iter: impl IntoIterator<Item = T>) -> &mut Self {
        for item in iter {
            self.prepend(item);
        }
        self
    }

    pub fn get_two_thirds_node(&self) -> Option<&Node<T>> {
        let head = self.head.as_deref()?;
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

#[cfg(test)]
mod tests;
