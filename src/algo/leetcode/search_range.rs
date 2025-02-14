// 34. 在排序数组中查找元素的第一个和最后一个位置
// 给你一个按照非递减顺序排列的整数数组 nums，和一个目标值 target。请你找出给定目标值在数组中的开始位置和结束位置。
// 如果数组中不存在目标值 target，返回 [-1, -1]。
// 你必须设计并实现时间复杂度为 O(log n) 的算法解决此问题。

// 示例 1：
// 输入：nums = [5,7,7,8,8,10], target = 8
// 输出：[3,4]

// 示例 2：
// 输入：nums = [5,7,7,8,8,10], target = 6
// 输出：[-1,-1]

// 示例 3：
// 输入：nums = [], target = 0
// 输出：[-1,-1]

// 提示：
// 0 <= nums.length <= 10^5
// -10^9 <= nums[i] <= 10^9
// nums 是一个非递减数组
// -10^9 <= target <= 10^9

use std::cmp::Ordering;

fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let start = lower_bound(&nums, &target);
    if start == nums.len() || nums[start] != target {
        return vec![-1, -1];
    }

    let end = lower_bound(&nums[start..], &(target + 1)) + start - 1;
    vec![start as i32, end as i32]
}

fn lower_bound(nums: &[i32], target: &i32) -> usize {
    let (mut left, mut right) = (0, nums.len());
    while left < right {
        let mid = left + (right - left) / 2;
        match nums[mid].cmp(target) {
            Ordering::Less => left = mid + 1,
            _ => right = mid,
        }
    }
    left
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn search_range_test() {
        let nums1 = vec![5, 7, 7, 8, 8, 10];
        let target1 = 8;
        assert_eq!(search_range(nums1, target1), vec![3, 4]);

        let nums2 = vec![5, 7, 7, 8, 8, 10];
        let target2 = 6;
        assert_eq!(search_range(nums2, target2), vec![-1, -1]);

        let nums3 = vec![];
        let target3 = 0;
        assert_eq!(search_range(nums3, target3), vec![-1, -1]);

        let nums4 = vec![1];
        let target4 = 1;
        assert_eq!(search_range(nums4, target4), vec![0, 0]);
    }
}
