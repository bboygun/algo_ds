// 33. 搜索旋转排序数组
// 整数数组 nums 按升序排列，数组中的值 互不相同 。
// 在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行了 旋转，使数组变为 [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]（下标 从 0 开始 计数）。例如， [0,1,2,4,5,6,7] 在下标 3 处经旋转后可能变为 [4,5,6,7,0,1,2] 。
// 给你 旋转后 的数组 nums 和一个整数 target ，如果 nums 中存在这个目标值 target ，则返回它的下标，否则返回 -1 。
// 你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。

// 示例 1：
// 输入：nums = [4,5,6,7,0,1,2], target = 0
// 输出：4

// 示例 2：
// 输入：nums = [4,5,6,7,0,1,2], target = 3
// 输出：-1

// 示例 3：
// 输入：nums = [1], target = 0
// 输出：-1

// 提示：
// 1 <= nums.length <= 5000
// -10^4 <= nums[i] <= 10^4
// nums 中的每个值都 独一无二
// 题目数据保证 nums 在预先未知的某个下标上进行了旋转
// -10^4 <= target <= 10^4

use std::cmp::Ordering;

fn search(nums: Vec<i32>, target: i32) -> i32 {
    let rotated_index = find_min_index(&nums);
    if let Some(index) = binary_search(&nums[..rotated_index], &target) {
        return index as i32;
    }
    match binary_search(&nums[rotated_index..], &target) {
        Some(index) => (rotated_index + index) as i32,
        None => -1,
    }
}

fn find_min_index(nums: &[i32]) -> usize {
    let right_value = nums[nums.len() - 1];
    let (mut left, mut right) = (0, nums.len());

    while left < right {
        let mid = left + (right - left) / 2;
        match nums[mid].cmp(&right_value) {
            Ordering::Greater => left = mid + 1,
            _ => right = mid,
        }
    }

    left
}

fn binary_search(nums: &[i32], target: &i32) -> Option<usize> {
    let (mut left, mut right) = (0, nums.len());
    while left < right {
        let mid = left + (right - left) / 2;
        match nums[mid].cmp(target) {
            Ordering::Less => left = mid + 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => right = mid,
        }
    }

    if left == nums.len() || nums[left] != *target {
        None
    } else {
        Some(left)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn search_in_rotated_sorted_array_test() {
        // let nums1 = vec![4, 5, 6, 7, 0, 1, 2];
        // let target1 = 0;
        // assert_eq!(search(nums1.clone(), target1), 4);

        // let target2 = 3;
        // assert_eq!(search(nums1, target2), -1);

        // let nums3 = vec![1];
        // let target3 = 0;
        // assert_eq!(search(nums3, target3), -1);

        let nums4 = vec![3, 1];
        let target4 = 0;
        assert_eq!(search(nums4, target4), -1);
    }

    #[test]
    fn search_rotated_index_test() {
        let nums1 = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(find_min_index(&nums1), 4);

        let nums2 = vec![1];
        assert_eq!(find_min_index(&nums2), 0);

        let nums3 = vec![1, 3];
        assert_eq!(find_min_index(&nums3), 0);
    }
}
