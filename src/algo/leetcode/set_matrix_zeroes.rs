// 73. 矩阵置零
// 给定一个 m x n 的矩阵，如果一个元素为 0 ，则将其所在行和列的所有元素都设为 0 。请使用 原地 算法。

// 示例 1：
// 输入：matrix = [[1,1,1],[1,0,1],[1,1,1]]
// 输出：[[1,0,1],[0,0,0],[1,0,1]]

// 示例 2：
// 输入：matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
// 输出：[[0,0,0,0],[0,4,5,0],[0,3,1,0]]

// 提示：
// m == matrix.length
// n == matrix[0].length
// 1 <= m, n <= 200
// -2^31 <= matrix[i][j] <= 2^31 - 1

// 进阶：
// 一个直观的解决方案是使用  O(mn) 的额外空间，但这并不是一个好的解决方案。
// 一个简单的改进方案是使用 O(m + n) 的额外空间，但这仍然不是最好的解决方案。
// 你能想出一个仅使用常量空间的解决方案吗？

fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut rows = vec![];
    let mut columns = vec![];

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 0 {
                rows.push(i);
                columns.push(j);
            }
        }
    }

    for row in rows {
        for j in 0..matrix[0].len() {
            matrix[row][j] = 0;
        }
    }

    for column in columns {
        for i in 0..matrix.len() {
            matrix[i][column] = 0;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn set_zeros_test() {
        let mut case1 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let ans1 = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        set_zeroes(&mut case1);
        assert_eq!(case1, ans1);

        let mut case2 = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let ans2 = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        set_zeroes(&mut case2);
        assert_eq!(case2, ans2);
    }
}
