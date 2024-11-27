use std::fs::read;

fn double_array(arr: &[u32]) -> u32 {
    let length = arr.len();
    let mut left_max = vec![0; length];
    let mut right_max = vec![0; length];

    // 第0个柱子的左边最高为0，后续第i个位置都用前一个位置的左边最高和前一个位置的柱子比较
    for i in 1..length {
        left_max[i] = if arr[i - 1] > left_max[i - 1] {
            arr[i - 1]
        } else {
            left_max[i - 1]
        }
    }

    // 第 length-1 个柱子的右边最高为0，后续第i个位置都用后一个位置的右边最高和后一个位置的柱子比较
    for i in (0..length - 1).rev() {
        right_max[i] = if arr[i + 1] > right_max[i + 1] {
            arr[i + 1]
        } else {
            right_max[i + 1]
        }
    }

    let mut ans = 0;
    for i in 1..length - 1 {
        let top = left_max[i].min(right_max[i]);
        if top > arr[i] {
            ans += top - arr[i];
        }
    }
    return ans;
}

fn double_pointer(arr: &[u32]) -> u32 {
    let (mut left, mut right) = (0, arr.len() - 1);
    let (mut left_max, mut right_max) = (arr[0], arr[arr.len() - 1]);
    let mut ans = 0;

    while left <= right {
        left_max = left_max.max(arr[left]);
        right_max = right_max.max(arr[right]);
        if left_max < right_max {
            ans += left_max - arr[left];
            left += 1;
        } else {
            ans += right_max - arr[right];
            let sub = right.checked_sub(1);
            if sub.is_none() {
                break;
            }
            right = sub.unwrap();
        }
    }
    ans
}

fn monotonic_stack(arr: &[u32]) -> u32 {
    let mut stack: Vec<usize> = Vec::new();
    stack.push(0);

    let mut ans = 0;
    for i in (1..arr.len()) {
        while !stack.is_empty() && arr[i] > arr[*stack.last().unwrap()] {
            let bottom_index = stack.pop().unwrap();
            if let Some(left_index) = stack.last() {
                let width = (i - left_index - 1) as u32;
                let height_diff = arr[*left_index].min(arr[i]) - arr[bottom_index];
                ans += width * height_diff;
            }
        }
        stack.push(i);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_array_test() {
        let case = get_case();
        for (height, ans) in case.iter() {
            assert_eq!(double_array(height), *ans);
        }
    }

    #[test]
    fn double_pointer_test() {
        let case = get_case();
        for (height, ans) in case.iter() {
            assert_eq!(double_pointer(height), *ans);
        }
    }

    #[test]
    fn monotonic_stack_test() {
        let case = get_case();
        for (height, ans) in case.iter() {
            assert_eq!(monotonic_stack(height), *ans);
        }
    }

    fn get_case() -> Vec<(Vec<u32>, u32)> {
        let height1 = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let ans1 = 6;

        let height2 = [4, 2, 0, 3, 2, 5];
        let ans2 = 9;

        let height3 = [0];
        let ans3 = 0;

        let mut vec: Vec<(Vec<u32>, u32)> = Vec::new();
        vec.push((height1.into(), ans1));
        vec.push((height2.into(), ans2));
        vec.push((height3.into(), ans3));
        vec
    }
}
