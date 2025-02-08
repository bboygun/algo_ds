// 79. 单词搜索
// 给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。如果 word 存在于网格中，返回 true ；否则，返回 false 。
// 单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。

// 示例 1：
// 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
// 输出：true

// 示例 2：
// 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
// 输出：true

// 示例 3：
// 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
// 输出：false

// 提示：
// m == board.length
// n = board[i].length
// 1 <= m, n <= 6
// 1 <= word.length <= 15
// board 和 word 仅由大小写英文字母组成

// 进阶：你可以使用搜索剪枝的技术来优化解决方案，使其在 board 更大的情况下可以更快解决问题？

fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let m = board.len();
    let n = board[0].len();
    let mut used: Vec<Vec<bool>> = vec![vec![false; n]; m];

    for i in 0..m {
        for j in 0..n {
            if backtrace(&board, &mut used, i, j, &word) {
                return true;
            }
        }
    }

    false
}

fn backtrace(
    board: &Vec<Vec<char>>,
    used: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
    word: &str,
) -> bool {
    if board[i][j] != word.chars().next().unwrap() {
        return false;
    }

    if word.chars().count() == 1 {
        return true;
    }

    let mut found = false;
    used[i][j] = true;
    if i > 0 && !used[i - 1][j] {
        found = found || backtrace(board, used, i - 1, j, &word[1..]);
    }
    if j > 0 && !used[i][j - 1] {
        found = found || backtrace(board, used, i, j - 1, &word[1..]);
    }
    if i < board.len() - 1 && !used[i + 1][j] {
        found = found || backtrace(board, used, i + 1, j, &word[1..]);
    }
    if j < board[0].len() - 1 && !used[i][j + 1] {
        found = found || backtrace(board, used, i, j + 1, &word[1..]);
    }
    used[i][j] = false;

    found
}

#[cfg(test)]
mod tests {

    use std::process::exit;

    use super::*;

    #[test]
    fn word_search_test() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word1 = "ABCCED";
        let word2 = "SEE";
        let word3 = "ABCB";

        let board2 = vec![vec!['a']];
        let word4 = "a";

        assert_eq!(exist(board.clone(), word1.into()), true);
        assert_eq!(exist(board.clone(), word2.into()), true);
        assert_eq!(exist(board.clone(), word3.into()), false);
        assert_eq!(exist(board2, word4.into()), true);
    }
}
