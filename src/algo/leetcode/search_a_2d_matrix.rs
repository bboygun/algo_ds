// 74. 搜索二维矩阵
// 给你一个满足下述两条属性的 m x n 整数矩阵：
// 每行中的整数从左到右按非严格递增顺序排列。
// 每行的第一个整数大于前一行的最后一个整数。
// 给你一个整数 target ，如果 target 在矩阵中，返回 true ；否则，返回 false 。

// 示例 1：
// 输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
// 输出：true

// 示例 2：
// 输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
// 输出：false

// 提示：
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 100
// -10^4 <= matrix[i][j], target <= 10^4

use std::cmp::Ordering;

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (m, n) = (matrix.len(), matrix[0].len());
    let (mut top, mut bottom) = (0, m);

    while top <= bottom && top < m {
        let mid = top + (bottom - top) / 2;
        match matrix[mid][0].cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => top = mid + 1,
            Ordering::Greater => {
                if mid > 0 {
                    bottom = mid - 1;
                } else {
                    break;
                }
            }
        }
    }

    if top == 0 {
        return false;
    }

    let row = top - 1;
    let (mut left, mut right) = (0, n);

    while left <= right && left < n {
        let mid = left + (right - left) / 2;
        match matrix[row][mid].cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => left = mid + 1,
            Ordering::Greater => {
                if mid > 0 {
                    right = mid - 1;
                } else {
                    break;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn search_matrix_test() {
        let matrix1 = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target1 = 3;
        assert_eq!(search_matrix(matrix1.clone(), target1), true);

        let target2 = 13;
        assert_eq!(search_matrix(matrix1, target2), false);
    }
}
