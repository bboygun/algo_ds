// 240. 搜索二维矩阵 II
// 编写一个高效的算法来搜索 m x n 矩阵 matrix 中的一个目标值 target 。该矩阵具有以下特性：
// 每行的元素从左到右升序排列。
// 每列的元素从上到下升序排列。

// 示例 1：
// 输入：matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 5
// 输出：true

// 示例 2：
// 输入：matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 20
// 输出：false

// 提示：
// m == matrix.length
// n == matrix[i].length
// 1 <= n, m <= 300
// -10^9 <= matrix[i][j] <= 10^9
// 每行的所有元素从左到右升序排列
// 每列的所有元素从上到下升序排列
// -10^9 <= target <= 10^9

use std::cmp::Ordering;

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (i, j) = (0, 0);
    // search(&matrix, i, j, target)
    linear_search(&matrix, i, j, target)
    // binary_search(&matrix, 0, 0, matrix.len() - 1, matrix[0].len() - 1, target)
}

fn search(matrix: &Vec<Vec<i32>>, i: usize, j: usize, target: i32) -> bool {
    if i == matrix.len() || j == matrix[0].len() || matrix[i][j] > target {
        return false;
    }
    matrix[i][j] == target || search(matrix, i + 1, j, target) || search(matrix, i, j + 1, target)
}

fn linear_search(matrix: &Vec<Vec<i32>>, i: usize, j: usize, target: i32) -> bool {
    if i == matrix.len() || j == matrix[0].len() || matrix[i][j] > target {
        return false;
    }
    let mut column = j;
    while column < matrix[0].len() {
        match matrix[i][column].cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => column += 1,
            Ordering::Greater => break,
        }
    }
    let mut row = i;
    while row < matrix.len() {
        match matrix[row][j].cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => row += 1,
            Ordering::Greater => break,
        }
    }
    return linear_search(matrix, i + 1, j + 1, target);
}

// 这是错的
#[deprecated(note = "this method is wrong")]
fn binary_search(
    matrix: &Vec<Vec<i32>>,
    start_i: usize,
    start_j: usize,
    end_i: usize,
    end_j: usize,
    target: i32,
) -> bool {
    println!(
        "start ({}, {}), end ({}, {})",
        start_i, start_j, end_i, end_j
    );
    if start_i > end_i || start_j > end_j {
        return false;
    }
    if start_i == end_i && start_j == end_j {
        return matrix[start_i][start_j] == target;
    }
    let mid_i = start_i + (end_i - start_i) / 2;
    let mid_j = start_j + (end_j - start_j) / 2;
    println!("mid ({}, {})", mid_i, mid_j);
    match matrix[mid_i][mid_j].cmp(&target) {
        Ordering::Equal => true,
        Ordering::Greater => {
            let mut find = false;
            if mid_i > 0 && mid_j > 0 {
                find |= binary_search(matrix, start_i, start_j, mid_i - 1, mid_j - 1, target);
            }
            if !find && mid_i > 0 {
                find |= binary_search(matrix, start_i, mid_j, mid_i - 1, end_j, target);
            }
            if !find && mid_j > 0 {
                find |= binary_search(matrix, mid_i, start_j, end_i, mid_j - 1, target);
            }
            find
        }
        Ordering::Less => {
            // 在 mid_i 行内查找
            let row = mid_i;
            let (mut left, mut right) = (mid_j, end_j);

            while left <= right {
                let mid = left + (right - left) / 2;
                match matrix[row][mid].cmp(&target) {
                    Ordering::Equal => return true,
                    Ordering::Less => left = mid + 1,
                    Ordering::Greater => right = mid - 1,
                }
            }

            // 在 mid_j 列内查找
            let column = mid_j;
            let (mut top, mut bottom) = (mid_i, end_i);
            while top <= bottom {
                let mid = top + (bottom - top) / 2;
                println!(
                    "find by column {}, top {}, bottom {}, mid {}, mid_value {}, target {}",
                    column, top, bottom, mid, matrix[mid][column], target
                );
                match matrix[mid][column].cmp(&target) {
                    Ordering::Equal => return true,
                    Ordering::Less => top = mid + 1,
                    Ordering::Greater => bottom = mid - 1,
                }
            }

            // 都没有，往右下查找
            binary_search(matrix, mid_i + 1, mid_j + 1, end_i, end_j, target)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn search_matrix_ii_test() {
        let case = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        let target1 = 5;
        let target2 = 20;
        assert!(search_matrix(case.clone(), target1));
        assert_eq!(search_matrix(case, target2), false);
        assert_eq!(search_matrix(vec![vec![1, 1]], 2), false);

        let case2 = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        let case2_target1 = 5;
        let case2_target2 = 21;
        assert_eq!(search_matrix(case2.clone(), case2_target1), true);
        assert_eq!(search_matrix(case2, case2_target2), true);
    }
}
