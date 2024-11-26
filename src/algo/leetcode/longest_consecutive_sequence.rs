// 128. 最长连续序列
// 给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
// 请你设计并实现时间复杂度为 O(n) 的算法解决此问题。

// 示例 1：
// 输入：nums = [100,4,200,1,3,2]
// 输出：4
// 解释：最长数字连续序列是 [1, 2, 3, 4]。它的长度为 4。

// 示例 2：
// 输入：nums = [0,3,7,2,5,8,4,6,0,1]
// 输出：9

// 提示：
// 0 <= nums.length <= 105
// -109 <= nums[i] <= 109

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    sort_method(&mut nums)
}

fn sort_method(nums: &mut Vec<i32>) -> i32 {
    nums.sort();

    let mut max_ans = 0;
    let mut left = 0;

    while left < nums.len() {
        let mut right = left;
        let mut ans = 1;
        while right < nums.len() - 1 && nums[right + 1] - nums[right] <= 1 {
            if nums[right + 1] - nums[right] == 1 {
                ans += 1;
            }
            right += 1;
        }
        left = right + 1;
        max_ans = max_ans.max(ans);
    }

    max_ans
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sort_method_test() {
        let (mut case, ans) = get_case();
        for i in 0..case.len() {
            assert_eq!(sort_method(&mut case[i]), ans[i]);
        }
    }

    fn get_case() -> (Vec<Vec<i32>>, Vec<i32>) {
        let case = vec![
            vec![],
            vec![100, 4, 200, 1, 3, 2],
            vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1],
            vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6],
        ];

        let ans = vec![0, 4, 9, 7];

        (case, ans)
    }
}
