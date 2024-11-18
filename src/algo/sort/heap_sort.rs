use crate::ds::heap::min_heap::MinHeap;

pub fn sort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    let mut min_heap = MinHeap::from(arr);
    for i in 0..arr.len() {
        arr[i] = min_heap.extract_min().unwrap();
    }
}
