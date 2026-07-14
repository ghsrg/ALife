use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RingBuffer<T> {
    capacity: usize,
    values: VecDeque<T>,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Result<Self, &'static str> {
        if capacity == 0 {
            return Err("capacity must be greater than zero");
        }
        Ok(Self {
            capacity,
            values: VecDeque::with_capacity(capacity),
        })
    }

    pub fn push(&mut self, value: T) {
        if self.values.len() == self.capacity {
            self.values.pop_front();
        }
        self.values.push_back(value);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn newest(&self) -> Option<&T> {
        self.values.back()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }
}
