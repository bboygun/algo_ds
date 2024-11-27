// 15. 三数之和
// 给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请你返回所有和为 0 且不重复的三元组。
// 注意：答案中不可以包含重复的三元组。

// 示例 1：
// 输入：nums = [-1,0,1,2,-1,-4]
// 输出：[[-1,-1,2],[-1,0,1]]
// 解释：
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
// 不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
// 注意，输出的顺序和三元组的顺序并不重要。

// 示例 2：
// 输入：nums = [0,1,1]
// 输出：[]
// 解释：唯一可能的三元组和不为 0 。

// 示例 3：
// 输入：nums = [0,0,0]
// 输出：[[0,0,0]]
// 解释：唯一可能的三元组和为 0 。

// 提示：
// 3 <= nums.length <= 3000
// -105 <= nums[i] <= 105

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    let mut ans = vec![];
    for k in 0..nums.len() - 2 {
        if nums[k] > 0 {
            break;
        }
        if k > 0 && nums[k] == nums[k - 1] {
            continue;
        }
        let (mut i, mut j) = (k + 1, nums.len() - 1);
        while i < j {
            let s = nums[k] + nums[i] + nums[j];
            if s < 0 {
                i += 1;
                while i < j && nums[i] == nums[i - 1] {
                    i += 1;
                }
            } else if s > 0 {
                j -= 1;
                while i < j && nums[j] == nums[j + 1] {
                    j -= 1
                }
            } else {
                ans.push(vec![nums[k], nums[i], nums[j]]);
                i += 1;
                while i < j && nums[i] == nums[i - 1] {
                    i += 1;
                }
                j -= 1;
                while i < j && nums[j] == nums[j + 1] {
                    j -= 1
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_case() -> (Vec<Vec<i32>>, Vec<Vec<Vec<i32>>>) {
        let case_list = vec![vec![-1, 0, 1, 2, -1, -4], vec![0, 1, 1], vec![0, 0, 0]];
        let ans_list = vec![
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            vec![],
            vec![vec![0, 0, 0]],
        ];
        (case_list, ans_list)
    }

    #[test]
    fn three_sum_test() {
        let (mut case_list, ans_list) = get_case();
        for (case, ans) in case_list.into_iter().zip(ans_list.into_iter()) {
            assert_eq!(three_sum(case), ans);
        }
    }
}
