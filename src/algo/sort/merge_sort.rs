pub fn sort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let (mut left, mut right) = arr.split_at_mut(mid);

    sort(&mut left);
    sort(&mut right);
    merge(&mut left, &mut right);
}

fn merge<T>(left: &mut [T], right: &mut [T])
where
    T: PartialOrd + Copy,
{
    let (mut index1, mut index2) = (0, 0);
    let mut vec: Vec<T> = vec![];
    while index1 < left.len() && index2 < right.len() {
        if left[index1] <= right[index2] {
            vec.push(left[index1]);
            index1 += 1;
        } else {
            vec.push(right[index2]);
            index2 += 1;
        }
    }
    while index1 < left.len() {
        vec.push(left[index1]);
        index1 += 1;
    }
    while index2 < right.len() {
        vec.push(right[index2]);
        index2 += 1;
    }

    // 使用 copy_from_slice 复制到左侧和右侧
    let mid_point = left.len();
    left.copy_from_slice(&vec[..mid_point]); // 复制合并后的结果到左侧切片
    right.copy_from_slice(&vec[mid_point..]); // 复制合并后的结果到右侧切片
}

#[test]
fn merge_test() {
    let mut vec = vec![4, 9, 1, 4, 7, 6, 4, 8, 7];
    let mid = vec.len() / 2;
    let (mut left, mut right) = vec.split_at_mut(mid);
    merge(&mut left, &mut right);
    assert_eq!(vec, vec![4, 7, 6, 4, 8, 7, 9, 1, 4]);
}
