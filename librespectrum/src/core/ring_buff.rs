
pub struct RingBuff<T, const N: usize> {
    items: [Option<T>; N],
    head_idx: usize,
    tail_idx: usize,
    len: usize,
}

impl<T: Copy, const N: usize> RingBuff<T, N> {

    /// Get next index value
    fn next(index: usize) -> usize {
        if index == N - 1 { 0 } else { index + 1 }
    }

    /// Get previous index value
    fn prev(index: usize) -> usize {
        if index == 0 { N - 1 } else { index - 1 }
    }

    pub fn new() -> Self {
        assert!(N > 0);
        Self { items: [None; N], head_idx: 0, tail_idx: 0, len: 0 }
    }

    /// Get buffer size (capacity)
    pub fn size(&self) -> usize {
        N
    }

    /// Get current buffer lenght
    pub fn len(&self) -> usize {
        self.len
    }

    /// Get the newest item (the buffer's head), if any.
    pub fn head(&self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.items[Self::prev(self.head_idx)]
        }
    }

    /// Get the oldest item (the buffer's tail), if any.
    pub fn tail(&self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.items[self.tail_idx]
        }
    }

    /// Pushes an item to the head of the ring buffer, evicting the tail item once full
    pub fn push(&mut self, item: T) -> &mut Self {
        self.items[self.head_idx] = Some(item);
        self.head_idx = Self::next(self.head_idx);
        if self.len == N {
            self.tail_idx = Self::next(self.tail_idx);
        } else {
            self.len += 1;
        }
        self
    }

    /// Pushes an item to the tail of the ring buffer, evicting the head item once full
    pub fn push_back(&mut self, item: T) -> &mut Self {
        self.tail_idx = Self::prev(self.tail_idx);
        self.items[self.tail_idx] = Some(item);
        if self.len == N {
            self.head_idx = Self::prev(self.head_idx);
        } else {
            self.len += 1;
        }
        self
    }

    /// Removes and returns the newest item (the buffer's head), if any.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.head_idx = Self::prev(self.head_idx);
        self.len -= 1;
        self.items[self.head_idx].take()
    }

    /// Removes and returns the oldest item (the buffer's tail), if any.
    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let item = self.items[self.tail_idx].take();
        self.tail_idx = Self::next(self.tail_idx);
        self.len -= 1;
        item
    }

    pub fn iter_to_tail<'a>(&'a self) -> RingBuffIterator<'a, T, N> {
        RingBuffIterator {
            items: &self.items,
            index: Self::prev(self.head_idx),
            count: self.len(),
            step: Self::prev,
        }
    }

    pub fn iter_to_head<'a>(&'a self) -> RingBuffIterator<'a, T, N> {
        RingBuffIterator {
            items: &self.items,
            index: self.tail_idx,
            count: self.len(),
            step: Self::next,
        }
    }

}

pub struct RingBuffIterator<'a, T, const N: usize> {
    items: &'a [Option<T>; N],
    index: usize,
    count: usize,
    step: fn(usize) -> usize,
}

