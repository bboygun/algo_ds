pub fn sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let len = arr.len();
    let mut swapped;

    for i in 0..len {
        swapped = false;
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                swapped = true;
                arr.swap(j, j + 1);
            }
        }
        if !swapped {
            break;
        }
    }
}
