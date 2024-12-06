// 41. 缺失的第一个正数
// 给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。
// 请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。

// 示例 1：
// 输入：nums = [1,2,0]
// 输出：3
// 解释：范围 [1,2] 中的数字都在数组中。

// 示例 2：
// 输入：nums = [3,4,-1,1]
// 输出：2
// 解释：1 在数组中，但 2 没有。

// 示例 3：
// 输入：nums = [7,8,9,11,12]
// 输出：1
// 解释：最小的正数 1 没有出现。

// 提示：
// 1 <= nums.length <= 10^5
// -2^31 <= nums[i] <= 2^31 - 1

fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let n = nums.len();

    for i in 0..n {
        while nums[i] > 0 && nums[i] <= (n as i32) && nums[(nums[i] - 1) as usize] != nums[i] {
            let to_swap = nums[i] - 1;
            nums.swap(i, to_swap as usize);
        }
    }

    for i in 0..n {
        if nums[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }
    (n + 1) as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn first_missing_positive_test() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