impl<T: Copy, const N: usize> Iterator for RingBuffIterator<'_, T, N> {

    type Item = T;

    fn next(&mut self) -> Option<T> {

        let next = if self.count > 0 { self.items[self.index] } else { None };
        if next.is_some() {
            self.index = (self.step)(self.index);
            self.count -= 1;
        }
        next

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_length() {
        let mut buff = RingBuff::<u8, 3>::new();
        buff.push(1).push(2);
        assert_eq!(buff.len(), 2);
    }

    #[test]
    fn allows_to_push_back_items() {
        let mut buff = RingBuff::<u8, 3>::new();
        buff.push(2).push(3);
        assert_eq!(buff.push_back(1).len(), 3);
        assert_eq!(buff.iter_to_head().collect::<Vec<u8>>(), vec![1, 2, 3]);
    }

    #[test]
    fn allows_to_iterate_from_head_to_tail() {
        let mut buff = RingBuff::<u8, 3>::new();
        buff.push(1).push(2).push(3);
        assert_eq!(buff.iter_to_tail().collect::<Vec<u8>>(), vec![3, 2, 1]);
    }

    #[test]
    fn allows_to_iterate_from_tail_to_head() {
        let mut buff = RingBuff::<u8, 3>::new();
        buff.push(1).push(2).push(3);
        assert_eq!(buff.iter_to_head().collect::<Vec<u8>>(), vec![1, 2, 3]);
    }

    #[test]
    fn push_evicts_tail_when_full() {
        let mut buff = RingBuff::<u8, 3>::new();
        buff.push(1).push(2);
        assert_eq!(buff.push(3).push(4).len(), 3);
        assert_eq!(buff.iter_to_tail().collect::<Vec<u8>>(), vec![4, 3, 2]);
    }

    #[test]
    fn push_back_evicts_head_when_full() {
        let mut buff = RingBuff::<u8, 3>::new();
        buff.push(1).push(2).push(3);
        assert_eq!(buff.push_back(0).len(), 3);
        assert_eq!(buff.iter_to_head().collect::<Vec<u8>>(), vec![0, 1, 2]);
    }

    #[test]
    fn head_and_tail_none_when_empty() {
        let buff = RingBuff::<u8, 3>::new();
        assert_eq!(buff.head(), None);
        assert_eq!(buff.tail(), None);
    }

    #[test]
    fn head_and_tail_after_push() {
        let mut buff = RingBuff::<u8, 3>::new();
        buff.push(1);
        assert_eq!(buff.head(), Some(1), "head is the newest item");
        assert_eq!(buff.tail(), Some(1), "tail is the only item");

        buff.push(2).push(3);
        assert_eq!(buff.head(), Some(3));
        assert_eq!(buff.tail(), Some(1));
    }

    #[test]
    fn head_and_tail_after_push_back() {
        let mut buff = RingBuff::<u8, 3>::new();
        buff.push(2).push(3);
        buff.push_back(1);
        assert_eq!(buff.head(), Some(3));
        assert_eq!(buff.tail(), Some(1));
    }

    #[test]
    fn head_evicts_tail_when_full() {
        let mut buff = RingBuff::<u8, 3>::new();
        buff.push(1).push(2).push(3).push(4);
        assert_eq!(buff.head(), Some(4));
        assert_eq!(buff.tail(), Some(2), "oldest item evicted when full");
    }

    #[test]
    fn tail_evicts_head_when_full() {
        let mut buff = RingBuff::<u8, 3>::new();
        buff.push(1).push(2).push(3);
        buff.push_back(0);
        assert_eq!(buff.head(), Some(2), "newest item evicted by push_back when full");
        assert_eq!(buff.tail(), Some(0));
    }

    #[test]
    fn pop_none_when_empty() {
        let mut buff = RingBuff::<u8, 3>::new();
        assert_eq!(buff.pop(), None);
        assert_eq!(buff.pop_back(), None);
    }

    #[test]
    fn pop_returns_newest_item_last_in_first_out() {
        let mut buff = RingBuff::<u8, 3>::new();
        buff.push(1).push(2).push(3);

        assert_eq!(buff.pop(), Some(3), "pop removes the newest item");
        assert_eq!(buff.len(), 2);
        assert_eq!(buff.pop(), Some(2));
        assert_eq!(buff.pop(), Some(1));
        assert_eq!(buff.pop(), None, "empty after removing all items");
    }

    #[test]
    fn pop_back_returns_oldest_item_first_in_first_out() {
        let mut buff = RingBuff::<u8, 3>::new();
        buff.push(1).push(2).push(3);

        assert_eq!(buff.pop_back(), Some(1), "pop_back removes the oldest item");
        assert_eq!(buff.len(), 2);
        assert_eq!(buff.pop_back(), Some(2));
        assert_eq!(buff.pop_back(), Some(3));
        assert_eq!(buff.pop_back(), None);
    }

}
