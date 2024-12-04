// 239. 滑动窗口最大值
// 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。
// 返回滑动窗口中的最大值 。

// 示例 1：
// 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
// 输出：[3,3,5,5,6,7]
// 解释：
// 滑动窗口的位置                最大值
// ---------------               -----
// [1  3  -1] -3  5  3  6  7       3
//  1 [3  -1  -3] 5  3  6  7       3
//  1  3 [-1  -3  5] 3  6  7       5
//  1  3  -1 [-3  5  3] 6  7       5
//  1  3  -1  -3 [5  3  6] 7       6
//  1  3  -1  -3  5 [3  6  7]      7

// 示例 2：
// 输入：nums = [1], k = 1
// 输出：[1]

// 提示：
// 1 <= nums.length <= 10^5
// -10^4 <= nums[i] <= 10^4
// 1 <= k <= nums.length
use crate::ds::heap::MaxHeap;

fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let (mut left, mut right) = (0, k as usize);
    let mut heap = MaxHeap::from(&nums[left..right]);

    let mut ans = vec![];
    while right < nums.len() {
        ans.push(heap.max().unwrap());
        heap.extract_value(nums[left]);
        heap.insert(nums[right]);
        left += 1;
        right += 1;
    }
    ans.push(heap.max().unwrap());
    ans
}

fn max_sliding_window_monotonic_stack(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let (mut left, mut right) = (0 - k, 0);
    let mut queue = vec![];
    let mut ans = vec![];

    while right < nums.len() {
        if left >= 0 && nums[left as usize] == queue[0] {
            queue.remove(0);
        }
        while let Some(&top) = queue.get(queue.len().saturating_sub(1)) {
            if top < nums[right] {
                queue.pop();
            } else {
                break;
            }
        }
        queue.push(nums[right]);
        if right >= (k - 1) as usize {
            ans.push(queue[0]);
        }
        right += 1;
        left += 1;
    }
    ans
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_sliding_window_test() {
        let ans1 = max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        assert_eq!(ans1, vec![3, 3, 5, 5, 6, 7]);

        let ans2 = max_sliding_window(vec![1], 1);
        assert_eq!(ans2, vec![1]);

        let ans3 = max_sliding_window(
            vec![
                -95, 92, -85, 59, -59, -14, 88, -39, 2, 92, 94, 79, 78, -58, 37, 48, 63, -91, 91,
                74, -28, 39, 90, -9, -72, -88, -72, 93, 38, 14, -83, -2, 21, 4, -75, -65, 3, 63,
                100, 59, -48, 43, 35, -49, 48, -36, -64, -13, -7, -29, 87, 34, 56, -39, -5, -27,
                -28, 10, -57, 100, -43, -98, 19, -59, 78, -28, -91, 67, 41, -64, 76, 5, -58, -89,
                83, 26, -7, -82, -32, -76, 86, 52, -6, 84, 20, 51, -86, 26, 46, 35, -23, 30, -51,
                54, 19, 30, 27, 80, 45, 22,
            ],
            10,
        );
        assert_eq!(
            ans3,
            vec![
                92, 94, 94, 94, 94, 94, 94, 94, 94, 94, 94, 91, 91, 91, 91, 91, 91, 91, 93, 93, 93,
                93, 93, 93, 93, 93, 93, 93, 63, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                59, 48, 87, 87, 87, 87, 87, 87, 87, 87, 87, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 78, 78, 78, 78, 78, 83, 83, 83, 83, 83, 83, 86, 86, 86, 86, 86, 86, 86,
                86, 86, 86, 84, 84, 84, 54, 54, 54, 54, 80, 80, 80
            ]
        );
    }

    #[test]
    fn max_sliding_window_monotonic_stack_test() {
        let ans1 = max_sliding_window_monotonic_stack(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        assert_eq!(ans1, vec![3, 3, 5, 5, 6, 7]);

        let ans2 = max_sliding_window_monotonic_stack(vec![1], 1);
        assert_eq!(ans2, vec![1]);

        let ans3 = max_sliding_window_monotonic_stack(
            vec![
                -95, 92, -85, 59, -59, -14, 88, -39, 2, 92, 94, 79, 78, -58, 37, 48, 63, -91, 91,
                74, -28, 39, 90, -9, -72, -88, -72, 93, 38, 14, -83, -2, 21, 4, -75, -65, 3, 63,
                100, 59, -48, 43, 35, -49, 48, -36, -64, -13, -7, -29, 87, 34, 56, -39, -5, -27,
                -28, 10, -57, 100, -43, -98, 19, -59, 78, -28, -91, 67, 41, -64, 76, 5, -58, -89,
                83, 26, -7, -82, -32, -76, 86, 52, -6, 84, 20, 51, -86, 26, 46, 35, -23, 30, -51,
                54, 19, 30, 27, 80, 45, 22,
            ],
            10,
        );
        assert_eq!(
            ans3,
            vec![
                92, 94, 94, 94, 94, 94, 94, 94, 94, 94, 94, 91, 91, 91, 91, 91, 91, 91, 93, 93, 93,
                93, 93, 93, 93, 93, 93, 93, 63, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                59, 48, 87, 87, 87, 87, 87, 87, 87, 87, 87, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 78, 78, 78, 78, 78, 83, 83, 83, 83, 83, 83, 86, 86, 86, 86, 86, 86, 86,
                86, 86, 86, 84, 84, 84, 54, 54, 54, 54, 80, 80, 80
            ]
        );
    }
}
