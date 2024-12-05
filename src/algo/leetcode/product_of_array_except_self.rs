// 238. 除自身以外数组的乘积
// 给你一个整数数组 nums，返回 数组 answer ，其中 answer[i] 等于 nums 中除 nums[i] 之外其余各元素的乘积 。
// 题目数据保证数组 nums之中任意元素的全部前缀元素和后缀的乘积都在 32 位整数范围内。

// 请不要使用除法，且在 O(n) 时间复杂度内完成此题。

// 示例 1:
// 输入: nums = [1,2,3,4]
// 输出: [24,12,8,6]

// 示例 2:
// 输入: nums = [-1,1,0,-3,3]
// 输出: [0,0,9,0,0]

// 提示：
// 2 <= nums.length <= 10^5
// -30 <= nums[i] <= 30
// 保证 数组 nums之中任意元素的全部前缀元素和后缀的乘积都在  32 位 整数范围内

// 进阶：你可以在 O(1) 的额外空间复杂度内完成这个题目吗？（ 出于对空间复杂度分析的目的，输出数组 不被视为 额外空间。）

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    prefix_suffix_method(nums)
}

fn divide_method(nums: Vec<i32>) -> Vec<i32> {
    let mut zero_count = 0;
    let mut product = 1;

    for &num in nums.iter() {
        if num == 0 {
            zero_count += 1;
            continue;
        }
        product *= num;
    }

    let mut ans = vec![0; nums.len()];
    if zero_count > 1 {
        return ans;
    }

    for (index, num) in nums.into_iter().enumerate() {
        if zero_count == 0 {
            ans[index] = product / num;
        } else if (num == 0) {
            ans[index] = product;
        }
    }
    ans
}

fn prefix_suffix_method(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![1; nums.len()];
    let mut prefix_product = 1;

    for i in 0..nums.len() {
        ans[i] = prefix_product;
        prefix_product *= nums[i];
    }

    let mut suffix_product = 1;
    for i in (0..nums.len()).rev() {
        ans[i] *= suffix_product;
        suffix_product *= nums[i];
    }

    ans
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn divide_method_test() {
        assert_eq!(divide_method(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(divide_method(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
    }

    #[test]
    fn prefix_suffix_method_test() {
        assert_eq!(prefix_suffix_method(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(
            prefix_suffix_method(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
