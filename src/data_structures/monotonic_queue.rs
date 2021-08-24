use std::collections::VecDeque;

pub struct MonotonicQueue<T> {
    deq: VecDeque<T>,
}

impl<T> MonotonicQueue<T> {
    pub fn new() -> MonotonicQueue<T> {
        MonotonicQueue {
            deq: VecDeque::new(),
        }
    }

    pub fn push_by<F>(&mut self, item: T, is_less: F)
    where
        F: Fn(&T, &T) -> bool,
    {
        while let Some(existing_item) = self.deq.back() {
            if is_less(existing_item, &item) {
                self.deq.pop_back();
            } else {
                break;
            }
        }
        self.deq.push_back(item);
    }

    pub fn peek(&self) -> Option<&T> {
        self.deq.front()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.deq.pop_front()
    }
}
