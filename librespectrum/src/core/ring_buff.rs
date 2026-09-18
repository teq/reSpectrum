
pub struct RingBuff<T, const N: usize> {
    items: [Option<T>; N],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T: Copy, const N: usize> RingBuff<T, N> {

    pub fn new() -> Self {
        assert!(N > 0);
        Self { items: [None; N], head: 0, tail: 0, len: 0 }
    }

    pub fn size(&self) -> usize {
        N
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Pushes an item to the head of the ring buffer, evicting the tail item once full
    pub fn push(&mut self, item: T) -> &mut Self {
        self.items[self.head] = Some(item);
        self.head = Self::next(self.head);
        if self.len == N {
            self.tail = Self::next(self.tail);
        } else {
            self.len += 1;
        }
        self
    }

    /// Pushes an item to the tail of the ring buffer, evicting the head item once full
    pub fn push_back(&mut self, item: T) -> &mut Self {
        self.tail = Self::prev(self.tail);
        self.items[self.tail] = Some(item);
        if self.len == N {
            self.head = Self::prev(self.head);
        } else {
            self.len += 1;
        }
        self
    }

    pub fn iter_to_tail<'a>(&'a self) -> RingBuffIterator<'a, T, N> {
        RingBuffIterator {
            items: &self.items,
            index: Self::prev(self.head),
            count: self.len(),
            step: Self::prev,
        }
    }

    pub fn iter_to_head<'a>(&'a self) -> RingBuffIterator<'a, T, N> {
        RingBuffIterator {
            items: &self.items,
            index: self.tail,
            count: self.len(),
            step: Self::next,
        }
    }

    pub fn next(index: usize) -> usize {
        if index == N - 1 { 0 } else { index + 1 }
    }

    pub fn prev(index: usize) -> usize {
        if index == 0 { N - 1 } else { index - 1 }
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

}
