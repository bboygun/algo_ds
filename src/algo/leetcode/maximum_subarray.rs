// 53. 最大子数组和
// 给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
// 子数组是数组中的一个连续部分。

// 示例 1：
// 输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
// 输出：6
// 解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。

// 示例 2：
// 输入：nums = [1]
// 输出：1

// 示例 3：
// 输入：nums = [5,4,-1,7,8]
// 输出：23

// 提示：
// 1 <= nums.length <= 10^5
// -104 <= nums[i] <= 10^4

// 进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。

fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = Vec::with_capacity(nums.len());
    dp.push(nums[0]);

    let mut max_sum = dp[0];
    for i in 1..nums.len() {
        if dp[i - 1] < 0 {
            dp.push(nums[i]);
        } else {
            dp.push(nums[i] + dp[i - 1]);
        }
        max_sum = max_sum.max(dp[i]);
    }
    max_sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_sub_array_test() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![1]), 1);
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
