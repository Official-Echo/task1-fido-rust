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

        let mut curr = head;
        let mut len = 1usize;
        let mut res = head;

        while let Some(next) = curr.next.as_deref() {
            curr = next;
            len += 1;

            if len > 2 && len % 3 != 1 {
                res = res.next.as_deref()?;
            }
        }

        if len <= 1 {
            return None;
        }

        Some(res)
    }
}

#[cfg(test)]
mod tests;
