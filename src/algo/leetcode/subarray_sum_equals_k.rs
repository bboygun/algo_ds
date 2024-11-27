// 560. 和为 K 的子数组
// 给你一个整数数组 nums 和一个整数 k ，请你统计并返回 该数组中和为 k 的子数组的个数 。
// 子数组是数组中元素的连续非空序列。

// 示例 1：
// 输入：nums = [1,1,1], k = 2
// 输出：2

// 示例 2：
// 输入：nums = [1,2,3], k = 3
// 输出：2

// 提示：
// 1 <= nums.length <= 2 * 104
// -1000 <= nums[i] <= 1000
// -107 <= k <= 107

use std::collections::HashMap;

fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut pre_sum = 0;
    let mut ans = 0;
    let mut map = HashMap::new();
    // 因为pre_num[i]为nums[0..=i]的和，要额外加上nums为空数组时的情况，即和为0必会出现一次
    map.insert(0, 1);
    for i in 0..nums.len() {
        pre_sum += nums[i];
        let target = pre_sum - k;
        if map.contains_key(&target) {
            ans += *map.get(&target).unwrap();
        }
        map.entry(pre_sum)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    ans
}

/*
此函数中使用前缀和数组，但其实最后计数的循环中是依次使用前缀和的，所以无需提前运算
可以优化成一个pre_sum变量，在计数的循环中一并处理。优化后就是上面的函数
*/
fn subarray_sum_array(nums: Vec<i32>, k: i32) -> i32 {
    let mut pre_sum = vec![0; nums.len()];
    pre_sum[0] = nums[0];

    for i in 1..nums.len() {
        pre_sum[i] = pre_sum[i - 1] + nums[i];
    }

    let mut ans = 0;
    let mut map = HashMap::new();
    map.insert(0, 1);
    for i in 0..pre_sum.len() {
        let target = pre_sum[i] - k;
        if map.contains_key(&target) {
            ans += *map.get(&target).unwrap();
        }
        map.entry(pre_sum[i])
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    ans
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn subarray_sum_test() {
        let case_list = [([1, 1, 1], 2), ([1, 2, 3], 3)];
        let ans_list = [2, 2];

        for ((nums, k), ans) in case_list.into_iter().zip(ans_list.into_iter()) {
            assert_eq!(subarray_sum(nums.into(), k), ans);
        }
    }
}
