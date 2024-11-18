fn sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        // 测试整数排序
        let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
        sort(&mut numbers);
        assert_eq!(numbers, vec![11, 12, 22, 25, 34, 64, 90]);

        // 测试浮点数排序
        let mut floats = vec![3.14, 1.0, 2.5, 0.5];
        sort(&mut floats);
        assert_eq!(floats, vec![0.5, 1.0, 2.5, 3.14]);

        // 测试字符排序
        let mut chars = vec!['d', 'a', 'c', 'b'];
        sort(&mut chars);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd']);

        // 测试空数组
        let mut empty: Vec<i32> = vec![];
        sort(&mut empty);
        assert_eq!(empty, vec![]);

        // 测试单个元素的数组
        let mut single = vec![1];
        sort(&mut single);
        assert_eq!(single, vec![1]);
    }
}
