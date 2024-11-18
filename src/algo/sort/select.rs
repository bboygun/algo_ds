pub fn sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let len = arr.len();
    for i in 0..len {
        let mut index = i;
        for j in i + 1..len {
            if arr[j] < arr[index] {
                index = j;
            }
        }
        arr.swap(i, index);
    }
}
