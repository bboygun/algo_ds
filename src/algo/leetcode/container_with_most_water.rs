// 11. 盛最多水的容器
// 给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i]) 。
// 找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
// 返回容器可以储存的最大水量。
// 说明：你不能倾斜容器。

// 示例 1：
// 输入：[1,8,6,2,5,4,8,3,7]
// 输出：49
// 解释：图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。

// 示例 2：
// 输入：height = [1,1]
// 输出：1

// 提示：
// n == height.length
// 2 <= n <= 105
// 0 <= height[i] <= 104

fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;

    let mut max_area = 0;
    while left < right {
        let top = height[left].min(height[right]) as usize;
        let cur_area = top * (right - left);
        max_area = max_area.max(cur_area);
        if height[left] <= height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max_area as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_cases() -> (Vec<Vec<i32>>, Vec<i32>) {
        let case_list = vec![vec![1, 8, 6, 2, 5, 4, 8, 3, 7], vec![1, 1]];
        let ans_list = vec![49, 1];
        (case_list, ans_list)
    }

    #[test]
    fn max_area_test() {
        let (case_list, ans_list) = get_cases();
        for (case, ans) in case_list.into_iter().zip(ans_list.into_iter()) {
            assert_eq!(max_area(case), ans);
        }
    }
}
