// 46. 全排列
// 给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。

// 示例 1：
// 输入：nums = [1,2,3]
// 输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
//
// 示例 2：
// 输入：nums = [0,1]
// 输出：[[0,1],[1,0]]
//
// 示例 3：
// 输入：nums = [1]
// 输出：[[1]]

// 提示：
// 1 <= nums.length <= 6
// -10 <= nums[i] <= 10
// nums 中的所有整数 互不相同

fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    let mut track: Vec<i32> = Vec::new();

    let mut used: Vec<bool> = vec![false; nums.len()];

    backtrace(&nums, &mut result, &mut track, &mut used);
    result
}

fn backtrace(nums: &[i32], result: &mut Vec<Vec<i32>>, track: &mut Vec<i32>, used: &mut Vec<bool>) {
    if track.len() == nums.len() {
        result.push(track.clone());
    }

    for i in 0..nums.len() {
        if used[i] {
            continue;
        }
        track.push(nums[i]);
        used[i] = true;
        backtrace(nums, result, track, used);
        used[i] = false;
        track.pop();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn permute_test() {
        let case1 = vec![1, 2, 3];
        let ans1 = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        assert_eq!(permute(case1), ans1);

        let case2 = vec![0, 1];
        let ans2 = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(permute(case2), ans2);

        let case3 = vec![1];
        let ans3 = vec![vec![1]];
        assert_eq!(permute(case3), ans3);
    }
}
