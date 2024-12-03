use std::fmt::Debug;

pub struct MaxHeap<T> {
    vec: Vec<T>,
}

impl<T> MaxHeap<T>
where
    T: PartialOrd + Copy + Debug,
{
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn from(arr: &[T]) -> Self {
        let mut heap = MaxHeap::new();
        for &item in arr {
            heap.insert(item);
        }
        heap
    }

    pub fn insert(&mut self, item: T) {
        self.vec.push(item);
        self.bubble_up(self.size() - 1);
    }

    pub fn extract_max(&mut self) -> Option<T> {
        if self.vec.is_empty() {
            return None;
        }
        let max = self.vec[0];
        let size = self.size();
        self.vec.swap(0, size - 1);
        self.vec.pop();
        self.bubble_down(0);
        Some(max)
    }

    pub fn size(&self) -> usize {
        self.vec.len()
    }

    fn bubble_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }
        let parent = (index - 1) / 2;
        if self.vec[parent] < self.vec[index] {
            self.vec.swap(parent, index);
            self.bubble_up(parent);
        }
    }

    fn bubble_down(&mut self, index: usize) {
        let left_index = index * 2 + 1;
        let right_index = left_index + 1;
        let size = self.size();

        let mut to_swap = index;
        if left_index < size && self.vec[left_index] > self.vec[to_swap] {
            to_swap = left_index;
        }

        if right_index < size && self.vec[right_index] > self.vec[to_swap] {
            to_swap = right_index;
        }

        if to_swap != index {
            self.vec.swap(to_swap, index);
            self.bubble_down(to_swap);
        }
    }

    pub fn extract_value(&mut self, value: T) -> Option<T> {
        let len = self.size();
        for index in 0..len {
            if self.vec[index] == value {
                self.vec.swap(index, len - 1);
                let extract_value = self.vec.pop();
                // 若返回的是最后一个元素，则不需要再处理
                if index != len - 1 {
                    if index > 0 && self.vec[index] > self.vec[(index - 1) / 2] {
                        self.bubble_up(index);
                    } else {
                        self.bubble_down(index);
                    }
                }
                return extract_value;
            }
        }
        None
    }

    pub fn max(&self) -> Option<T> {
        if self.size() > 0 {
            Some(self.vec[0])
        } else {
            None
        }
    }

    pub fn println_vec(&self) {
        println!("{:?}", self.vec);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_heap_test() {
        let vec = vec![4, 9, 1, 4, 7, 6, 4, 8, 7];
        let mut max_heap = MaxHeap::from(&vec);

        let expected_vec = vec![9, 8, 6, 7, 4, 1, 4, 4, 7];
        assert_eq!(max_heap.vec, expected_vec);

        assert_eq!(Some(9), max_heap.extract_max());
        assert_eq!(max_heap.vec, vec![8, 7, 6, 7, 4, 1, 4, 4]);

        assert_eq!(Some(8), max_heap.extract_max());
        assert_eq!(max_heap.vec, vec![7, 7, 6, 4, 4, 1, 4]);

        assert_eq!(Some(7), max_heap.extract_max());
        assert_eq!(max_heap.vec, vec![7, 4, 6, 4, 4, 1]);

        assert_eq!(Some(7), max_heap.extract_max());
        assert_eq!(max_heap.vec, vec![6, 4, 1, 4, 4]);

        assert_eq!(Some(6), max_heap.extract_max());
        assert_eq!(max_heap.vec, vec![4, 4, 1, 4]);

        assert_eq!(Some(4), max_heap.extract_max());
        assert_eq!(max_heap.vec, vec![4, 4, 1]);

        assert_eq!(Some(4), max_heap.extract_max());
        assert_eq!(max_heap.vec, vec![4, 1]);

        assert_eq!(Some(4), max_heap.extract_max());
        assert_eq!(max_heap.vec, vec![1]);

        assert_eq!(Some(1), max_heap.extract_max());
        assert_eq!(max_heap.vec, vec![]);

        assert!(max_heap.extract_max().is_none());
    }

    #[test]
    fn extrace_value_test() {
        let vec = vec![4, 9, 1, 4, 7, 6, 4, 8, 7];
        let mut max_heap = MaxHeap::from(&vec);

        assert_eq!(Some(4), max_heap.extract_value(4));
        assert_eq!(None, max_heap.extract_value(99));
        assert_eq!(max_heap.vec, vec![9, 8, 6, 7, 7, 1, 4, 4]);
    }

    #[test]
    fn badcase_test() {
        let vec = vec![100, 63, 43, 35, 59, -75, -65, -49, -48, 3];
        let mut max_heap = MaxHeap::from(&vec);
        max_heap.println_vec();
        max_heap.extract_value(-75);
        max_heap.println_vec();
        max_heap.insert(48);
        max_heap.println_vec();

        println!();

        let vec = vec![100, 63, 43, 35, 59, -75, -65, -49, -48, 3];
        let mut max_heap = MaxHeap::from(&vec);
        max_heap.println_vec();
        max_heap.insert(48);
        max_heap.println_vec();
        max_heap.extract_value(-75);
        max_heap.println_vec();
    }
}
