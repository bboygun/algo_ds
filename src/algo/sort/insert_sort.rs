pub fn sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let len = arr.len();
    // 第0个元素默认有序，所以从1开始比较
    for cur in 1..len {
        // 由于从当前元素开始往前比较，所以要取到cur
        for i in (1..=cur).rev() {
            if arr[i] < arr[i - 1] {
                arr.swap(i, i - 1);
            }
        }
    }
}
