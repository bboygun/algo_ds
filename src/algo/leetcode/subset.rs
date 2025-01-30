// 78. 子集
// 给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的子集（幂集）。
// 解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。

// 示例 1：
// 输入：nums = [1,2,3]
// 输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]

// 示例 2：
// 输入：nums = [0]
// 输出：[[],[0]]

// 提示：
// 1 <= nums.length <= 10
// -10 <= nums[i] <= 10
// nums 中的所有元素 互不相同

fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut track: Vec<i32> = vec![];

    backtrace(0, &nums, &mut result, &mut track);
    result
}

fn backtrace(start: usize, nums: &Vec<i32>, result: &mut Vec<Vec<i32>>, track: &mut Vec<i32>) {
    result.push(track.clone());

    for i in start..nums.len() {
        track.push(nums[i]);
        backtrace(i + 1, nums, result, track);
        track.pop();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn subset_test() {
        let case1 = vec![1, 2, 3];
        let ans1 = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        println!("{:?}", subsets(case1));
    }
}
