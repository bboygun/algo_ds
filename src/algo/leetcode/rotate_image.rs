// 48. 旋转图像
// 给定一个 n × n 的二维矩阵 matrix 表示一个图像。请你将图像顺时针旋转 90 度。
// 你必须在 原地 旋转图像，这意味着你需要直接修改输入的二维矩阵。请不要 使用另一个矩阵来旋转图像。

// 示例 1：
// 输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
// 输出：[[7,4,1],[8,5,2],[9,6,3]]

// 示例 2：
// 输入：matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
// 输出：[[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]

// 提示：
// n == matrix.length == matrix[i].length
// 1 <= n <= 20
// -1000 <= matrix[i][j] <= 1000

fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    // 从外向内旋转，n为偶数，需要旋转n/2层; n为奇数，最后一层不用旋转，也是n/2层（向下取整）
    for i in 0..(n / 2) {
        // 列从 i 到 n - i - 2 (因为 n - i - 1 在第一次已经被换掉了)
        for j in i..(n - i - 1) {
            // 以(i, j)为上坐标，求得其关联的右，下，左坐标如下
            let (top_i, top_j) = (i, j);
            let (right_i, right_j) = (j, n - 1 - i);
            let (bottom_i, bottom_j) = (n - 1 - i, n - 1 - j);
            let (left_i, left_j) = (n - 1 - j, i);

            // 顺时针旋转90度，即 上=左，左=下，下=右，右=上
            let tmp = matrix[top_i][top_j];
            matrix[top_i][top_j] = matrix[left_i][left_j];
            matrix[left_i][left_j] = matrix[bottom_i][bottom_j];
            matrix[bottom_i][bottom_j] = matrix[right_i][right_j];
            matrix[right_i][right_j] = tmp;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn rotate_test() {
        let mut case1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let ans1 = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        rotate(&mut case1);
        assert_eq!(case1, ans1);

        println!();
        let mut case2 = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let ans2 = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        rotate(&mut case2);
        assert_eq!(case2, ans2);
    }
}
