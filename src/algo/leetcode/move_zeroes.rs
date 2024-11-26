// 283. 移动零
// 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
// 请注意 ，必须在不复制数组的情况下原地对数组进行操作。

// 示例 1:
// 输入: nums = [0,1,0,3,12]
// 输出: [1,3,12,0,0]

// 示例 2:
// 输入: nums = [0]
// 输出: [0]

// 提示:
// 1 <= nums.length <= 104
// -231 <= nums[i] <= 231 - 1

fn move_zeroes(nums: &mut Vec<i32>) {
    let mut left = 0;
    for right in 0..nums.len() {
        // 找到非0
        if nums[right] == 0 {
            continue;
        }
        // 与左边交换
        if left != right {
            nums.swap(left, right);
        }
        // 交换过一次之后，left必然是非0，且从[left + 1, right)全是0
        left += 1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_case() -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
        let questions = vec![vec![0, 1, 0, 3, 12], vec![0]];
        let ans = vec![vec![1, 3, 12, 0, 0], vec![0]];
        (questions, ans)
    }

    #[test]
    fn move_zeroes_test() {
        let (mut nums, ans) = get_case();
        for i in 0..nums.len() {
            move_zeroes(&mut nums[i]);
            assert_eq!(nums[i], ans[i]);
        }
    }
}
