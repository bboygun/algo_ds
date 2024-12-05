// 189. 轮转数组
// 给定一个整数数组 nums，将数组中的元素向右轮转 k 个位置，其中 k 是非负数。

// 示例 1:
// 输入: nums = [1,2,3,4,5,6,7], k = 3
// 输出: [5,6,7,1,2,3,4]
// 解释:
// 向右轮转 1 步: [7,1,2,3,4,5,6]
// 向右轮转 2 步: [6,7,1,2,3,4,5]
// 向右轮转 3 步: [5,6,7,1,2,3,4]

// 示例 2:
// 输入：nums = [-1,-100,3,99], k = 2
// 输出：[3,99,-1,-100]
// 解释:
// 向右轮转 1 步: [99,-1,-100,3]
// 向右轮转 2 步: [3,99,-1,-100]

// 提示：
// 1 <= nums.length <= 10^5
// -2^31 <= nums[i] <= 2^31 - 1
// 0 <= k <= 10^5

// 进阶：
// 尽可能想出更多的解决方案，至少有 三种 不同的方法可以解决这个问题。
// 你可以使用空间复杂度为 O(1) 的 原地 算法解决这个问题吗？

fn rotate(nums: &mut Vec<i32>, k: i32) {}

fn copy_method(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let k = k as usize % len;
    let tmp = nums[len - k..len].to_vec();
    nums.copy_within(0..len - k, k);
    nums[0..k].copy_from_slice(&tmp);
}

fn rotate_method(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let k = k as usize % len;
    nums.rotate_right(k);
}

fn reverse_method(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let k = k as usize % len;
    nums.reverse();
    nums[0..k].reverse();
    nums[k..len].reverse();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn copy_method_test() {
        let mut nums1 = [1, 2, 3, 4, 5, 6, 7].to_vec();
        copy_method(&mut nums1, 3);
        assert_eq!(nums1, [5, 6, 7, 1, 2, 3, 4]);

        let mut nums2 = [-1, -100, 3, 99].to_vec();
        copy_method(&mut nums2, 2);
        assert_eq!(nums2, [3, 99, -1, -100]);
    }

    #[test]
    fn rotate_method_test() {
        let mut nums1 = [1, 2, 3, 4, 5, 6, 7].to_vec();
        rotate_method(&mut nums1, 3);
        assert_eq!(nums1, [5, 6, 7, 1, 2, 3, 4]);

        let mut nums2 = [-1, -100, 3, 99].to_vec();
        rotate_method(&mut nums2, 2);
        assert_eq!(nums2, [3, 99, -1, -100]);
    }

    #[test]
    fn reverse_method_test() {
        let mut nums1 = [1, 2, 3, 4, 5, 6, 7].to_vec();
        reverse_method(&mut nums1, 3);
        assert_eq!(nums1, [5, 6, 7, 1, 2, 3, 4]);

        let mut nums2 = [-1, -100, 3, 99].to_vec();
        reverse_method(&mut nums2, 2);
        assert_eq!(nums2, [3, 99, -1, -100]);
    }
}
