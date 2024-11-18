pub fn sort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    if arr.len() <= 1 {
        return;
    }

    let len = arr.len();
    let pivot = arr[0];
    let (mut left, mut right) = (0, len - 1);
    while left < right {
        while arr[left] < pivot {
            left += 1;
        }
        while arr[right] > pivot {
            right -= 1;
        }
        if left < right {
            arr.swap(left, right);
        }
    }
    sort(&mut arr[0..left]);
    sort(&mut arr[left + 1..]);
}
