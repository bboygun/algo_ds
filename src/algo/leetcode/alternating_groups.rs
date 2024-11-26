// 3206. 交替组 I
// 给你一个整数数组 colors ，它表示一个由红色和蓝色瓷砖组成的环，第 i 块瓷砖的颜色为 colors[i] ：

// colors[i] == 0 表示第 i 块瓷砖的颜色是 红色 。
// colors[i] == 1 表示第 i 块瓷砖的颜色是 蓝色 。
// 环中连续 3 块瓷砖的颜色如果是 交替 颜色（也就是说中间瓷砖的颜色与它 左边 和 右边 的颜色都不同），那么它被称为一个 交替 组。

// 请你返回 交替 组的数目。

// 注意 ，由于 colors 表示一个 环 ，第一块 瓷砖和 最后一块 瓷砖是相邻的。

// 示例 1：
// 输入：colors = [1,1,1]
// 输出：0

// 示例 2：
// 输入：colors = [0,1,0,0,1]
// 输出：3

// 提示：
// 3 <= colors.length <= 100
// 0 <= colors[i] <= 1

fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
    let len = colors.len();
    let mut ans = 0;
    for (index, &color) in colors.iter().enumerate() {
        let left = get_left(&colors, index, len);
        let right = get_right(&colors, index, len);
        if left != color && color != right {
            ans += 1;
        }
    }
    ans
}

fn get_left(colors: &Vec<i32>, index: usize, len: usize) -> i32 {
    let left_index = if index == 0 { len - 1 } else { index - 1 };
    colors[left_index]
}

fn get_right(colors: &Vec<i32>, index: usize, len: usize) -> i32 {
    let right_index = if index == len - 1 { 0 } else { index + 1 };
    colors[right_index]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn alternating_groups_test() {
        let colors = vec![1, 1, 1];
        assert_eq!(number_of_alternating_groups(colors), 0);

        let colors = vec![0, 1, 0, 0, 1];
        assert_eq!(number_of_alternating_groups(colors), 3);
    }
}
