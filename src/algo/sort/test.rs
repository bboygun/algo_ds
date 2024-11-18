#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::sort;

    #[test]
    fn test_bubble_sort() {
        test_sort_i32(sort::bubble_sort::sort);
        test_sort_f64(sort::bubble_sort::sort);
        test_sort_char(sort::bubble_sort::sort);
    }

    #[test]
    fn test_insert_sort() {
        test_sort_i32(sort::insert_sort::sort);
        test_sort_f64(sort::insert_sort::sort);
        test_sort_char(sort::insert_sort::sort);
    }

    #[test]
    fn test_quick_sort() {
        test_sort_i32(sort::quick_sort::sort);
        test_sort_f64(sort::quick_sort::sort);
        test_sort_char(sort::quick_sort::sort);
    }

    #[test]
    fn test_merge_sort() {
        test_sort_i32(sort::merge_sort::sort);
        test_sort_f64(sort::merge_sort::sort);
        test_sort_char(sort::merge_sort::sort);
    }

    #[test]
    fn test_select_sort() {
        test_sort_i32(sort::select_sort::sort);
        test_sort_f64(sort::select_sort::sort);
        test_sort_char(sort::select_sort::sort);
    }

    #[test]
    fn test_heap_sort() {
        test_sort_i32(sort::heap_sort::sort);
        test_sort_f64(sort::heap_sort::sort);
        test_sort_char(sort::heap_sort::sort);
    }

    fn test_sort_i32<F>(mut sort_fn: F)
    where
        F: FnMut(&mut [i32]),
    {
        // 测试整数排序
        let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
        sort_fn(&mut numbers);
        assert_eq!(numbers, vec![11, 12, 22, 25, 34, 64, 90]);

        // 测试空数组
        let mut empty: Vec<i32> = vec![];
        sort_fn(&mut empty);
        assert_eq!(empty, vec![]);

        // 测试单个元素的数组
        let mut single = vec![1];
        sort_fn(&mut single);
        assert_eq!(single, vec![1]);
    }

    fn test_sort_f64<F>(sort_fn: F)
    where
        F: Fn(&mut [f64]),
    {
        // 测试浮点数排序
        let mut floats = vec![3.14, 1.0, 2.5, 0.5];
        sort_fn(&mut floats);
        assert_eq!(floats, vec![0.5, 1.0, 2.5, 3.14]);
    }

    fn test_sort_char<F>(sort_fn: F)
    where
        F: Fn(&mut [char]),
    {
        // 测试字符排序
        let mut chars = vec!['d', 'a', 'c', 'b'];
        sort_fn(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd']);
    }
}
