use std::collections::VecDeque;

struct SearchQueue<T> {
    pub items: VecDeque<T>,
}

impl<T> SearchQueue<T> {
    pub fn new() -> SearchQueue<T> {
        SearchQueue {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, v: T) {
        self.items.push_back(v)
    }

    pub fn dequeue(&mut self) -> T {
        self.items
            .pop_front()
            .expect("Cannot dequeue from empty queue")
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
}

struct SearchStack<T> {
    pub items: VecDeque<T>,
}

impl<T> SearchStack<T> {
    pub fn new() -> SearchStack<T> {
        SearchStack {
            items: VecDeque::new(),
        }
    }

    pub fn push(&mut self, v: T) {
        self.items.push_back(v)
    }

    pub fn pop(&mut self) -> T {
        self.items
            .pop_back()
            .expect("Cannot dequeue from empty stash")
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_searchq_operations() {
        let mut q = SearchQueue::<i32>::new();
        assert_eq!(q.is_empty(), true);
        q.enqueue(5);
        assert_eq!(q.is_empty(), false);
        q.enqueue(2);
        assert_eq!(q.dequeue(), 5);
    }

    #[test]
    fn test_searchstack_operations() {
        let mut stack = SearchStack::<i32>::new();
        assert_eq!(stack.is_empty(), true);
        stack.push(5);
        assert_eq!(stack.is_empty(), false);
        stack.push(2);
        assert_eq!(stack.pop(), 2);
    }
}
