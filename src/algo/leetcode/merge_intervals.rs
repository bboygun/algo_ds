// 56. 合并区间
// 以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。请你合并所有重叠的区间，并返回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。

// 示例 1：
// 输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
// 输出：[[1,6],[8,10],[15,18]]
// 解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].

// 示例 2：
// 输入：intervals = [[1,4],[4,5]]
// 输出：[[1,5]]
// 解释：区间 [1,4] 和 [4,5] 可被视为重叠区间。

// 提示：
// 1 <= intervals.length <= 10^4
// intervals[i].length == 2
// 0 <= starti <= endi <= 10^4

fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // 先按开始排序
    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[0]);

    let mut ans: Vec<Vec<i32>> = vec![];
    let mut cur_interval = intervals[0].clone();
    for interval in intervals {
        let (start, end) = (interval[0], interval[1]);
        // 若新区间在当前区间之后，则将当前区间加入答案后进行更新
        if start > cur_interval[1] {
            ans.push(cur_interval);
            cur_interval = interval;
            continue;
        }
        // 若新区间超出了当前区间，进行合并
        if end > cur_interval[1] {
            cur_interval = vec![cur_interval[0], end];
        }
    }
    ans.push(cur_interval);

    ans
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn merge_test() {
        let question1 = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let ans1 = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(merge(question1), ans1);

        let question2 = vec![vec![1, 4], vec![4, 5]];
        let ans2 = vec![vec![1, 5]];
        assert_eq!(merge(question2), ans2);

        let question3 = vec![vec![1, 2]];
        let ans3 = vec![vec![1, 2]];
        assert_eq!(merge(question3), ans3);
    }
}
