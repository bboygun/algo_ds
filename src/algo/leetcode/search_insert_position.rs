// 35. 搜索插入位置
// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
// 请必须使用时间复杂度为 O(log n) 的算法。

// 示例 1:
// 输入: nums = [1,3,5,6], target = 5
// 输出: 2

// 示例 2:
// 输入: nums = [1,3,5,6], target = 2
// 输出: 1

// 示例 3:
// 输入: nums = [1,3,5,6], target = 7
// 输出: 4

// 提示:
// 1 <= nums.length <= 10^4
// -10^4 <= nums[i] <= 10^4
// nums 为 无重复元素 的 升序 排列数组
// -10^4 <= target <= 10^4

use std::cmp::Ordering;

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len());

    while left < right {
        let mid = left + (right - left) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Less => left = mid + 1,
            _ => right = mid,
        }
    }

    left as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn search_insert_test() {
        let case1 = vec![1, 3, 5, 6];
        let target1 = 5;
        let ans1 = 2;

        let case2 = vec![1, 3, 5, 6];
        let target2 = 2;
        let ans2 = 1;

        let case3 = vec![1, 3, 5, 6];
        let target3 = 7;
        let ans3 = 4;

        let case4 = vec![1];
        let target4 = 0;
        let ans4 = 0;

        let case5 = vec![1];
        let target5 = 2;
        let ans5 = 1;

        assert_eq!(search_insert(case1, target1), ans1);
        assert_eq!(search_insert(case2, target2), ans2);
        assert_eq!(search_insert(case3, target3), ans3);
        assert_eq!(search_insert(case4, target4), ans4);
        assert_eq!(search_insert(case5, target5), ans5);
    }
}
