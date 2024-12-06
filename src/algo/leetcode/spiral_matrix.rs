// 54. 螺旋矩阵
// 给你一个 m 行 n 列的矩阵 matrix ，请按照 顺时针螺旋顺序 ，返回矩阵中的所有元素。

// 示例 1：
// 输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
// 输出：[1,2,3,6,9,8,7,4,5]

// 示例 2：
// 输入：matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
// 输出：[1,2,3,4,8,12,11,10,9,5,6,7]

// 提示：
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 10
// -100 <= matrix[i][j] <= 100

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let (mut top, mut bottom) = (0 as i32, (matrix.len() - 1) as i32);
    let (mut left, mut right) = (0 as i32, (matrix[0].len() - 1) as i32);

    let mut ans = vec![];
    loop {
        for column in left..=right {
            ans.push(matrix[top as usize][column as usize]);
        }
        top += 1;
        if top > bottom {
            break;
        }

        for row in top..=bottom {
            ans.push(matrix[row as usize][right as usize]);
        }
        right -= 1;
        if left > right {
            break;
        }

        for column in (left..=right).rev() {
            ans.push(matrix[bottom as usize][column as usize]);
        }
        bottom -= 1;
        if top > bottom {
            break;
        }

        for row in (top..=bottom).rev() {
            ans.push(matrix[row as usize][left as usize]);
        }
        left += 1;
        if left > right {
            break;
        }
    }
    ans
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn spiral_order_test() {
        let case1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let ans1 = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(spiral_order(case1), ans1);

        let case2 = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let ans2 = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(spiral_order(case2), ans2);

        let case3 = vec![vec![3], vec![2]];
        let ans3 = vec![3, 2];
        assert_eq!(spiral_order(case3), ans3);
    }
}
