pub struct MinHeap<T> {
    vec: Vec<T>,
}

impl<T> MinHeap<T>
where
    T: PartialOrd + Copy,
{
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn from(arr: &[T]) -> Self {
        let mut heap = MinHeap::new();
        for &item in arr {
            heap.insert(item);
        }
        heap
    }

    pub fn insert(&mut self, item: T) {
        self.vec.push(item);
        self.bubble_up(self.size() - 1);
    }

    pub fn extract_min(&mut self) -> Option<T> {
        if self.vec.is_empty() {
            return None;
        }
        let min = self.vec[0];
        let size = self.size();
        self.vec.swap(0, size - 1);
        self.vec.pop();
        self.bubble_down(0);
        Some(min)
    }

    pub fn size(&self) -> usize {
        self.vec.len()
    }

    fn bubble_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }
        let parent = (index - 1) / 2;
        if self.vec[parent] > self.vec[index] {
            self.vec.swap(parent, index);
        }
        self.bubble_up(parent);
    }

    fn bubble_down(&mut self, index: usize) {
        let left_index = index * 2 + 1;
        let right_index = left_index + 1;
        let size = self.size();

        let mut to_swap = index;
        if left_index >= size {
            return;
        } else {
            if self.vec[left_index] < self.vec[index] {
                to_swap = left_index;
            }
        }

        if right_index < size && self.vec[right_index] < self.vec[to_swap] {
            to_swap = right_index;
        }

        if to_swap != index {
            self.vec.swap(to_swap, index);
            self.bubble_down(to_swap);
        }
    }
}

#[test]
fn min_heap_test() {
    let vec = vec![4, 9, 1, 4, 7, 6, 4, 8, 7];
    let min_heap = MinHeap::from(&vec);

    let expected_vec = vec![1, 4, 4, 7, 7, 6, 4, 9, 8];
    assert_eq!(min_heap.vec, expected_vec);
}
